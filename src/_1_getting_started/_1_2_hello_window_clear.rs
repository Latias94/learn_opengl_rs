use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;

pub fn main_1_1_2() {
    let init_info = WindowInitInfo {
        width: 1024,
        height: 768,
        title: "Hello window clear".to_string(),
        major: 3,
        minor: 3,
    };
    unsafe {
        run(init_info, App {});
    }
}

struct App {}

impl Application for App {
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

    fn handle_event(&mut self, _ctx: &GLContext) {}

    fn exit(&mut self, _ctx: &GLContext) {}
}
