# Learn OpenGL in Rust

Following [learnopengl.com](https://learnopengl.com/) with Rust, compatible with both desktop and WebGL via
WebAssembly (wasm).

Using [glow](https://github.com/grovesNL/glow), [glutin](https://github.com/rust-windowing/glutin), [winit](https://github.com/rust-windowing/winit), [winit_input_helper](https://github.com/rukai/winit_input_helper), [nalgebra-glm](https://github.com/dimforge/nalgebra)
etc.

This project make some different choices from the original tutorial, such as window abstraction, input handling,
camera control etc.

## Chapters

### [1. Getting started](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_1_getting_started)

<p align="left">
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/1_3_2.png" width="32%" alt="1_3_2" />
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/1_4_2.png" width="32%"  alt="1_4_2"/>
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/1_7_3.png" width="32%"  alt="1_7_3"/>
</p>

### [2. Lighting](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_2_lighting)

<p align="left">
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/2_2_1.png" width="32%" alt="2_2_1" />
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/2_4_2.png" width="32%"  alt="2_4_2"/>
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/2_6_1.png" width="32%"  alt="2_6_1"/>
</p>

### [3. Model loading](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_3_model_loading)

<p align="left">
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/3_1_1.png" width="32%" alt="3_1_1" />
  <img src="https://github.com/Latias94/learn_opengl_rs/raw/HEAD/misc/screenshots/3_1_1-web.png" width="47%" alt="3_1_1-web" />
</p>

### [4. Advanced OpenGL](https://github.com/Latias94/learn_opengl_rs/tree/main/src/_4_advanced_opengl)

## Notes

- The function to load models and textures is implemented in `resources.rs`. Basically `build.rs` file will copy all
  resources to the output directory, so we can use relative path to load resources. For wasm, `build.rs` file will also
  copy resources to `web` directory, then we can download them from the local server.
- I use `tobj` crate to load models, thus only support `.obj` format.
- I use `include_str!` macro to load shaders for simplicity.
- `egui` is only supported in desktop build for now, demo: `just run 9_1_1`.

## Build

### Prerequisites

Install [Just](https://github.com/casey/just?tab=readme-ov-file#installation), a handy command runner. Or you can run
command found in `justfile` manually.

### Desktop

```shell
just run 1_2_1 # Debug build then Run tutorial 1_2_1
just rrun 3_1_1 # Release build then run tutorial 3_1_1
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
