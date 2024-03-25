use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use std::mem::size_of;

pub async unsafe fn main_1_3_5() {
    let init_info = WindowInitInfo::builder()
        .title("Shaders Exercise 2".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

const VERTICES: [f32; 18] = [
    // pos           color
    0.5, -0.5, 0.0, 1.0, 0.0, 0.0, //
    -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, //
    0.0, 0.5, 0.0, 0.0, 0.0, 1.0, //
];

struct App {
    vao: VertexArray,
    vbo: Buffer,
    shader: MyShader,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/3.5.shader.vs"),
            include_str!("./shaders/3.3.shader.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

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

        Self { vao, vbo, shader }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        gl.clear(COLOR_BUFFER_BIT);

        self.shader.use_shader(gl);
        self.shader.set_float(gl, "xOffset", 0.5);

        // seeing as we only have a single VAO there's no need to bind it every time,
        // but we'll do so to keep things a bit more organized
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_arrays(TRIANGLES, 0, 3);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.vao);

        gl.delete_buffer(self.vbo);
    }
}
