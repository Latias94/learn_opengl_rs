use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use std::mem::size_of;

pub async unsafe fn main_1_2_4() {
    let init_info = WindowInitInfo::builder()
        .title("Hello Triangle Exercise 2".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

const FIRST_TRIANGLE: [f32; 9] = [
    -0.9, -0.5, 0.0, // left
    -0.0, -0.5, 0.0, // right
    -0.45, 0.5, 0.0, // top
];
const SECOND_TRIANGLE: [f32; 9] = [
    0.0, -0.5, 0.0, // left
    0.9, -0.5, 0.0, // right
    0.45, 0.5, 0.0, // top
];

struct App {
    first_vao: VertexArray,
    first_vbo: Buffer,
    second_vao: VertexArray,
    second_vbo: Buffer,
    program: Program,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = &ctx.gl();
        let shader_version = ctx.suggested_shader_version();
        let first_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        let first_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        let second_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        let second_vbo = gl.create_buffer().expect("Cannot create vbo buffer");

        gl.bind_vertex_array(Some(first_vao));

        gl.bind_buffer(ARRAY_BUFFER, Some(first_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&FIRST_TRIANGLE),
            STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 3 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_vertex_array(Some(second_vao));

        gl.bind_buffer(ARRAY_BUFFER, Some(second_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&SECOND_TRIANGLE),
            STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 3 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        // note that this is allowed, the call to glVertexAttribPointer registered VBO
        // as the vertex attribute's bound vertex buffer object so afterward we can safely unbind
        gl.bind_buffer(ARRAY_BUFFER, None);
        // You can unbind the VAO afterward so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyway, so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl.bind_vertex_array(None);

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"layout (location = 0) in vec3 aPos;
                void main()
                {
                    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                }"#,
            // precision mediump float; is needed for WebGL, or it will raise ERROR: 0:2: '' : No precision specified for (float)
            r#"
                precision mediump float;
                out vec4 FragColor;
                void main()
                {
                    FragColor = vec4(1.0f, 0.5, 0.2f, 1.0f);
                }"#,
        );

        let shader_sources = [
            (VERTEX_SHADER, vertex_shader_source),
            (FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        Self {
            first_vao,
            first_vbo,
            second_vao,
            second_vbo,
            program,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = &ctx.gl();
        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        gl.clear(COLOR_BUFFER_BIT);
        gl.use_program(Some(self.program));

        gl.bind_vertex_array(Some(self.first_vao));
        gl.draw_arrays(TRIANGLES, 0, 3);

        gl.bind_vertex_array(Some(self.second_vao));
        gl.draw_arrays(TRIANGLES, 0, 3);
        // gl.bind_vertex_array(None); // no need to unbind it every time
    }

    unsafe fn resize(&mut self, ctx: &AppContext, width: u32, height: u32) {
        let gl = &ctx.gl();
        gl.viewport(0, 0, width as i32, height as i32);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = &ctx.gl();

        gl.delete_program(self.program);

        gl.delete_vertex_array(self.first_vao);

        gl.delete_buffer(self.first_vbo);

        gl.delete_vertex_array(self.second_vao);

        gl.delete_buffer(self.second_vbo);
    }
}
