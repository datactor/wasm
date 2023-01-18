import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
    js.greet("WebAssembly with npm. from rust");
});

// This imports the new module from the node_modules folder.
// Once it's loaded, it calls the greet func from that module, passing "WebAssembly as a string.
// We're calling into Rust code!!!!