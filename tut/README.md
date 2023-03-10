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

  <sub>Built with ๐ฆ๐ธ by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**๐ Read this template tutorial! ๐**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html


## wasm ํ๋ก์ ํธ ์์ฑํ๊ธฐ
$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
(๊ถ์ฅ๋๋ ๋ฐฉ๋ฒ)

๋ช๋ น์ด ์๋ ฅ ํ ํ๋ก์ ํธ ์ด๋ฆ์ ์๋ ฅํ๋ฉด ์์ฑ๋จ

๋๋

$ cargo new --lib [project name]


## Cargo.toml ํธ์ง
1) [lib] ์น์ ์ถ๊ฐ
   [lib] ์น์์ crate-type ํ๋๋ฅผ ์ถ๊ฐํจ

   crate-type = ["cdylib"] // ์ด๋ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๊ฐ C ํธํ ๋์  ๋ผ์ด๋ธ๋ฌ๋ฆฌ์์ ์ง์ ํจ.
   wasm crate๋ normal crate์๋ ์ฝ๊ฐ์ ์ฐจ์ด๊ฐ ์๋ค.
   ์ผ๋ฐ์ ์ธ crate๋ rlib(๊ธฐ๋ณธ๊ฐ) ๋๋ ๋ฐ์ด๋๋ฆฌ์ผ ๊ฒฝ์ฐ bin์ด ๊ธฐ๋ณธ๊ฐ์ด๋ค.
   cdylib์ ์ผ๋ฐ์ ์ธ ๋ผ์ด๋ธ๋ฌ๋ฆฌ์์๋ ์ปดํ์ผ๋ฌ๊ฐ ๋์  ์์คํ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๋ฅผ ์์ฑํ๊ธฐ๋ฅผ ์ํ  ๋ ์ฌ์ฉํ์ง๋ง,
   wasm์ ๊ฒฝ์ฐ "create a *.wasm file without a start function"์ ์๋ฏธํจ.
   ๋ค๋ฅธ ํ๋ซํผ์์ ์ด ์ถ๋ ฅ ์ ํ์ linux์ *.so, macOS์ *.dylib ๋ฐ windows์ *.dll ํ์ผ ์์ฑ


    dynamic link lib?
    ๋์  ๋งํฌ๋ผ๊ณ  ํ๋ฉฐ ์คํ ํ์ผ์์ ํด๋น ๋ผ์ด๋ธ๋ฌ๋ฆฌ์ ๊ธฐ๋ฅ์ ์ฌ์ฉ ์์๋ง ๋ผ์ด๋ธ๋ฌ๋ฆฌ ํ์ผ์ ์ฐธ์กฐํ์ฌ(ํน์ ๋ค์ด๋ก๋๋ฐ์) ๊ธฐ๋ฅ์ ํธ์ถํ๋ค.
    ์ ์  ๋งํฌ์๋ ๋ค๋ฅด๊ฒ ์ปดํ์ผ ์์ ์ ์คํ ํ์ผ์ ํจ์๋ฅผ ๋ณต์ฌํ์ง ์๊ณ , ํจ์์ ์์น์ ๋ณด๋ง ๊ฐ๊ณ  ๊ทธ ํจ์๋ฅผ ํธ์ถํ  ์ ์๊ฒ ํ๋ค.
    
    ๋ํ ์ฐ๋ฆฌ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๊ฐ wasm-pack test๋ก unit test๋  ์ ์๋๋ก ํ๊ธฐ ์ํด crate-type = ["rlib"]์ ์ง์ ํจ.
    ์ด๊ฒ ์์ผ๋ฉด cdylib crate ์ ํ์ด wasm-pack์ unit test stype๊ณผ ํธํ๋์ง ์๊ธฐ ๋๋ฌธ์ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๋ฅผ ํ์คํธํ  ์ ์์.

