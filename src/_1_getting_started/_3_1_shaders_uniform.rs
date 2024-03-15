use crate::window::{run, Application, GLContext, WindowInitInfo};
use chrono::Utc;
use glow::*;
use std::mem::size_of;

pub fn main_1_3_1() {
    let init_info = WindowInitInfo::builder()
        .title("Shaders Uniform".to_string())
        .build();
    unsafe {
        run(init_info, App::default());
    }
}

const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0, //
    0.5, -0.5, 0.0, //
    0.0, 0.5, 0.0,
];

#[derive(Default)]
struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    program: Option<Program>,
    start: chrono::DateTime<Utc>,
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

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 3 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);

            // note that this is allowed, the call to glVertexAttribPointer registered VBO
            // as the vertex attribute's bound vertex buffer object so afterward we can safely unbind
            gl.bind_buffer(ARRAY_BUFFER, None);
            // You can unbind the VAO afterward so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
            // VAOs requires a call to glBindVertexArray anyway, so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
            gl.bind_vertex_array(None);

            let (vertex_shader_source, fragment_shader_source) = (
                r#"layout (location = 0) in vec3 aPos;
                void main()
                {
                    gl_Position = vec4(aPos, 1.0);
                }"#,
                r#"
                precision mediump float;
                out vec4 FragColor;
                uniform vec4 ourColor;
                void main()
                {
                    FragColor = ourColor;
                }"#,
            );
            let program = create_program(
                &gl,
                vertex_shader_source,
                fragment_shader_source,
                shader_version,
            )
            .expect("Failed to create program");

            self.program = Some(program);
            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.start = Utc::now();
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);

            gl.use_program(self.program);

            let current = Utc::now();
            let duration = current - self.start;
            let time = duration.num_milliseconds() as f32 / 1000.0;
            let green_value = (time.sin() / 2.0) + 0.5;

            let our_color = gl
                .get_uniform_location(self.program.unwrap(), "ourColor")
                .unwrap();
            gl.uniform_4_f32(Some(&our_color), 0.0, green_value, 0.0, 1.0);

            // seeing as we only have a single VAO there's no need to bind it every time,
            // but we'll do so to keep things a bit more organized
            gl.bind_vertex_array(self.vao);
            gl.draw_arrays(TRIANGLES, 0, 3);
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

fn create_program(
    gl: &Context,
    vertex_shader: &str,
    fragment_shader: &str,
    shader_version: &str,
) -> Result<Program, String> {
    let (vertex_shader, fragment_shader) = (
        format!("{}\n{}", shader_version, vertex_shader),
        format!("{}\n{}", shader_version, fragment_shader),
    );

    let program = unsafe { gl.create_program().expect("Cannot create program") };

    let (vertex, fragment) = (
        compile_shader(&gl, VERTEX_SHADER, &vertex_shader)?,
        compile_shader(&gl, FRAGMENT_SHADER, &fragment_shader)?,
    );

    unsafe {
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);
    }

    if !unsafe { gl.get_program_link_status(program) } {
        return Err(unsafe { gl.get_program_info_log(program) });
    }

    unsafe {
        gl.detach_shader(program, vertex);
        gl.detach_shader(program, fragment);
        gl.delete_shader(vertex);
        gl.delete_shader(fragment);
    }

    Ok(program)
}

fn compile_shader(gl: &Context, shader_type: u32, source: &str) -> Result<Shader, String> {
    let shader = unsafe { gl.create_shader(shader_type).expect("Cannot create shader") };
    unsafe {
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
    }

    if !unsafe { gl.get_shader_compile_status(shader) } {
        return Err(unsafe { gl.get_shader_info_log(shader) });
    }

    Ok(shader)
}
