use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_10_1() {
    let init_info = WindowInitInfo::builder()
        .title("Instancing Quads".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const QUAD_VERTICES: [f32; 30] = [
    // pos (2d)  // color
    -0.05,  0.05,  1.0, 0.0, 0.0,
    0.05, -0.05,  0.0, 1.0, 0.0,
    -0.05, -0.05,  0.0, 0.0, 1.0,

    -0.05,  0.05,  1.0, 0.0, 0.0,
    0.05, -0.05,  0.0, 1.0, 0.0,
    0.05,  0.05,  0.0, 1.0, 1.0
];

struct App {
    instance_vbo: Buffer,
    quad_vbo: Buffer,
    quad_vao: VertexArray,

    shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_10_1_instancing.vs"),
            include_str!("shaders/_10_1_instancing.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);

        // generate a list of 100 quad locations/translation-vectors
        // ---------------------------------------------------------
        let mut translations = Vec::with_capacity(100);
        let offset = 0.1;
        for y in (-10..10).step_by(2) {
            for x in (-10..10).step_by(2) {
                let translation = glm::vec2(x as f32 / 10.0 + offset, y as f32 / 10.0 + offset);
                translations.push(translation);
            }
        }

        // store instance data in an array buffer
        // --------------------------------------
        let instance_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(instance_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&translations),
            STATIC_DRAW,
        );

        let quad_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(quad_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&QUAD_VERTICES),
            STATIC_DRAW,
        );

        let quad_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(quad_vao));
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
        // also set instance data
        let stride = 2 * size_of::<f32>() as i32;
        gl.bind_buffer(ARRAY_BUFFER, Some(instance_vbo)); // this attribute comes from a different vertex buffer
        gl.vertex_attrib_pointer_f32(2, 2, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_divisor(2, 1); // tell OpenGL this is an instanced vertex attribute.

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        Self {
            instance_vbo,
            quad_vbo,
            quad_vao,
            shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        self.shader.use_shader(gl);
        gl.bind_vertex_array(Some(self.quad_vao));
        gl.draw_arrays_instanced(TRIANGLES, 0, 6, 100); // 100 triangles of 6 vertices each

        gl.bind_vertex_array(None);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.quad_vao);
        gl.delete_buffer(self.quad_vbo);
        gl.delete_buffer(self.instance_vbo);
    }
}
