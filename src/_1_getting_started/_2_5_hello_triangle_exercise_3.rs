use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use std::mem::size_of;

pub fn main_1_2_5() {
    let init_info = WindowInitInfo::builder()
        .title("Hello Triangle Exercise 3".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
    }
}

const VERTICES: [f32; 18] = [
    // first triangle
    -0.9, -0.5, 0.0, // left
    -0.0, -0.5, 0.0, // right
    -0.45, 0.5, 0.0, // top
    // second triangle
    0.0, -0.5, 0.0, // left
    0.9, -0.5, 0.0, // right
    0.45, 0.5, 0.0, // top
];

#[derive(Default)]
struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    program_0: Option<Program>,
    program_1: Option<Program>,
}

impl Application for App {
    fn new(_ctx: &GLContext) -> Self {
        Self::default()
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            let shader_version = ctx.suggested_shader_version;
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

            let (vertex_shader_source, fragment_shader_source_0, fragment_shader_source_1) = (
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
                r#"
                precision mediump float;
                out vec4 FragColor;
                void main()
                {
                    FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
                }"#,
            );
            let program_0 = create_program(
                gl,
                vertex_shader_source,
                fragment_shader_source_0,
                shader_version,
            )
            .expect("Failed to create program");
            let program_1 = create_program(
                gl,
                vertex_shader_source,
                fragment_shader_source_1,
                shader_version,
            )
            .expect("Failed to create program");

            self.program_0 = Some(program_0);
            self.program_1 = Some(program_1);
            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);
            // seeing as we only have a single VAO there's no need to bind it every time,
            // but we'll do so to keep things a bit more organized
            gl.bind_vertex_array(self.vao);

            gl.use_program(self.program_0);
            gl.draw_arrays(TRIANGLES, 0, 3);

            gl.use_program(self.program_1);
            gl.draw_arrays(TRIANGLES, 3, 3);
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
            if let Some(program) = self.program_0 {
                gl.delete_program(program);
            }

            if let Some(program) = self.program_1 {
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
        compile_shader(gl, VERTEX_SHADER, &vertex_shader)?,
        compile_shader(gl, FRAGMENT_SHADER, &fragment_shader)?,
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
