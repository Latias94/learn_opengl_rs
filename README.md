# Learn OpenGL in Rust

Install [Just](https://github.com/casey/just?tab=readme-ov-file#installation), [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

```shell
just run 1_2_1 # Run tutorial 1_2_1 with glutin-winit
just web # Build wasm with wasm-pack and run, generate smaller wasm file
just web-bindgen # Run wasm, then you can open http://127.0.0.1:8000/?tutorial=1_2_1
```

## Todos

- [ ] implement update in wasm
