use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_8_1() {
    let init_info = WindowInitInfo::builder()
        .title("Advanced GLSL Ubo".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 108] = [
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,

    -0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,

    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,

    0.5,  0.5,  0.5,
    0.5,  0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,

    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,

    -0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
];

struct App {
    cube_vbo: Buffer,
    cube_vao: VertexArray,

    shader_red: MyShader,
    shader_green: MyShader,
    shader_blue: MyShader,
    shader_yellow: MyShader,

    ubo_matrices: Buffer,

    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader_red = MyShader::new_from_source(
            gl,
            include_str!("shaders/_8_1_advanced_glsl.vs"),
            include_str!("shaders/_8_1_red.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let shader_green = MyShader::new_from_source(
            gl,
            include_str!("shaders/_8_1_advanced_glsl.vs"),
            include_str!("shaders/_8_1_green.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let shader_blue = MyShader::new_from_source(
            gl,
            include_str!("shaders/_8_1_advanced_glsl.vs"),
            include_str!("shaders/_8_1_blue.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let shader_yellow = MyShader::new_from_source(
            gl,
            include_str!("shaders/_8_1_advanced_glsl.vs"),
            include_str!("shaders/_8_1_yellow.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);

        //  cube vao
        let cube_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(cube_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&CUBE_VERTICES),
            STATIC_DRAW,
        );

        let cube_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(cube_vao));
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 3 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_vertex_array(None);

        // configure a uniform buffer object
        // ---------------------------------
        // first. We get the relevant block indices
        let uniform_block_index_red = gl
            .get_uniform_block_index(shader_red.program(), "Matrices")
            .expect("Failed to get uniform block index");
        let uniform_block_index_green = gl
            .get_uniform_block_index(shader_green.program(), "Matrices")
            .expect("Failed to get uniform block index");
        let uniform_block_index_blue = gl
            .get_uniform_block_index(shader_blue.program(), "Matrices")
            .expect("Failed to get uniform block index");
        let uniform_block_index_yellow = gl
            .get_uniform_block_index(shader_yellow.program(), "Matrices")
            .expect("Failed to get uniform block index");
        // then we link each shader's uniform block to this uniform binding point
        gl.uniform_block_binding(shader_red.program(), uniform_block_index_red, 0);
        gl.uniform_block_binding(shader_green.program(), uniform_block_index_green, 0);
        gl.uniform_block_binding(shader_blue.program(), uniform_block_index_blue, 0);
        gl.uniform_block_binding(shader_yellow.program(), uniform_block_index_yellow, 0);

        // Now actually create the buffer
        let ubo_matrices = gl.create_buffer().expect("Failed to create ubo buffer");
        gl.bind_buffer(UNIFORM_BUFFER, Some(ubo_matrices));
        gl.buffer_data_size(
            UNIFORM_BUFFER,
            2 * size_of::<glm::Mat4>() as i32,
            STATIC_DRAW,
        );
        gl.bind_buffer(UNIFORM_BUFFER, None);
        // define the range of the buffer that links to a uniform binding point
        gl.bind_buffer_range(
            UNIFORM_BUFFER,
            0,
            Some(ubo_matrices),
            0,
            2 * size_of::<glm::Mat4>() as i32,
        );

        // store the projection matrix (we only do this once now) (note: we're not using zoom anymore by changing the FoV)
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            45.0_f32.to_radians(),
            0.1,
            100.0,
        );
        gl.bind_buffer(UNIFORM_BUFFER, Some(ubo_matrices));
        gl.buffer_sub_data_u8_slice(
            UNIFORM_BUFFER,
            0,
            bytemuck::cast_slice(&projection.as_slice()),
        );
        gl.bind_buffer(UNIFORM_BUFFER, None);

        Self {
            cube_vbo,
            cube_vao,
            shader_red,
            shader_green,
            shader_blue,
            shader_yellow,
            ubo_matrices,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // set the view and projection matrix in the uniform block - we only have to do this once per loop iteration.
        let view = self.camera.view_matrix();
        gl.bind_buffer(UNIFORM_BUFFER, Some(self.ubo_matrices));
        gl.buffer_sub_data_u8_slice(
            UNIFORM_BUFFER,
            size_of::<glm::Mat4>() as i32,
            bytemuck::cast_slice(&view.as_slice()),
        );
        gl.bind_buffer(UNIFORM_BUFFER, None);

        // draw 4 cubes
        // RED
        gl.bind_vertex_array(Some(self.cube_vao));
        self.shader_red.use_shader(gl);
        let model = glm::translation(&glm::vec3(-0.75, 0.75, 0.0)); // move top-left
        self.shader_red.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // GREEN
        self.shader_green.use_shader(gl);
        let model = glm::translation(&glm::vec3(0.75, 0.75, 0.0)); // move top-right
        self.shader_green.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // YELLOW
        self.shader_yellow.use_shader(gl);
        let model = glm::translation(&glm::vec3(-0.75, -0.75, 0.0)); // move bottom-left
        self.shader_yellow.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // BLUE
        self.shader_blue.use_shader(gl);
        let model = glm::translation(&glm::vec3(0.75, -0.75, 0.0)); // move bottom-right
        self.shader_blue.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        gl.bind_vertex_array(None);
        gl.depth_func(LESS);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader_red.delete(gl);
        self.shader_green.delete(gl);
        self.shader_blue.delete(gl);
        self.shader_yellow.delete(gl);

        gl.delete_buffer(self.cube_vbo);
        gl.delete_vertex_array(self.cube_vao);

        gl.delete_buffer(self.ubo_matrices);
    }
}
