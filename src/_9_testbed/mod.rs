#[cfg(not(target_arch = "wasm32"))]
mod _1_1_egui;
#[cfg(not(target_arch = "wasm32"))]
pub use _1_1_egui::main_9_1_1;
