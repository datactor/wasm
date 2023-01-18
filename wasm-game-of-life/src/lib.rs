mod timer;
mod utils;

use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    back_cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let cells = &self.cells;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += cells[nw] as u8;

        let n = self.get_index(north, column);
        count += cells[n] as u8;

        let ne = self.get_index(north, east);
        count += cells[ne] as u8;

        let w = self.get_index(row, west);
        count += cells[w] as u8;

        let e = self.get_index(row, east);
        count += cells[e] as u8;

        let sw = self.get_index(south, west);
        count += cells[sw] as u8;

        let s = self.get_index(south, column);
        count += cells[s] as u8;

        let se = self.get_index(south, east);
        count += cells[se] as u8;

        count
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                self.back_cells.set(idx, next_cell);
            }
        }

        std::mem::swap(&mut self.cells, &mut self.back_cells);
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }
        let back_cells = cells.clone();

        Universe {
            width,
            height,
            cells: cells,
            back_cells: back_cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells.clear();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells.clear();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]); // toggle the value of the cell
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cells = &self.cells;
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let symbol = if cells[i] { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}





// 1. 다음 명령어를 통해 wasm-pack build
// $ wasm-pack build
// 명령을 실행하면 아래와 같은 순서로 진행된다.
// 1) rust code를 webassembly로 컴파일한다.
// 2) 해당 webassembly에서 wasm-bindgen을 실행해 해당 webassembly 파일을 브라우저가 이해할 수 있는
//    모듈로 래핑하는 javascript파일을 생성한다.
// 3) pkg 디렉토리를 생성하고, javascript 파일과 webassembly 코드를 pkg디렉토리로 이동시킨다.
// 4) Cargo.toml을 읽고 그에 상응하는 package.json 파일을 생성한다.
// 5) README.md 파일이 있다면 패키지로 복사된다.
// 6) 최종 결과는 pkg 디렉토리 안에 패키지가 있음.

// 생성된 wasm_game_of_life_bg.wasm 파일은 Rust source에서 Rust 컴파일러에 의해 생성된 WebAssembly 바이너리이다.
// 여기에 모든 Rust fn을 비롯한 기능을 담고 있음. 예를 들어 내보낸 greet 기능이 있다.

// wasm-bindgen에 의해 생성된 wasm_game_of_life.js 파일은 DOM 및 JavaScripts 함수를 Rust로 가져오고
// API를 WebAssembly함수에 Javascript로 노출시키기 위한 Javascript glue가 포함되어 있음.

// wasm_game_of_life.d.ts파일에는 JavaScript glue에 대한 TypeScript type 선언이 포함되어 있음.

// package.json 파일에는 생성된 JavaScript 및 WebAssembly 패키지에 대한 메타데이터가 포함되어 있음.
// 여기에는 npm 및 JavaScript bundler에서 패키지, 패키지이름, 버전 등 여러 항목 간의 종속성을
// 결정하는데 사용됨.


// 2. 웹 페이지에 넣기
// 패키지를 wasm-game-of-life 웹페이지에서 사용하기 위해 create-wasm-app JavaScript 프로젝트 템플릿을 사용하기 위한 명령어
// $ npm init wasm-app www
// wasm-game-of-life/www 디렉토리가 다음 구조대로 생성됨
// wasm-game-of-life/www/
// ├── bootstrap.js
// ├── index.html
// ├── index.js
// ├── LICENSE-APACHE
// ├── LICENSE-MIT
// ├── package.json
// ├── README.md
// └── webpack.config.js

// 새로 생성된 package.json은 npm에 게시된 초기 wasm-pack-template 패키지의 버전인
// hello-wasm-pack에 대한 종속성뿐만 아니라 webpack 및 webpack-dev-server 종속성으로
// 사전 구성되어 제공됨

// webpack.config.js는 웹팩과 해당 로컬 개발 서버를 구성함. 미리 구성된 상태로 제공되며
// 이를 조정할 필요가 없음

// index.html은 웹페이지의 루트 HTML. index.js를 둘러싼 얇은 래퍼인 bootstrap.js을 load하는 것
// 외에 많은 작업을 수행하지는 않음.

