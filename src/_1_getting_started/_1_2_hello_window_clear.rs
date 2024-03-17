use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;

pub fn main_1_1_2() {
    let init_info = WindowInitInfo::builder()
        .title("Hello Window Clear".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
    }
}

#[derive(Default)]
struct App {}

impl Application for App {
    fn new(_ctx: &GLContext) -> Self {
        Self::default()
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear(COLOR_BUFFER_BIT);
        }
    }

    fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        unsafe {
            let gl = &ctx.gl;
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}
