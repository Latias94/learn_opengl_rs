use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use std::mem::size_of;

pub fn main_1_3_3() {
    let init_info = WindowInitInfo::builder()
        .title("Shaders Class".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
    }
}

const VERTICES: [f32; 18] = [
    // pos           color
    0.5, -0.5, 0.0, 1.0, 0.0, 0.0, //
    -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, //
    0.0, 0.5, 0.0, 0.0, 0.0, 1.0, //
];

struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    shader: MyShader,
}

impl Application for App {
    fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/3.3.shader.vs"),
            include_str!("./shaders/3.3.shader.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");
        Self {
            shader,
            vao: None,
            vbo: None,
        }
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            let vbo = gl.create_buffer().expect("Cannot create vbo buffer");

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 6 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(
                1,
                3,
                FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                3 * size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(1);

            // note that this is allowed, the call to glVertexAttribPointer registered VBO
            // as the vertex attribute's bound vertex buffer object so afterward we can safely unbind
            gl.bind_buffer(ARRAY_BUFFER, None);
            // You can unbind the VAO afterward so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
            // VAOs requires a call to glBindVertexArray anyway, so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
            gl.bind_vertex_array(None);

            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);

            self.shader.use_shader(gl);

            // seeing as we only have a single VAO there's no need to bind it every time,
            // but we'll do so to keep things a bit more organized
            gl.bind_vertex_array(self.vao);
            gl.draw_arrays(TRIANGLES, 0, 3);
        }
    }

    fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        unsafe {
            let gl = &ctx.gl;
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            self.shader.delete(gl);

            if let Some(vertex_array) = self.vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(buffer) = self.vbo {
                gl.delete_buffer(buffer);
            }
        }
    }
}