2) wasm-bindgen ์ข์์ฑ ์ถ๊ฐ(JS์ wasm ๋ชจ๋ ๊ฐ์ ์ํธ ์ด์ฉ์ฑ์ ์ฉ์ดํ๊ฒ ํจ)
   ์ด ํจํค์ง๋ฅผ ์ฌ์ฉํ๋ฉด, #[wasm-bindgen] attribute๋ฅผ ์ด์ฉํด JS์ Rust์ ์์ฑ๋ wasm ์ฌ์ด์
   ์ํ๋ interface๋ฅผ ๋ํ๋ด๋ tag code์ tag๋ฅผ ์ง์ ํ  ์ ์์.
   ์ด attribute๋ฅผ ์ฌ์ฉํด JS๋ฅผ ๊ฐ์ ธ์ค๊ณ  Rust๋ฅผ ๋ด๋ณด๋ผ ์ ์์.

3) [features] and wee_alloc, console_error_panic_hook dependencies
   ๋ ์์กด์ฑ์ wasm crate ๊ฐ๋ฐ ์ํฌํ๋ก์ ํน์  ๋ถ๋ถ์์ ์ ์ฉํ๊ธฐ ๋๋ฌธ์, ๋ ๊ฐ์ง๋ฅผ ๋ชจ๋ ์์กด์ฑ์ผ๋ก ํฌํจํ  ์ ์๊ณ ,
   ์ ํ์ ์ผ๋ก ํฌํจํ  ์ ์๋ ์ฝ๊ฐ์ glue code๋ ์ค์ ํ์.

`console_error_panic_hook` ํฌ๋ ์ดํธ๋ ํจ๋์ `console.error`๋ก ๊ธฐ๋กํ์ฌ ๋ ๋์ ํจ๋ ๋๋ฒ๊น์ ์ ๊ณตํจ.
์ด๋ ๊ฐ๋ฐ์ ์ ํฉํ์ง๋ง ๋ชจ๋  `std::fmt` ๋ฐ `std::panicking` ์ธํ๋ผ๊ฐ ํ์ํ๋ฏ๋ก ๋ฐฐํฌ ์ ์ฝ๋ ํฌ๊ธฐ์ ์ ํฉํ์ง ์๋ค(๋๋ฌด ๋ง์ด ์๊ตฌํจ).

'wee_alloc'์ default allocator(10K์ดํ)์ ๋นํด ์ฝ๋ ํฌ๊ธฐ๊ฐ 1K์ดํ์ ๋ถ๊ณผํ ์์ ํ ๋น์์.
๊ทธ๋ฌ๋ ๊ธฐ๋ณธ ํ ๋น์๋ณด๋ค ๋๋ฆฌ๋ค.. ์ํ๊น๊ฒ๋ 'wee_alloc'์ ํ์ฌ wasm์ ๋์์ผ๋ก ์ปดํ์ผ ํ ๋ nightly Rust๊ฐ ํ์ํ๋ค.

์ฝ๋์์ ํน์  [features], ํนํ console_error_panic_hook ๋ฐ wee_alloc์ด enabled๋ ๊ฒฝ์ฐ์๋ง
์ฝ๋์ ํน์  ๋ถ๋ถ์ด ์คํ๋๋ ๊ฒ์ผ๋ก ํ์ํ๋ค. ๊ธฐ๋ณธ์ ์ผ๋ก console_error_panic_hook๋ง ํ์ฑํ๋๋ค.
๊ธฐ๋ณธ์ ์ผ๋ก ๊ธฐ๋ฅ์ ๋นํ์ฑํํ๊ฑฐ๋ ํ์ฑํํ๋ ค๋ฉด. [features] ๋ด์์ default ๋ฒกํฐ๋ฅผ ํธ์งํ  ์ ์์.


## crate๊ฐ Cargo์ ๊ตฌ์ฑ๋์์ผ๋ฏ๋ก ์ด์  ํ ์ผ์ ํ์ผ์ Rust ์ฝ๋๋ฅผ ํฌํจํ๋ ๊ฒ์(src/lib.rs)
(Rust์์ ์ถ๊ฐ์ ์ธ mod ๊ตฌ์กฐ ์ ์ธ ์์ด ์์์ ์ปดํ์ผํด์ฃผ๋ source file์ธ lib.rs, main.rs์ค lib.rs ์ฌ์ฉ)
์ฌ๊ธฐ์๋ ๋ค์ 3๊ฐ์ง์ key parts๊ฐ ํฌํจ๋์ด ์๋ค.
- '#[wasm_bindgen] func'
- Crate imports
- wee_alloc optional dependency

