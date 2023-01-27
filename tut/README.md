<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html


## wasm 프로젝트 생성하기
$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
(권장되는 방법)

명령어 입력 후 프로젝트 이름을 입력하면 생성됨

또는

$ cargo new --lib [project name]


## Cargo.toml 편집
1) [lib] 섹션 추가
   [lib] 섹션에 crate-type 필드를 추가함
   crate-type = ["cdylib"] // 이는 라이브러리가 C 호환 동적 라이브러리임을 지정함.
   wasm crate는 normal crate와는 약간의 차이가 있다.
   일반적인 crate는 rlib(기본값) 또는 바이너리일 경우 bin이 기본값이다.
   cdylib은 일반적인 라이브러리에서는 컴파일러가 동적 시스템 라이브러리를 생성하기를 원할 때 사용하지만,
   wasm의 경우 "create a *.wasm file without a start function"을 의미함.
   다른 플랫폼에서 이 출력 유형은 linux의 *.so, macOS의 *.dylib 및 windows의 *.dll 파일 생성

dynamic link lib?
동적 링크라고 하며 실행 파일에서 해당 라이브러리의 기능을 사용 시에만 라이브러리 파일을 참조하여(혹은 다운로드받아) 기능을 호출한다.
정적 링크와는 다르게 컴파일 시점에 실행 파일에 함수를 복사하지 않고, 함수의 위치정보만 갖고 그 함수를 호출할 수 있게 한다.

또한 우리 라이브러리가 wasm-pack test로 unit test될 수 있도록 하기 위해 crate-type = ["rlib"]을 지정함.
이게 없으면 cdylib crate 유형이 wasm-pack의 unit test stype과 호환되지 않기 때문에 라이브러리를 테스트할 수 없음.

2) wasm-bindgen 종속성 추가(JS와 wasm 모듈 간의 상호 운용성을 용이하게 함)
   이 패키지를 사용하면, #[wasm-bindgen] attribute를 이용해 JS와 Rust에 생성된 wasm 사이에
   원하는 interface를 나타내는 tag code에 tag를 지정할 수 있음.
   이 attribute를 사용해 JS를 가져오고 Rust를 내보낼 수 있음.

3) [features] and wee_alloc, console_error_panic_hook dependencies
   두 의존성은 wasm crate 개발 워크플로의 특정 부분에서 유용하기 때문에, 두 가지를 모두 의존성으로 포함할 수 있고,
   선택적으로 포함할 수 있는 약간의 glue code도 설정했음.

`console_error_panic_hook` 크레이트는 패닉을 `console.error`로 기록하여 더 나은 패닉 디버깅을 제공함.
이는 개발에 적합하지만 모든 `std::fmt` 및 `std::panicking` 인프라가 필요하므로 배포 시 코드 크기에 적합하지 않다(너무 많이 요구함).

'wee_alloc'은 default allocator(10K이하)에 비해 코드 크기가 1K이하에 불과한 작은 할당자임.
그러나 기본 할당자보다 느리다.. 안타깝게도 'wee_alloc'은 현재 wasm을 대상으로 컴파일 할때 nightly Rust가 필요하다.

코드에서 특정 [features], 특히 console_error_panic_hook 및 wee_alloc이 enabled된 경우에만
코드의 특정 부분이 실행되는 것으로 표시한다. 기본적으로 console_error_panic_hook만 활성화된다.
기본적으로 기능을 비활성화하거나 활성화하려면. [features] 내에서 default 벡터를 편집할 수 있음.


## crate가 Cargo에 구성되었으므로 이제 할일은 파일에 Rust 코드를 포함하는 것임(src/lib.rs)
(Rust에서 추가적인 mod 구조 선언 없이 알아서 컴파일해주는 source file인 lib.rs, main.rs중 lib.rs 사용)
여기에는 다음 3가지의 key parts가 포함되어 있다.
- '#[wasm_bindgen] func'
- Crate imports
- wee_alloc optional dependency

use wasm_bindgen::prelude::*; // wasm_bindgen도 prelude가 있음.
'#[wasm_bindgen]'을 '#[wasm_bindgen::prelude::wasm_bindgen]'으로 써도 되지만 권장되진 않음.

