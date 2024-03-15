use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;

pub fn main_1_2_1() {
    let init_info = WindowInitInfo {
        width: 1024,
        height: 768,
        title: "Hello Triangle".to_string(),
        major: 3,
        minor: 3,
    };
    unsafe {
        run(init_info, App::default());
    }
}

#[derive(Default)]
struct App {
    vertex_array: Option<VertexArray>,
    program: Option<Program>,
}

impl Application for App {
    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            let shader_version = ctx.shader_version;
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
                r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
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

            gl.use_program(Some(program));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);

            self.program = Some(program);
            self.vertex_array = Some(vertex_array);
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear(COLOR_BUFFER_BIT);
            gl.draw_arrays(TRIANGLES, 0, 3);
        }
    }

    fn handle_event(&mut self, _ctx: &GLContext) {}

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            if let Some(program) = self.program {
                gl.delete_program(program);
            }

            if let Some(vertex_array) = self.vertex_array {
                gl.delete_vertex_array(vertex_array);
            }
        }
    }
}
