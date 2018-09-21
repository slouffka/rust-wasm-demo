# Rust -> WASM -> JS Demo

Made using this manual: https://rustwasm.github.io/wasm-bindgen/examples/hello-world.html

## Installation && Running

Install Rust:

```
curl https://sh.rustup.rs -sSf | sh
```

Set Rust Nightly as default:

```
rustup default nightly
```

Next, we need to install the tools for WASM:

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

Then install Node.js dependencies with Yarn:

```
yarn
yarn build
yarn serve
```
