use wasm_bindgen::prelude::*;

/// #[] 내부의 속성(rust에서는 trait)을 명시해 이어지는 statement를 수정함.
/// 이 경우 extern statement는 아래의 외부적으로 정의된 함수를 호출하기 원한다고 말하는 것이고,
/// #[]내부의 속성인 wasm-bindgen은 이 함수를 찾는 방법을 알고 있다는 뜻임.
#[wasm_bindgen] // 아래 extern block 내부의 body에 있는 함수들을 알고 있는 attribute를 #[]에 추가
extern {
    pub fn alert(s: &str);
} // javascript의 함수를 불러오고 싶으면 위의 extern code block에 추가하면 됨

/// #[wasm_bindgen] 이후에 나오는 'fn' statement는 extern과는 반대이다. block을 수정하는 것이
/// 아니라 이 fn이 javascript에 의해 호출될 수 있기를 원한다는 뜻임. 즉 우리가 세상에 제공하는 기능이다.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

// 이후에 컴파일을 위해 Cargo.toml을 수정하고
// $ wasm-pack build --target web
// 명령을 실행하면 아래와 같은 순서로 진행된다.
// 1. rust code를 webassembly로 컴파일한다.
// 2. 해당 webassembly에서 wasm-bindgen을 실행해 해당 webassembly 파일을 브라우저가 이해할 수 있는
//    모듈로 래핑하는 javascript파일을 생성한다.
// 3. pkg 디렉토리를 생성하고, javascript 파일과 webassembly 코드를 pkg디렉토리로 이동시킨다.
// 4. Cargo.toml을 읽고 그에 상응하는 package.json 파일을 생성한다.
// 5. README.md 파일이 있다면 패키지로 복사한다.
// 6. 최종 결과는 pkg 디렉토리 안에 패키지가 있음.
// 코드 사이즈 최적화는?
// 위의 예제에선 rust에게 최적화를 지시하지 않았음. 지시한다면 사이즈를 줄일 수 있다.
// https://rustwasm.github.io/book/game-of-life/code-size.html#shrinking-wasm-size

// 패키지를 npm에서 사용할 수 있도록 하기
// npm과 함께 WebAssembly 모듈을 사용하려면 몇가지 사항을 변경해야한다.
//
// target bundler 옵션을 사용해 Rust를 recompile하기
// $ wasm-pack build --target bundler
//
// npm link를 사용해서 이 패키지를 설치된 다른 javascript 패키지에서 사용할 수 있도록 하기
// $ cd pkg
// $ npm link
//
// WebAssembly로 컴파일되었으며 Rust로 작성된 npm 패키지가 준비되었음. javascript에서 바로 사용할 수 있으며
// 유저가 Rust를 설치할 필요도 없음.
//
// web에서 npm 패키지 사용
// 루트로 이동 후 site 디렉토리 생성
// $ mkdir site
// $ cd site
// $ npm link hello-wasm
// 이러면 site 디렉토리에 node-modules가 생성됨
// 이제 npm을 사용할 준비가 되었음
// site 내에
// 1. package.json,
// 2. webpack.config.js,
// 3. index.js(rust code를 호출할 수 있다!!!),
// 4. index.html
// 를 생성해준다.

// 이후에 웹서버를 실행해보자
// $ npm install
// $ npm run serve
//
// 성공적!