const rust = import('../build/rust_wasm_demo');

rust.then(m => m.greet('username'));
