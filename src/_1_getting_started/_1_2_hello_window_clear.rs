use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;

pub async unsafe fn main_1_1_2() {
    let init_info = WindowInitInfo::builder()
        .title("Hello Window Clear".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {}

impl Application for App {
    async unsafe fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        unsafe {
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
        }
        Self {}
    }

    unsafe fn render(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            gl.clear(COLOR_BUFFER_BIT);
        }
    }

    unsafe fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        let gl = &ctx.gl;
        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }
}