use wasm_bindgen::prelude::*; // wasm_bindgen๋ prelude๊ฐ ์์.
'#[wasm_bindgen]'์ '#[wasm_bindgen::prelude::wasm_bindgen]'์ผ๋ก ์จ๋ ๋์ง๋ง ๊ถ์ฅ๋์ง ์์.

1) '#[wasm_bindgen]' func
'#[wasm_bindgen]' attribute๋ ๊ทธ ์๋์ ํจ์๊ฐ JS์ Rust์์ ๋ชจ๋ ์ก์ธ์ค ๊ฐ๋ฅํจ์ ๋ํ๋.

'#[wasm_bindgen]'
extern {
  fn alert(s: &str);
}
extern block์ external JS func์ธ alert๋ฅผ Rust๋ก ๊ฐ์ ธ์ด. ์ด ์ ์ธ์ alert๋ฅผ Rust์์ ํธ์ถํ๋๋ฐ ํ์ํจ.
์ด๋ฐ์์ผ๋ก ์ ์ธํ๋ฉด wasm-bindgen์ Rust์ JS์ฌ์ด์์ ๋ฌธ์์ด์ ์๋ค๋ก ์ ๋ฌํ  ์ ์๋ alert JS stubs๋ฅผ ์์ฑํจ.
alertํจ์๋ฅผ JS์์ ๊ฐ์ ธ์์ผ๋ฏ๋ก Rust ํจ์๋ฅผ ์์ฑํด์ ๋ฃ์ด๋ณด์

'#[wasm_bindgen]' // ์ด attribute ์์ด ์์ฑํ๋ค๋ฉด JS๋ด์์ greetํจ์์ ์ฝ๊ฒ ์ก์ธ์คํ  ์ ์์ผ๋ฉฐ &str๊ณผ ๊ฐ์ ํน์  type์ ๋ณํํ  ์ ์์

pub fn greet() {
alert("Hello, tut!");
}


## crate organization
mod utils; // utils.rs ์์ฒด๋ง์ผ๋ก๋ ์ปดํ์ผ๋์ง ์์. mod utils๋ฅผ ์ ์ธํด์ค์ ์ ๋ชจ๋์ ์ ์ธํด์ค.

if #[cfg(feature = "wee_alloc")] {	#[global_allocator]

'#[global_allocator]'

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
์ปดํ์ผ ๋, ์ด ์ปดํ์ผ์ wee_alloc feature๊ฐ ํ์ฑํ๋์ด ์๋์ง ํ์คํธํจ. ํ์ฑํ๋๋ฉด wee_alloc์ docs์ ๋ฐ๋ผ
global allocator๋ฅผ ๊ตฌ์ฑํ๊ณ  ๊ทธ๋ ์ง ์์ผ๋ฉด ์๋ฌด๊ฒ๋ ์ปดํ์ผํ์ง ์์.

์์ ๋ณธ ๊ฒ์ฒ๋ผ [features]์ default vector์๋ "console_error_panic_hook"๋ง ํฌํจ๋์ด ์๊ธฐ ๋๋ฌธ์(Cargo.toml)
์ด ๋ธ๋ก์ ์ฝ๋๋ก ๋์ฒด๋์ง ์๊ณ  wee_alloc ๋์  default memory allocator๊ฐ ์ฌ์ฉ๋๋ค.

1) set_panic_hook
   utils.rs ๋ชจ๋ ๋ด๋ถ์ set_panic_hook์ console_error_panic_hook feature๊ฐ ํ์ฑํ๋์ง ์์ ๊ฒฝ์ฐ, set_panic_hook์
   inlined๋ empty ํจ์๋ก ์ ์๋๋ค(๊ทธ๋ ์ง๋ง ํ์ฑํ ๋์). ๋ฐ๋ผ์ ์ฌ์ฉ์ผ๋ก ์ธํด ๋ฐ์ํ๋ run time ์ฑ๋ฅ ๋๋
   code size ํจ๋ํฐ๊ฐ ์์.