// index.js는 웹 페이지의 JavaScript에 대한 main entry point이다. 기본 wasm-pack-template의
// 컴파일된 WebAssembly 및 JavaScript glue가 포함된 hello-wasm-pack npm 패키지를 가져온 다음
// Hello-wasm-pack의 welcome 함수를 호출한다.
// 여기서 Rust의 코드를 호출할 수 있음!!


// 3. Install the dependencies
// 하위 디렉토리 wasm-game-of-life/www로 들어가 다음 명령어를 실행하여 로컬 서버와 해당 종속성이 설치되었는지 확인.
// (node_modules 생성)
// Note_ Rust 및 WebAssembly 작업에는 webpack이 필요하지 않다. 원하는 경우 번들러 없이 Rust 및
// WebAssembly를 사용할 수 있다.
// $ npm install


// 4. Using our Local wasm-game-of-life Package in www
// npm패키지의 hello-wasm-pack를 사용하는 대신 로컬 wasm-game-of-life 패키지를 사용해보자
// (대신 이 방식으로 빌드하면 코드 수정할 때마다 매번 빌드를 새로 해주어야 한다)
// www/package.json을 열고 "devDependencies"위에 다음 필드를 아래와 같이 추가해주자
// "dependencies": {                     // Add this three lines block!
//     "wasm-game-of-life": "file:../pkg"
//   },
//
// www/index.js파일의 import 부분을 hello-wasm-pack 대신 wasm-game-of-life로 수정해주자.
//
// 새로운 dependencis를 선언했으니 설치해주자.
// $ npm install


// 5. Serving Locally
// $ npm run start


// 6. 로컬 패키지를 사용하는 경우, 코드를 수정한다면 재빌드가 필요하다. 로컬의 러스트 lib.rs와는 실시간 자동
//    연동이 되지 않음.
// (코드 수정 후)
// $ wasm-pack build
// $ npm run start
//
// npm 패키지를 사용하면 패키지 관리자가 소스 코드의 변경 사항을 자동으로 감시하고 필요에 따라 프로젝트를 다시 빌드한다.
// npm run start는 webpack 개발 서버가 코드의 변경사항을 감시하고 감지되면 페이지를 자동으로 다시 빌드하고
// 다시 로드하도록 구성됨. 그러나 로컬 패키지의 경우에는 npm run start를 다시 하는 것으로도 변경사항이 반영되지 않음.
// 로컬 패키지를 사용하는 경우 개발 서버는 패키지를 인식하지 못하며 페이지를 자동으로 빌드 & 로드할 수 없음.
// 이것을 해결하려면 wsam-pack watch같은 도구를 사용하면 로컬 패키지의 변경 사항을 감시하고 감지되면
// 자동으로 프로젝트를 다시 빌드할 수 있다. 또는 watch-rust와 같은 개발 웹팩 플러그인을 사용하여
// Rust 코드의 변경 사항을 감지하고 웹팩 빌드 프로세스를 트리거할 수 있으며,
// cargo watch 또한 사용할 수 있음. cargo watch는 source code를 감시하고 변경이 감지되면
// 자동으로 명령을 실행하는 명령줄 utility이다.


