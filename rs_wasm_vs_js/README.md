# JS vs Rust-wasm 

Speed test with Fibonacci calculations.

(Pn = Pn-1 + Pn-2)

n = 40

### chrome
- `JS`: 848.6669921875 ms
- `Rust-wasm`: 483.325927734375 ms

### Ms Edge
- `JS`: 856.367919921875 ms
- `Rust-wasm`: 423.906005859375 ms

### Firefox
- `JS`: 1716 ms
- `Rust-wasm`: 289 ms

In any case, `Rust-wasm` is about 1.75x faster than `JS`

Edge and Chrome are similar enough that there is no difference, but in Firefox, it is about 1.5 times faster compared to the same wasm.
On the other hand, running JS in Firefox is about twice as slow compared to the previous both browsers.

It is better to run `JS` in `Edge` or `Chrome` and run `wasm` in `Firefox`.