1) '#[wasm_bindgen]' func
'#[wasm_bindgen]' attribute는 그 아래의 함수가 JS와 Rust에서 모두 액세스 가능함을 나타냄.

'#[wasm_bindgen]'
extern {
  fn alert(s: &str);
}
extern block은 external JS func인 alert를 Rust로 가져옴. 이 선언은 alert를 Rust에서 호출하는데 필요함.
이런식으로 선언하면 wasm-bindgen은 Rust와 JS사이에서 문자열을 앞뒤로 전달할 수 있는 alert JS stubs를 생성함.
alert함수를 JS에서 가져왔으므로 Rust 함수를 작성해서 넣어보자

'#[wasm_bindgen]' // 이 attribute 없이 작성한다면 JS내에서 greet함수에 쉽게 액세스할 수 없으며 &str과 같은 특정 type을 변환할 수 없음

pub fn greet() {
alert("Hello, tut!");
}


## crate organization
mod utils; // utils.rs 자체만으로는 컴파일되지 않음. mod utils를 선언해줘서 새 모듈을 선언해줌.

if #[cfg(feature = "wee_alloc")] {	#[global_allocator]

'#[global_allocator]'

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
컴파일 때, 이 컴파일에 wee_alloc feature가 활성화되어 있는지 테스트함. 활성화되면 wee_alloc의 docs에 따라
global allocator를 구성하고 그렇지 않으면 아무것도 컴파일하지 않음.

앞서 본 것처럼 [features]의 default vector에는 "console_error_panic_hook"만 포함되어 있기 때문에(Cargo.toml)
이 블록은 코드로 대체되지 않고 wee_alloc 대신 default memory allocator가 사용된다.

1) set_panic_hook
   utils.rs 모듈 내부의 set_panic_hook은 console_error_panic_hook feature가 활성화되지 않은 경우, set_panic_hook은
   inlined된 empty 함수로 정의된다(그렇지만 활성화 됐음). 따라서 사용으로 인해 발생하는 run time 성능 또는
   code size 패널티가 없음.

앞서 설명했듯이 console_error_panic_hook은 panic을 우아하게 종료하기 위한 방법임.
여기서 cfg attribute는 앞에 오는 함수를 정의한다. #[cfg(feature = "console_error_panic_hook")] attribute는 Rust에게
console_error_panic_hook feature가 설정되어 있는지 compile time에 확인하도록 지시함.
설정되어 있으면 이 함수를 호출, 그렇지 않으면 호출하지 않음.

2) What is console_error_panic_hook?
   반복하지만 console_error_panic_hook은 panic을 우아하게 종료하기 위한 패닉 메시지 디버깅 기능임.
   예를 들어 이 기능을 사용하기 전과 후를 살펴보자.
   Before: "RuntimeError: Unreachable executed"

After: "panicked at 'index out of bounds: the len is 3 but the index is 4', libcore/slice/mod.rs:2046:10"

console_error_panic_hook은 완전 자동 기능이 아니므로, 코드를 실행하기 전에
utils::set_panic_hook()가 호출되었는지 확인해야 함(set_panic_hook를 여러번 실행해도 안전하다).


## wee_alloc
wee_alloc? wasm용으로 설계된 tiny allocator로서 압축 전 코드 사이즈가 1킬로바이트에 불과하다.
wasm code는 유선으로 user들에게 빈번하게 전송되므로 컴파일된 코드 size는 애플리케이션을 빠르게 load되고 응답하도록
보장하는데 중요한 경우가 많다.
https://fitzgeraldnick.com/2018/02/09/wee-alloc.html 다음 분석에 따르면, wasm 메모리 공간의 최소 절반 이상이 Rust의
default memory allocator에게 필요하다. 그러나 wasm code는 종종 몇 개의 대규모 초기 할당만 요청하기 때문에
sophisticated(정교한) allocator가 필요하지 않은 경우가 많다.

wee_alloc은 size를 위해 speed를 trade off 한다. tiny code-size footprint를 가지고 있지만, default global allocator와
속도면에서 경쟁력이 없다.
자세한 내용은 다음 링크 참조 https://github.com/rustwasm/wee_alloc, https://rustwasm.github.io/docs/book/reference/code-size.html