์์ ์ค๋ชํ๋ฏ์ด console_error_panic_hook์ panic์ ์ฐ์ํ๊ฒ ์ข๋ฃํ๊ธฐ ์ํ ๋ฐฉ๋ฒ์.
์ฌ๊ธฐ์ cfg attribute๋ ์์ ์ค๋ ํจ์๋ฅผ ์ ์ํ๋ค. #[cfg(feature = "console_error_panic_hook")] attribute๋ Rust์๊ฒ
console_error_panic_hook feature๊ฐ ์ค์ ๋์ด ์๋์ง compile time์ ํ์ธํ๋๋ก ์ง์ํจ.
์ค์ ๋์ด ์์ผ๋ฉด ์ด ํจ์๋ฅผ ํธ์ถ, ๊ทธ๋ ์ง ์์ผ๋ฉด ํธ์ถํ์ง ์์.

2) What is console_error_panic_hook?
   ๋ฐ๋ณตํ์ง๋ง console_error_panic_hook์ panic์ ์ฐ์ํ๊ฒ ์ข๋ฃํ๊ธฐ ์ํ ํจ๋ ๋ฉ์์ง ๋๋ฒ๊น ๊ธฐ๋ฅ์.
   ์๋ฅผ ๋ค์ด ์ด ๊ธฐ๋ฅ์ ์ฌ์ฉํ๊ธฐ ์ ๊ณผ ํ๋ฅผ ์ดํด๋ณด์.
   Before: "RuntimeError: Unreachable executed"

After: "panicked at 'index out of bounds: the len is 3 but the index is 4', libcore/slice/mod.rs:2046:10"

console_error_panic_hook์ ์์  ์๋ ๊ธฐ๋ฅ์ด ์๋๋ฏ๋ก, ์ฝ๋๋ฅผ ์คํํ๊ธฐ ์ ์
utils::set_panic_hook()๊ฐ ํธ์ถ๋์๋์ง ํ์ธํด์ผ ํจ(set_panic_hook๋ฅผ ์ฌ๋ฌ๋ฒ ์คํํด๋ ์์ ํ๋ค).


## wee_alloc
wee_alloc? wasm์ฉ์ผ๋ก ์ค๊ณ๋ tiny allocator๋ก์ ์์ถ ์  ์ฝ๋ ์ฌ์ด์ฆ๊ฐ 1ํฌ๋ก๋ฐ์ดํธ์ ๋ถ๊ณผํ๋ค.
wasm code๋ ์ ์ ์ผ๋ก user๋ค์๊ฒ ๋น๋ฒํ๊ฒ ์ ์ก๋๋ฏ๋ก ์ปดํ์ผ๋ ์ฝ๋ size๋ ์ ํ๋ฆฌ์ผ์ด์์ ๋น ๋ฅด๊ฒ load๋๊ณ  ์๋ตํ๋๋ก
๋ณด์ฅํ๋๋ฐ ์ค์ํ ๊ฒฝ์ฐ๊ฐ ๋ง๋ค.
https://fitzgeraldnick.com/2018/02/09/wee-alloc.html ๋ค์ ๋ถ์์ ๋ฐ๋ฅด๋ฉด, wasm ๋ฉ๋ชจ๋ฆฌ ๊ณต๊ฐ์ ์ต์ ์ ๋ฐ ์ด์์ด Rust์
default memory allocator์๊ฒ ํ์ํ๋ค. ๊ทธ๋ฌ๋ wasm code๋ ์ข์ข ๋ช ๊ฐ์ ๋๊ท๋ชจ ์ด๊ธฐ ํ ๋น๋ง ์์ฒญํ๊ธฐ ๋๋ฌธ์
sophisticated(์ ๊ตํ) allocator๊ฐ ํ์ํ์ง ์์ ๊ฒฝ์ฐ๊ฐ ๋ง๋ค.

