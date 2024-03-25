use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_2_1() {
    let init_info = WindowInitInfo::builder()
        .title("Stencil Testing".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const VERTICES: [f32; 180] = [
    // pos           // texture Coords
    -0.5, -0.5, -0.5,  0.0, 0.0,
    0.5, -0.5, -0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5,  0.5,  0.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  1.0, 1.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0
];

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 30] = [
    // positions    texture Coords
    //  (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
    5.0, -0.5, 5.0, 2.0, 0.0,
    -5.0, -0.5, 5.0, 0.0, 0.0,
    -5.0, -0.5, -5.0, 0.0, 2.0,

    5.0, -0.5, 5.0, 2.0, 0.0,
    -5.0, -0.5, -5.0, 0.0, 2.0,
    5.0, -0.5, -5.0, 2.0, 2.0
];

struct App {
    cube_vbo: Buffer,
    cube_vao: VertexArray,
    cube_texture: texture::Texture,

    plane_vbo: Buffer,
    plane_vao: VertexArray,
    plane_texture: texture::Texture,

    shader: MyShader,
    shader_single_color: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_1_1_depth_testing.vs"),
            include_str!("shaders/_1_1_depth_testing.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let shader_single_color = MyShader::new_from_source(
            gl,
            include_str!("shaders/_1_1_depth_testing.vs"),
            include_str!("shaders/_2_1_stencil_single_color.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);
        gl.depth_func(LESS);
        gl.enable(STENCIL_TEST);
        gl.stencil_func(NOTEQUAL, 1, 0xFF);
        gl.stencil_op(KEEP, KEEP, REPLACE);

        //  cube vao
        let cube_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(cube_vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

        let cube_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(cube_vao));
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);

        // plane vao
        let plane_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(plane_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&PLANE_VERTICES),
            STATIC_DRAW,
        );

        let plane_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(plane_vao));
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);

        // load texture
        let cube_texture = resources::load_texture(gl, "textures/marble.jpg")
            .await
            .expect("Failed to load texture");
        let plane_texture = resources::load_texture(gl, "textures/metal.png")
            .await
            .expect("Failed to load texture");

        shader.use_shader(gl);
        shader.set_int(gl, "texture1", 0);

        Self {
            cube_vbo,
            cube_vao,
            cube_texture,
            plane_vbo,
            plane_vao,
            plane_texture,
            shader,
            shader_single_color,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT);

        // set uniforms
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            self.camera.zoom().to_radians(),
            0.1,
            100.0,
        );
        let view = self.camera.view_matrix();

        self.shader_single_color.use_shader(gl);
        self.shader_single_color
            .set_mat4(gl, "projection", &projection);
        self.shader_single_color.set_mat4(gl, "view", &view);

        self.shader.use_shader(gl);
        self.shader.set_mat4(gl, "projection", &projection);
        self.shader.set_mat4(gl, "view", &view);

        // draw plane as normal, but don't write the plane to the stencil buffer,
        // we only care about the cubes. We set its mask to 0x00 to not write to the stencil buffer.
        gl.stencil_mask(0x00);
        // plane
        gl.bind_vertex_array(Some(self.plane_vao));
        self.plane_texture.bind(gl, 0);
        let model = glm::Mat4::identity();
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 6);

        // 1st. render pass, draw objects as normal, writing to the stencil buffer
        // --------------------------------------------------------------------
        gl.stencil_func(ALWAYS, 1, 0xFF);
        gl.stencil_mask(0xFF);

        // cubes
        gl.bind_vertex_array(Some(self.cube_vao));
        self.cube_texture.bind(gl, 0);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // 2nd. render pass: now draw slightly scaled versions of the objects, this time disabling stencil writing.
        // Because the stencil buffer is now filled with several 1s. The parts of the buffer that are 1 are not drawn, thus only drawing
        // the objects' size differences, making it look like borders.
        // -----------------------------------------------------------------------------------------------------------------------------
        gl.stencil_func(NOTEQUAL, 1, 0xFF);
        gl.stencil_mask(0x00);
        gl.disable(DEPTH_TEST);

        self.shader_single_color.use_shader(gl);
        let scale = 1.1;
        gl.bind_vertex_array(Some(self.cube_vao));
        self.cube_texture.bind(gl, 0);

        let mut model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
        model = glm::scale(&model, &glm::vec3(scale, scale, scale));
        self.shader_single_color.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        let mut model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
        model = glm::scale(&model, &glm::vec3(scale, scale, scale));
        self.shader_single_color.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        gl.bind_vertex_array(None);

        gl.stencil_mask(0xFF);
        gl.stencil_func(ALWAYS, 0, 0xFF);
        gl.enable(DEPTH_TEST);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);
        self.shader_single_color.delete(gl);

        gl.delete_buffer(self.cube_vbo);
        gl.delete_vertex_array(self.cube_vao);
        self.cube_texture.delete(gl);

        gl.delete_buffer(self.plane_vbo);
        gl.delete_vertex_array(self.plane_vao);
        self.plane_texture.delete(gl);
    }
}
