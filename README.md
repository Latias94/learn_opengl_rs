# Learn OpenGL in Rust

Following [learnopengl.com](https://learnopengl.com/) with Rust, compatible with both desktop and WebGL via
WebAssembly (wasm).

Using [glow](https://github.com/grovesNL/glow), [glutin](https://github.com/rust-windowing/glutin), [winit](https://github.com/rust-windowing/winit), [winit_input_helper](https://github.com/rukai/winit_input_helper), [nalgebra-glm](https://github.com/dimforge/nalgebra)
etc.

## Chapters

### [1. Getting started](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_1_getting_started)

### [2. Lighting](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_2_lighting)

## Build

### Prerequisites

Install [Just](https://github.com/casey/just?tab=readme-ov-file#installation), a handy command runner. Or you can run
command found in `justfile` manually.

### Desktop

```shell
just run 1_2_1 # Run tutorial 1_2_1
```

### WebGL

#### wasm-pack (Recommended)

`wasm-pack` has bundled [WebAssembly/binaryen](https://github.com/WebAssembly/binaryen) for you, which can optimize wasm
file.

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
2. To build and run with wasm-pack (generates an optimized wasm file), then visit http://127.0.0.1:8000/?tutorial=1_2_1.
    ```shell
     just web
    ```

#### wasm-bindgen

Alternatively, use `wasm-bindgen` directly for a less optimized build.

1. Install `wasm32-unknown-unknown` target and `wasm-bindgen-cli`:
    ```shell
    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli
    ```
2. To build and run with wasm-bindgen, then visit http://127.0.0.1:8000/?tutorial=1_2_1.
    ```shell
    just web-bindgen
    ```