// 7. Conway의 Game-of-life
// 1) 설계
//  우주 디자인 - 고정된 크기의 주기적인 우주
//
// 2) rust와 javascript의 인터페이스
//  Objects, Arrays 및 DOM nodes가 할당되는 JavaScript의 GC 힙은 Rust 값이 있는 WebAssembly의
//  linear memory space와 다르다. WebAssembly는 현재 GC 힙에 직접 액세스할 수 없다(2018년 4월
//  "Interface Types" proposal"로 변경될 것으로 예상됨 -> 21년 9월 기준으로도 불가능함:
//  wasm는 JS와 별도의 메모리 공간에서 작동하도록 설계되었으며, 둘은 서로의 메모리에 직접 액세스할 수 없다.
//  즉, JS와 wasm 간에 데이터를 전달할 때 두 메모리 공간 간에 데이터를 복사해야하므로 상대적으로 느리고 비효율적일 수 있음!
//  이러한 한계를 극복하기 위해 wasm 커뮤니티는 JS와 wasm간의 효율적이고 직접적인 통신을 허용하기 위한 몇가지 제안을 진행하고 있음.
//  현재 JS와 wasm간에 데이터를 공유하는 가장 좋은 방법은 JS의 typed array를 사용해 byte 형태로 데이터를 전달하는 것과 같이
//  브라우저에서 제공하는 JS API와 wasm 런타임에서 제공하는 wasm API를 사용하는 것임.
//  또는 wasm과 JS간에 메모리 buffer를 공유할 수 있는 wasm memory API를 사용하는 것이다.
//  ).
//
//  반면 JavaScript는 WebAssembly 선형 메모리 공간을 읽고 쓸 수 있지만 스칼라 값(u8, i32, f64 등)의
//  ArrayBuffer로만 가능하다. WebAssembly 함수는 또한 스칼라 값을 취하고 반환할 것이다. 이들은 모든 WebAssembly
//  및 JavaScript 통신이 구성되는 building block이다.
//
//  wasm_bindgen은 이 경계를 넘어 복합 구조로 작업하는 방법에 대한 일반적인 이해를 정의한다(여기에는 Rust 구조를 boxing하고
//  유용성을 위해 포인터를 JavaScript 클래스로 래핑하거나 Rust에서 JavaScript 개체 테이블로 인덱싱하는 작업이 포함됨).
//  wasm_bindgen은 매우 편리하지만 데이터 표현과 이 경계를 넘어 전달되는 값과 구조를 고려할 필요가 없음.
//  대신 선택한 인터페이스 디자인을 구현하기 위한 도구로 생각하면 된다.
//
//  WebAssembly와 JavaScript 간의 인터페이스를 설계할 때 다음 속성에 대해 최적화 해보자:
//   1) 위에 언급했던 것처럼 wasm의 linear memory에 대한 복사를 최소화 해보자.
//   2) serializing & deserializing 역시 최소화 해보자.
//      wasm_bindgen은 JS의 objects와 Rust의 boxed structures의 애매한 구조체들을 명백히 하는데 도움을 줄 것이다.
//  일반적으로 좋은 JS <-> wasm 인터페이스 디자인은 크고 라이프타임이 긴 데이터 structures를 Rust type으로 구현되고
//  JS의 opaque handle로 노출되는 경우가 많다. JS는 이러한 opaque handle을 사용하고, 데이터를 변환하고, 많은 계산을
//  수행하고, 데이터를 쿼리하고, 궁극적으로 복사 가능한 작은 결과를 반환하는 내보낸 wasm 함수를 호출한다.
//  계산의 작은 결과만 반환함으로써 JS GC heap과 wasm linear memory 사이에서 앞뒤로 모든 것을 복사 또는 serialization을
//  방지한다.
//
//
// game-of-life에서 Rust와 JS의 interface
// 피해야할 몇 가지 위험을 나열해보자. 모든 틱에서 wasm 선형 메모리 안팎으로 전체 유니버스를 복사되는 것, 유니버스의
// 모든 셀에 object를 할당하는 것, 각 셀을 읽고 쓰기 위해 경계를 넘는 호출을 부과하는 것.
// 메모리의 4 * 4 2차원 배열을 1차원 배열로 만들어 보자.
// 다음 수식으로 유니버스의 지정된 행과 열에서 셀의 배열 인덱스를 찾을 수 있다.
// index(row, column, universe) = row * width(universe) + column
//
// 유니버스의 셀을 JavaScript에 노출하는 방법?
// 먼저 std::fmt::Disply로 유니버스를 구현하여 텍스트 문자로 렌더링된 셀을 Rust 문자열로 생성:
// 그런 다음 이 Rust 문자열은 wasm linear memory에서 JS의 GC 힙에 있는 JS 문자열로 복사된 다음 HTML을 설정해 표시.
// 그런 다음 힙 간에 유니버스의 셀 복사를 피하고 <canvas>로 렌더링 하도록 이 구현을 발전시켜보자.
// 또 다른 대안?
// Rust가 전체 유니버스를 JS에 노출하는 대신 각 틱 후에 상태가 변경된 모든 셀의 목록을 반환하기.
// 이렇게 하면 JS는 렌더링할 때 전체 유니버스를 반복할 필요 없이 관련 하위 집합만 반복함.
// (The trade off is that this delta-based design is slightly more difficult to implement.)

