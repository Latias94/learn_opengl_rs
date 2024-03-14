winit NUM="1.1":
  @echo 'Run winit'
  cargo run --features=glutin_winit

sdl2 NUM="1.1":
  @echo 'Run sdl2'
  cargo run --features=sdl2

web NUM="1.1":
  @echo 'Run 1.1'
  cargo build --target wasm32-unknown-unknown && wasm-bindgen ./target/wasm32-unknown-unknown/debug/learn_opengl_rs.wasm --out-dir web --target web
  @echo 'Open http://localhost:8000 in your browser!'
  cd web && python3 -m http.server