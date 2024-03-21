build:
  @echo 'Build client in release mode'
  cargo build -r

build-web:
  @echo 'Build wasm in release mode'
  wasm-pack build --out-dir web --target web --release

run TUTORIAL="1_1_1":
  @echo 'Debug mode, Run winit with TUTORIAL={{TUTORIAL}}'
  RUST_LOG=info cargo run -- {{TUTORIAL}}

rrun TUTORIAL="1_1_1":
  @echo 'Release mode, Run winit with TUTORIAL={{TUTORIAL}}'
  RUST_LOG=info cargo run -r -- {{TUTORIAL}}

web TUTORIAL="1_2_1":
  @echo 'Build and serve for web'
  wasm-pack build --out-dir web --target web
  @echo 'Open http://127.0.0.1:8000/?tutorial={{TUTORIAL}} in your browser!'
  @echo 'You can also change the tutorial number in the URL to see different tutorials. e.g. 1_1_1, 1_1_2, 1_2_1, ...'
  cd web && python3 -m http.server

web-bindgen:
  @echo 'Build and serve for web'
  cargo build --target wasm32-unknown-unknown && wasm-bindgen ./target/wasm32-unknown-unknown/debug/learn_opengl_rs.wasm --out-dir web --target web
  @echo 'Open http://127.0.0.1:8000/?tutorial=1_2_1 in your browser!'
  @echo 'You can also change the tutorial number in the URL to see different tutorials. e.g. 1_1_1, 1_1_2, 1_2_1, ...'
  cd web && python3 -m http.server