// 8. 이제 위의 코드를 수정해 구현해보자.
// pre의 렌더링 구현이 완료

// 9. 메모리에서 직접 canvas로 렌더링(위에서 언급한 메모리 문제를 해결해보자.)
// Rust에서 문자열을 생성(및 할당)한 다음 wasm-bindgen이 유효한 JS 문자열로 변환하도록 하면?
// 유니버스 셀의 불필요한 복사본이 만들어진다.
// JS 문자열을 사용해 유니버스의 셀을 나타내지만, JS 문자열은 일단 생성되면 변경할 수 없다. 따라서 프로그램은
// 유니버스의 셀에 대한 Rust의 문자열 표현을 JS 문자열로 변환해야한다. 즉, 생성은 러스트에서 하고 JS 문자열로 변환해야 한다.
// 그렇지만 그렇게 하려면 새 JS 문자열을 생성한 다음 Rust 문자열 내용을 여기에 복사해야 한다. 이렇게 하면 유니버스 셀의
// 불필요한 새 복사본이 생성되어 추가 메모리를 차지하고 프로그램 성능이 느려질 수 있다.
// 즉, JS에 문자열 생성(불필요) 후, Rust에서 문자열 생성 후, 다시 javascript에 복사해서 넣는다.
// 여기에는 초기에 JS 문자열을 생성하는 불필요한 과정이 포함되어 있으므로 메모리 낭비가 된다.
//
// 새 JS 문자열을 생성하거나 Rust 문자열의 복사본을 생성하지 않고 Rust 문자열을 javascript에 직접 사용하는 방법?
//  1. Rust에서 문자열을 만들고 변수에 저장함
//  2. wasm-bindgen 라이브러리를 사용해 Rust 변수를 JS 변수로 직접 내보냄
//     (it is automatically wrapped in a JavaScript object that provides the necessary functionality to access the variable from JavaScript)!
//  3. 내보낸 변수에 JS에서 직접 액세스.
// 이렇게 Rust변수를 직접 내보내면 JS는 데이터의 새 복사본을 만들지 않고도 Rust 변수의 메모리에 액세스 할 수 있음.
// 이렇게 하면 새 JS 문자열을 만들고 Rust 문자열의 내용을 여기에 복사할 필요가 없어진다!!
// 이 방법은 효율적일 뿐만 아니라 안전하다. wasm-bindgen 라이브러리는 Rust 변수가 적절하게 관리되고 더 이상 필요하지
// 않을 때 정리되도록 하여 메모리 leak을 방지한다.
//
// 유의점:
// wasm-bindgen을 사용하여 변수를 JS로 내보냈을 때, 변수는 JS에서 변수에 엑세스하는 데 필요한 기능을 제공하는 JS object로 wrapping된다.
// 즉, Rust가 아닌 JS의 규칙을 따라 움직인다. 만약 내보낸 이후에 Rust에서 lifetime이 다해 삭제된다면
// JS object는 댕글링 포인터가 되어버린다!!(Rust에서 먼저 삭제된다면 JS object는 삭제된 Rust var의 주소를 가리킴).
// JS의 GC와 Rust는 독립적으로 움직인다. 즉 반대로 JS에서 변수를 삭제한다고 해도 Rust에서 삭제가 되는 것이 아닌 참조
// 해제할 뿐이다.
// 역시 이런 경우에서는 RC나 Arc를 사용해야함. -> wasm-bindgen에서 제공하는 JsValue type을 사용한다면
// Arc를 사용하지 않아도 된다. 여러 스레드에서 안전하고, JS에서 더 이상 필요하지 않을 때
// Rust 변수가 적절하게 삭제되도록 보장해준다.

// JavaScript 코드는 이미 유니버스의 너비와 높이를 알고 있고,
// 셀을 구성하는 wasm의 linear memory를 직접 읽을 수 있으므로 cells array의 시작에 대한 포인터를 반환하도록
// render 메서드를 수정.
// 또한 유니코드 텍스트를 렌더링하는 대신 Canvas API를 사용하도록 전환해보자.