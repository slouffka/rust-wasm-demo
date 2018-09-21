const wasm = import('../build/rust_wasm_demo');

wasm.then(m => m.greet('vortex'));
