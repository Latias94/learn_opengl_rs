use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_9_1() {
    let init_info = WindowInitInfo::builder()
        .title("Geometry Shader Houses".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const POINTS: [f32; 20] = [
    // pos (2d)  // color
    -0.5,  0.5, 1.0, 0.0, 0.0, // top-left
    0.5,  0.5, 0.0, 1.0, 0.0, // top-right
    0.5, -0.5, 0.0, 0.0, 1.0, // bottom-right
    -0.5, -0.5, 1.0, 1.0, 0.0  // bottom-left
];

struct App {
    vbo: Buffer,
    vao: VertexArray,

    shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_with_geometry_from_source(
            gl,
            include_str!("shaders/_9_1_geometry_shader.vs"),
            include_str!("shaders/_9_1_geometry_shader.fs"),
            include_str!("shaders/_9_1_geometry_shader.gs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);

        let vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&POINTS), STATIC_DRAW);

        let vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vao));
        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            2 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

        Self {
            vbo,
            vao,
            shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        self.shader.use_shader(gl);
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_arrays(glow::POINTS, 0, 4);

        gl.bind_vertex_array(None);
        gl.depth_func(LESS);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_buffer(self.vbo);
        gl.delete_vertex_array(self.vao);
    }
}