wee_alloc์ size๋ฅผ ์ํด speed๋ฅผ trade off ํ๋ค. tiny code-size footprint๋ฅผ ๊ฐ์ง๊ณ  ์์ง๋ง, default global allocator์
์๋๋ฉด์์ ๊ฒฝ์๋ ฅ์ด ์๋ค.
์์ธํ ๋ด์ฉ์ ๋ค์ ๋งํฌ ์ฐธ์กฐ https://github.com/rustwasm/wee_alloc, https://rustwasm.github.io/docs/book/reference/code-size.html

Enabling wee_alloc
lib.rs์๋ cfg_if! macro ์์ wee_alloc์ ๋ํ configuration์ด ์๋ค.
์ด ์ฝ๋ block์ wee_alloc์ global memery allocator๋ก ์ด๊ธฐํํ๊ธฐ ์ํ ๊ฒ์ด์ง๋ง compile time์ wee_alloc feature๊ฐ
ํ์ฑํ๋ ๊ฒฝ์ฐ์๋ง ๊ฐ๋ฅํ๋ค. ์ด feature๋ buildํ๋ ๋์ extra options๋ฅผ ์ ๋ฌํ์ฌ ํ์ฑํ์ํฌ ์ ์๋ค.

$ wasm-pack build -- --features wee_alloc

or alternatively you could turn it on by default in Cargo.toml
[features]
default = ["console_error_panic_hook", "wee_alloc"]
(์ฐ๋ฆฌ์ Cargo.toml [features] ์น์์ default ํ๋์๋ ์ถ๊ฐ์ํค์ง ์์์ผ๋ฏ๋ก ์ ์์ ๋ฐฉ๋ฒ์ ์ฌ์ฉํด๋ณด์)


## test/web.rs
web.rs? wasm-pack test command๋ฅผ ํตํด headless web browser์์ ์คํ๋๋๋ก ์๋๋ Cargo integration test์ด๋ค.
์ฌ๊ธฐ์๋ ๋ค์ 3๊ฐ์ง key parts๊ฐ ์๋ค.
- '#[wasm_bindgen_Test]' func
- Crate Configuration
- '#![cfg]' directives
1) '#[wasm_bindgen_test]' func
'#[wasm_bindgen_test]'๋ wasm ๋ฐ headless web browser test์ ์ก์ธ์คํ  ์ ์๋ ํ์คํธ๋ฅผ ์ ์ํ๋ค๋ ์ ์ ์ ์ธํ๋ฉด,
normal Rust '#[test]' attribute์ ๋น์ทํ๋ค. https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html
(์ฐธ๊ณ : ํ์ฌ๊ธฐ์ค์ผ๋ก '#[test]'๋ wasm์์๋ ์๋ํจ. custom test framworks๊ฐ ๋ถ์์ ํ ๊ฒ ๋ฟ. ์ฆ, '#[test]'์ ๊ฑฐ์ ์ฐจ์ด๊ฐ ์์)

2) crate configuration
   use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
wasm_bindgen_test_configure! macro๋ test๊ฐ default์ธ Node.js๊ฐ ์๋ web borwser์์ ์คํ๋๋๋ก ์๋๋์์์ ๋ํ๋

3) '#![cfg]' directives
'#![cfg(target_arch = "wasm32")]' statement๋ test๊ฐ wasm32 ์ํคํ์ฒ ๋๋ wasm32-unknown-unknown๋ง์ ๋์์ผ๋ก ํ๋ค๋ ๊ฒ์ ์๋ฏธํจ.
์ด๋ ๊ฒ ํ๋ฉด ์ด๋ฌํ test๊ฐ web browser์์๋ง ์คํ๋๋๋ก ํ์ฌ ๋ผ์ด๋ธ๋ฌ๋ฆฌ๊ฐ ๋ค๋ฅธ ํ๋ซํผ์ฉ์ผ๋ก ๊ฐ๋ฐ๋๋ ๊ฒฝ์ฐ์๋
cargo test๊ฐ ํ๋ก์ ํธ์์ ์๋ํ  ์ ์์!!


## Building your project
$ wasm-pack build

๋ง์ฝ Node.js๋ฅผ ํ์๋ก ํ๋ ํจํค์ง(CommonJS ๋ชจ๋, e.g. require)๋ฅผ ์ฌ์ฉํ๋ ๊ฒฝ์ฐ ๋ค์ ๋ฐฉ๋ฒ์ผ๋ก ๋น๋ํ๋ค.

