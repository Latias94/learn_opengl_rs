use crate::camera::Camera;
use crate::resources::load_binary;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use easy_imgui_window::easy_imgui::{Ui, UiBuilder};
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_6_1() {
    let init_info = WindowInitInfo::builder()
        .title("Cubemaps Skybox".to_string())
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
const SKYBOX_VERTICES: [f32; 108] = [
    -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,

    1.0, -1.0, -1.0,
    1.0, -1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0, -1.0,
    1.0, -1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,

    -1.0,  1.0, -1.0,
    1.0,  1.0, -1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    1.0, -1.0,  1.0
];

struct App {
    cube_vbo: Buffer,
    cube_vao: VertexArray,
    cube_texture: texture::Texture,

    skybox_vbo: Buffer,
    skybox_vao: VertexArray,
    skybox_texture: Texture,

    shader: MyShader,
    skybox_shader: MyShader,
    camera: Camera,
}

impl UiBuilder for App {
    fn do_ui(&mut self, ui: &Ui<Self>) {
        ui.show_demo_window(None);
    }
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

        let skybox_shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_6_1_skybox.vs"),
            include_str!("shaders/_6_1_skybox.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);
        // gl.depth_func(LESS);
        gl.depth_func(ALWAYS); // always pass the depth test (same effect as glDisable(GL_DEPTH_TEST))

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

        // skybox vao
        let skybox_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(skybox_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&SKYBOX_VERTICES),
            STATIC_DRAW,
        );

        let skybox_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(skybox_vao));
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 3 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        gl.bind_vertex_array(None);

        // load texture
        let cube_texture = resources::load_texture(gl, "textures/container.jpg")
            .await
            .expect("Failed to load texture");
        let skybox_texture = load_cubemap(
            gl,
            &[
                "textures/skybox/right.jpg",
                "textures/skybox/left.jpg",
                "textures/skybox/top.jpg",
                "textures/skybox/bottom.jpg",
                "textures/skybox/front.jpg",
                "textures/skybox/back.jpg",
            ],
        )
        .await;

        shader.use_shader(gl);
        shader.set_int(gl, "texture1", 0);

        skybox_shader.use_shader(gl);
        skybox_shader.set_int(gl, "skybox", 0);

        Self {
            cube_vbo,
            cube_vao,
            cube_texture,
            skybox_vbo,
            skybox_vao,
            skybox_texture,
            shader,
            skybox_shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        self.shader.use_shader(gl);
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            self.camera.zoom().to_radians(),
            0.1,
            100.0,
        );
        let view = self.camera.view_matrix();
        self.shader.set_mat4(gl, "projection", &projection);
        self.shader.set_mat4(gl, "view", &view);

        // cubes
        gl.bind_vertex_array(Some(self.cube_vao));
        self.cube_texture.bind(gl, 0);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // skybox
        gl.bind_vertex_array(Some(self.skybox_vao));
        gl.bind_texture(TEXTURE_2D, Some(self.skybox_texture));
        let model = glm::Mat4::identity();
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 6);

        gl.bind_vertex_array(None);
    }

    unsafe fn resize(&mut self, ctx: &AppContext, width: u32, height: u32) {
        let gl = ctx.gl();
        gl.viewport(0, 0, width as i32, height as i32);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_buffer(self.cube_vbo);
        gl.delete_vertex_array(self.cube_vao);
        self.cube_texture.delete(gl);

        gl.delete_buffer(self.skybox_vbo);
        gl.delete_vertex_array(self.skybox_vao);
        gl.delete_texture(self.skybox_texture);
    }
}

async unsafe fn load_cubemap(gl: &Context, faces: &[&str]) -> Texture {
    let texture = gl.create_texture().expect("Failed to create texture");
    gl.bind_texture(TEXTURE_CUBE_MAP, Some(texture));

    for (i, face) in faces.iter().enumerate() {
        let data = load_binary(face).await.expect("Failed to load texture");
        let img = image::load_from_memory(&data).expect("Failed to load image");
        let img = img.flipv();
        let (width, height) = img.dimensions();
        let data = img.to_rgb8();
        gl.tex_image_2d(
            TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
            0,
            RGB as i32,
            width as i32,
            height as i32,
            0,
            RGB,
            UNSIGNED_BYTE,
            Some(&data),
        );
    }
    gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MIN_FILTER, LINEAR as i32);
    gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_MAG_FILTER, LINEAR as i32);
    gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(TEXTURE_CUBE_MAP, TEXTURE_WRAP_R, CLAMP_TO_EDGE as i32);

    texture
}
