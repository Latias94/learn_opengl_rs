use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use std::mem::size_of;

pub fn main_1_2_2() {
    let init_info = WindowInitInfo::builder()
        .title("Hello Triangle Indexed".to_string())
        .build();
    unsafe {
        run(init_info, App::default());
    }
}

const VERTICES: [f32; 12] = [
    0.5, 0.5, 0.0, //
    0.5, -0.5, 0.0, //
    -0.5, -0.5, 0.0, //
    -0.5, 0.5, 0.0, //
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

#[derive(Default)]
struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    program: Option<Program>,
}

impl Application for App {
    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            let shader_version = ctx.shader_version;
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            let vbo = gl.create_buffer().expect("Cannot create vbo buffer");
            let ebo = gl.create_buffer().expect("Cannot create ebo buffer");

            // 1. bind Vertex Array Object
            gl.bind_vertex_array(Some(vao));

            // 2. copy our vertices array in a vertex buffer for OpenGL to use
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

            // 3. copy our index array in a element buffer for OpenGL to use
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&INDICES),
                STATIC_DRAW,
            );

            // 4. then set the vertex attributes pointers
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
                    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
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

            self.program = Some(program);
            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);
            gl.use_program(self.program);
            // seeing as we only have a single VAO there's no need to bind it every time,
            // but we'll do so to keep things a bit more organized
            gl.bind_vertex_array(self.vao);
            gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
            // gl.bind_vertex_array(None); // no need to unbind it every time
        }
    }

    fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        log::info!("Resizing to {}x{}", width, height);
        unsafe {
            let gl = &ctx.gl;
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            if let Some(program) = self.program {
                gl.delete_program(program);
            }

            if let Some(vertex_array) = self.vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(buffer) = self.vbo {
                gl.delete_buffer(buffer);
            }
        }
    }
}