$ wasm-pack build --target nodejs

์คํ ์ ์ด ๋ช๋ น์ ๋ค์์ ์์๋ค์ ์ํํจ
code๋ฅผ wasm์ผ๋ก complieํจ(์์ง ํ์ง ์์ ๊ฒฝ์ฐ)
wasm file, JS wrapper file around the wasm, README, package.json file์ด ๋ค์ด์๋ pkg ํด๋๋ฅผ ์์ฑํ๋ค.


## Testing your project(https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html)

$ wasm-pack test --firefox

$ wasm-pack test --headless --firefox


## Package Code for npm

$ wasm-pack login

npm์ ๋ก๊ทธ์ธํ์ id๊ฐ ์์ผ๋ฉด ๋ง๋ค๋ฉด ๋๋ค

๋ก๊ทธ์ธ ํ์ผ๋ฉด ํจํค์ง ํด๋ณด์

$ wasm-pack build --scope MYSCOPE

MYSCOPE๋ username์ผ๋ก ํ๋ฉด ๋๋ค.
์คํ์ build์ ๋ง์ฐฌ๊ฐ์ง๋ก ๋ค์์ ์์์ ์ํํจ
- wasm์ผ๋ก ์ปดํ์ผํจ(์์ง ํ์ง ์์ ๊ฒฝ์ฐ)
- wasm file, JS wrapper file around the wasm, README, package.json file์ด ๋ค์ด์๋ pkg ํด๋๋ฅผ ์์ฑํ๋ค.
  ์ด๊ฒ๋ค์ด npm์ ์ฝ๋๋ฅผ ์๋ก๋ํ๋ ๋ฐ ํ์ํ ์ ๋ถ๋ค.

๋ก๊ทธ์ธ ํ๋ค๋ฉด pkg๋ก ๋ค์ด๊ฐ ํจํค์ง๋ฅผ ์๋ก๋ ํด๋ณด์

$ cd pkg

$ npm publish --access=public


## Run the code from npm(https://webpack.js.org/ ์ฐธ์กฐ)
์ ํจํค์ง๋ฅผ ์ฌ์ฉํ  ์ ์๋ ํ๋ก์ ํธ๋ฅผ ๊ตฌ์ถํ๊ธฐ ์ํด create-wasm-app์ด๋ผ๋ npm ํํ๋ฆฟ์ ์ฌ์ฉํด๋ณด์
๋ค์ ๋ช๋ น์ด๋ก ์ฌ์ฉ

$ npm init wasm-app my-new-wasm-app

npm ํจํค์ง ์ถ๊ฐ
scaffolded ํ๋ก์ ํธ์๋ package.json์ wasm ํจํค์ง์ธ hello-wasm-pack์ด ํฌํจ๋์ด ์์.
default ๊ฐ์ธ hello-wasm-pack์ด ์ข์์ฑ์ผ๋ก ๋์ด์๋๋ฐ,
์ด๊ฒ์ ์ ๊ฑฐํ๊ณ  ๋์ ํจํค์ง๋ฅผ ์ข์์ฑ์ผ๋ก ์ถ๊ฐํด๋ณด์
(--scope์ ์ธ์๋ก ์ฃผ์๋ var(์ด๋ฆ)์ผ๋ก "@์ด๋ฆ/ํจํค์ง๋ช" ์ผ๋ก ์ข์์ฑ์ ์ถ๊ฐํด์ค๋ค.)

index.js ํ์ผ์ ์ด๊ณ  "hello-wasm-pack"์ "@์ด๋ฆ/ํจํค์ง๋ช"๋ก ๋ฐ๊ฟ๋ณด์

์ข์์ฑ์ npm์ผ๋ก๋ถํฐ installํ๊ธฐ์ํด

$ npm install

์๋ฃ๋์๋ค๋ฉด

$ npm start




## ๐ด Usage

### ๐ Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### ๐ ๏ธ Build with `wasm-pack build`

```
wasm-pack build
```

### ๐ฌ Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### ๐ Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## ๐ Batteries Included

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