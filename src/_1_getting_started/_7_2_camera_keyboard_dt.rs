use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;
use std::time::Duration;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_1_7_2() {
    let init_info = WindowInitInfo::builder()
        .title("Camera Keyboard Dt".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

// rectangle, pos tex_coord
#[rustfmt::skip]
const VERTICES: [f32; 180] = [
    // pos            tex_coord
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

const CUBE_POSITIONS: [glm::Vec3; 10] = [
    glm::Vec3::new(0.0, 0.0, 0.0),
    glm::Vec3::new(2.0, 5.0, -15.0),
    glm::Vec3::new(-1.5, -2.2, -2.5),
    glm::Vec3::new(-3.8, -2.0, -12.3),
    glm::Vec3::new(2.4, -0.4, -3.5),
    glm::Vec3::new(-1.7, 3.0, -7.5),
    glm::Vec3::new(1.3, -2.0, -2.5),
    glm::Vec3::new(1.5, 2.0, -2.5),
    glm::Vec3::new(1.5, 0.2, -1.5),
    glm::Vec3::new(-1.3, 1.0, -1.5),
];

const CAMERA_FRONT: glm::Vec3 = glm::Vec3::new(0.0, 0.0, -1.0);
const CAMERA_UP: glm::Vec3 = glm::Vec3::new(0.0, 1.0, 0.0);

struct App {
    vao: VertexArray,
    vbo: Buffer,
    texture_1: Texture,
    texture_2: Texture,
    shader: MyShader,
    camera_pos: glm::Vec3,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/6.1.coordinate_systems.vs"),
            include_str!("./shaders/5.1.transform.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        gl.enable(DEPTH_TEST);

        let vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        let vbo = gl.create_buffer().expect("Cannot create vbo buffer");

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

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

        // texture 1
        // ---------
        let texture_1 = gl.create_texture().expect("Cannot create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture_1));

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

        let img = image::load_from_memory(include_bytes!("../../resources/textures/container.jpg"))
            .expect("Failed to load image")
            .flipv();
        let (width, height) = img.dimensions();
        let img_data = img.to_rgb8().into_raw();
        gl.tex_image_2d(
            // target, level, internal_format, width, height, border, format, type, pixels
            TEXTURE_2D,
            0,
            RGB as i32,
            width as i32,
            height as i32,
            0,
            RGB,
            UNSIGNED_BYTE,
            Some(&img_data),
        );
        gl.generate_mipmap(TEXTURE_2D);

        // texture 2
        // ---------
        let texture_2 = gl.create_texture().expect("Cannot create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture_2));
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

        let img =
            image::load_from_memory(include_bytes!("../../resources/textures/awesomeface.png"))
                .expect("Failed to load image")
                .flipv();
        let (width, height) = img.dimensions();
        let img_data = img.to_rgb8().into_raw();
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGB as i32,
            width as i32,
            height as i32,
            0,
            RGB,
            UNSIGNED_BYTE,
            Some(&img_data),
        );
        gl.generate_mipmap(TEXTURE_2D);

        shader.use_shader(gl);
        shader.set_int(gl, "texture1", 0);
        shader.set_int(gl, "texture2", 1);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        Self {
            shader,
            vao,
            vbo,
            texture_1,
            texture_2,
            camera_pos: glm::vec3(0.0, 0.0, 3.0),
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        gl.active_texture(TEXTURE0);
        gl.bind_texture(TEXTURE_2D, Some(self.texture_1));

        gl.active_texture(TEXTURE1);
        gl.bind_texture(TEXTURE_2D, Some(self.texture_2));

        gl.bind_vertex_array(Some(self.vao));
        self.shader.use_shader(gl);

        let center = self.camera_pos + CAMERA_FRONT;
        let view = glm::look_at(&self.camera_pos, &center, &CAMERA_UP);

        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            45.0_f32.to_radians(),
            0.1,
            100.0,
        );
        self.shader.set_mat4(gl, "view", &view);
        self.shader.set_mat4(gl, "projection", &projection);

        for (i, pos) in CUBE_POSITIONS.iter().enumerate() {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, pos);
            let angle = 20.0 * i as f32;
            model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
            self.shader.set_mat4(gl, "model", &model);
            gl.draw_arrays(TRIANGLES, 0, 36);
        }
    }

    unsafe fn resize(&mut self, ctx: &AppContext, width: u32, height: u32) {
        let gl = ctx.gl();
        gl.viewport(0, 0, width as i32, height as i32);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        let delta_time = input.delta_time().unwrap_or(Duration::new(0, 0));
        let delta_time = delta_time.as_secs_f32();
        let camera_speed = 2.5f32 * delta_time;
        if input.key_held(KeyCode::KeyW) || input.key_held(KeyCode::ArrowUp) {
            self.camera_pos += CAMERA_FRONT * camera_speed;
        } else if input.key_held(KeyCode::KeyS) || input.key_held(KeyCode::ArrowDown) {
            self.camera_pos -= CAMERA_FRONT * camera_speed;
        } else if input.key_held(KeyCode::KeyA) || input.key_held(KeyCode::ArrowLeft) {
            self.camera_pos -=
                glm::normalize(&glm::cross(&CAMERA_FRONT, &CAMERA_UP)) * camera_speed;
        } else if input.key_held(KeyCode::KeyD) || input.key_held(KeyCode::ArrowRight) {
            self.camera_pos +=
                glm::normalize(&glm::cross(&CAMERA_FRONT, &CAMERA_UP)) * camera_speed;
        }
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.vao);

        gl.delete_buffer(self.vbo);

        gl.delete_texture(self.texture_1);

        gl.delete_texture(self.texture_2);
    }
}
