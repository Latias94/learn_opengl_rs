winit INDEX="1_1_1":
  @echo 'Run winit with INDEX={{INDEX}}'
  cargo run --features=glutin_winit -- {{INDEX}}

sdl2 INDEX="1_1_1":
  @echo 'Run sdl2 with INDEX={{INDEX}}'
  cargo run --features=sdl2 -- {{INDEX}}

web INDEX="1_1_1":
  @echo 'Build and serve for web'
  cargo build --target wasm32-unknown-unknown && wasm-bindgen ./target/wasm32-unknown-unknown/debug/learn_opengl_rs.wasm --out-dir web --target web
  @echo 'Open http://localhost:8000 in your browser!'
  cd web && python3 -m http.server