Enabling wee_alloc
lib.rs에는 cfg_if! macro 안에 wee_alloc에 대한 configuration이 있다.
이 코드 block은 wee_alloc을 global memery allocator로 초기화하기 위한 것이지만 compile time에 wee_alloc feature가
활성화된 경우에만 가능하다. 이 feature는 build하는 동안 extra options를 전달하여 활성화시킬 수 있다.

$ wasm-pack build -- --features wee_alloc

or alternatively you could turn it on by default in Cargo.toml
[features]
default = ["console_error_panic_hook", "wee_alloc"]
(우리의 Cargo.toml [features] 섹션의 default 필드에는 추가시키지 않았으므로 전자의 방법을 사용해보자)


## test/web.rs
web.rs? wasm-pack test command를 통해 headless web browser에서 실행되도록 의도된 Cargo integration test이다.
여기에는 다음 3가지 key parts가 있다.
- '#[wasm_bindgen_Test]' func
- Crate Configuration
- '#![cfg]' directives
1) '#[wasm_bindgen_test]' func
'#[wasm_bindgen_test]'는 wasm 및 headless web browser test에 액세스할 수 있는 테스트를 정의한다는 점을 제외하면,
normal Rust '#[test]' attribute와 비슷하다. https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html
(참고: 현재기준으로 '#[test]'는 wasm에서도 작동함. custom test framworks가 불안정한 것 뿐. 즉, '#[test]'와 거의 차이가 없음)

2) crate configuration
   use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
wasm_bindgen_test_configure! macro는 test가 default인 Node.js가 아닌 web borwser에서 실행되도록 의도되었음을 나타냄

3) '#![cfg]' directives
'#![cfg(target_arch = "wasm32")]' statement는 test가 wasm32 아키텍처 또는 wasm32-unknown-unknown만을 대상으로 한다는 것을 의미함.
이렇게 하면 이러한 test가 web browser에서만 실행되도록 하여 라이브러리가 다른 플랫폼용으로 개발되는 경우에도
cargo test가 프로젝트에서 작동할 수 있음!!


## Building your project
$ wasm-pack build

만약 Node.js를 필요로 하는 패키지(CommonJS 모듈, e.g. require)를 사용하는 경우 다음 방법으로 빌드한다.

$ wasm-pack build --target nodejs

실행 시 이 명령은 다음의 작업들을 수행함
code를 wasm으로 complie함(아직 하지 않은 경우)
wasm file, JS wrapper file around the wasm, README, package.json file이 들어있는 pkg 폴더를 생성한다.


## Testing your project(https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html)

$ wasm-pack test --firefox

$ wasm-pack test --headless --firefox


## Package Code for npm

$ wasm-pack login

npm에 로그인하자 id가 없으면 만들면 된다

로그인 했으면 패키징 해보자

$ wasm-pack build --scope MYSCOPE

MYSCOPE는 username으로 하면 된다.
실행시 build와 마찬가지로 다음의 작업을 수행함
- wasm으로 컴파일함(아직 하지 않은 경우)
- wasm file, JS wrapper file around the wasm, README, package.json file이 들어있는 pkg 폴더를 생성한다.
  이것들이 npm에 코드를 업로드하는 데 필요한 전부다.

로그인 했다면 pkg로 들어가 패키지를 업로드 해보자

$ cd pkg

$ npm publish --access=public


## Run the code from npm(https://webpack.js.org/ 참조)
새 패키지를 사용할 수 있는 프로젝트를 구축하기 위해 create-wasm-app이라는 npm 템플릿을 사용해보자
다음 명령어로 사용

$ npm init wasm-app my-new-wasm-app

npm 패키지 추가
scaffolded 프로젝트에는 package.json에 wasm 패키지인 hello-wasm-pack이 포함되어 있음.
default 값인 hello-wasm-pack이 종속성으로 되어있는데,
이것을 제거하고 나의 패키지를 종속성으로 추가해보자
(--scope에 인수로 주었던 var(이름)으로 "@이름/패키지명" 으로 종속성을 추가해준다.)

index.js 파일을 열고 "hello-wasm-pack"을 "@이름/패키지명"로 바꿔보자

종속성을 npm으로부터 install하기위해

$ npm install

완료되었다면

$ npm start




## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.