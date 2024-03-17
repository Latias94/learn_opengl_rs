use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, Key, MouseEvent, WindowInitInfo};
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;

pub fn main_1_7_6() {
    let init_info = WindowInitInfo::builder()
        .title("Camera Exercise 2".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
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

const CAMERA_UP: glm::Vec3 = glm::Vec3::new(0.0, 1.0, 0.0);

struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    texture_1: Option<Texture>,
    texture_2: Option<Texture>,
    shader: MyShader,
    camera_pos: glm::Vec3,
    camera_front: glm::Vec3,
    first_mouse: bool,
    yaw: f32,
    pitch: f32,
    last_x: f64,
    last_y: f64,
    fov: f32,
}

impl Application for App {
    fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/6.1.coordinate_systems.vs"),
            include_str!("./shaders/5.1.transform.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");
        // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
        let yaw = -90.0f32;
        let last_x = ctx.width as f64 * ctx.scale_factor / 2.0;
        let last_y = ctx.height as f64 * ctx.scale_factor / 2.0;
        let fov = 45.0f32;
        Self {
            shader,
            vao: None,
            vbo: None,
            texture_1: None,
            texture_2: None,
            camera_pos: glm::vec3(0.0, 0.0, 3.0),
            camera_front: glm::vec3(0.0, 0.0, -1.0),
            first_mouse: false,
            yaw,
            pitch: 0.0,
            last_x,
            last_y,
            fov,
        }
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;

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

            let img =
                image::load_from_memory(include_bytes!("../../resources/textures/container.jpg"))
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

            self.shader.use_shader(gl);
            self.shader.set_int(gl, "texture1", 0);
            self.shader.set_int(gl, "texture2", 1);

            gl.bind_buffer(ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.texture_1 = Some(texture_1);
            self.texture_2 = Some(texture_2);
        }
    }

    fn update(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, self.texture_1);

            gl.active_texture(TEXTURE1);
            gl.bind_texture(TEXTURE_2D, self.texture_2);

            gl.bind_vertex_array(self.vao);
            self.shader.use_shader(gl);

            let center = self.camera_pos + self.camera_front;
            // let view = glm::look_at(&self.camera_pos, &center, &CAMERA_UP);
            let view = calculate_look_at_matrix(&self.camera_pos, &center, &CAMERA_UP);

            let projection = glm::perspective(
                ctx.width as f32 / ctx.height as f32,
                self.fov.to_radians(),
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
                gl.draw_arrays(
                    // mode, first, count
                    TRIANGLES, // mode
                    0,         // first
                    36,        // count
                );
            }
        }
    }

    fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        unsafe {
            let gl = &ctx.gl;
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn process_keyboard(&mut self, ctx: &GLContext, key: Key, is_pressed: bool) {
        if !is_pressed {
            return;
        }
        let camera_speed = 2.5f32 * ctx.delta_time;
        if key == Key::W {
            self.camera_pos += self.camera_front * camera_speed;
        } else if key == Key::S {
            self.camera_pos -= self.camera_front * camera_speed;
        } else if key == Key::A {
            self.camera_pos -=
                glm::normalize(&glm::cross(&self.camera_front, &CAMERA_UP)) * camera_speed;
        } else if key == Key::D {
            self.camera_pos +=
                glm::normalize(&glm::cross(&self.camera_front, &CAMERA_UP)) * camera_speed;
        }
    }

    fn process_mouse(&mut self, _ctx: &GLContext, event: MouseEvent) {
        // log::info!("Mouse event: {:?}", event);
        match event {
            MouseEvent::Move { x, y } => {
                if self.first_mouse {
                    self.last_x = x;
                    self.last_y = y;
                    self.first_mouse = false;
                }
                let x_offset = x - self.last_x;
                let y_offset = self.last_y - y; // reversed since y-coordinates go from bottom to top
                self.last_x = x;
                self.last_y = y;

                let sensitivity = 0.1; // change this value to your liking
                let x_offset = x_offset as f32 * sensitivity;
                let y_offset = y_offset as f32 * sensitivity;

                self.yaw += x_offset;
                self.pitch += y_offset;

                // make sure that when pitch is out of bounds, screen doesn't get flipped
                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                }
                if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }

                let front = glm::vec3(
                    self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
                    self.pitch.to_radians().sin(),
                    self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
                );
                self.camera_front = glm::normalize(&front);
            }
            MouseEvent::Wheel { y_offset } => {
                self.fov -= y_offset;

                if self.fov <= 1.0 {
                    self.fov = 1.0;
                }
                if self.fov >= 45.0 {
                    self.fov = 45.0;
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            self.shader.delete(gl);

            if let Some(vertex_array) = self.vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(buffer) = self.vbo {
                gl.delete_buffer(buffer);
            }

            if let Some(texture) = self.texture_1 {
                gl.delete_texture(texture);
            }

            if let Some(texture) = self.texture_2 {
                gl.delete_texture(texture);
            }
        }
    }
}

fn calculate_look_at_matrix(position: &glm::Vec3, target: &glm::Vec3, up: &glm::Vec3) -> glm::Mat4 {
    let zaxis = glm::normalize(&(position - target));
    let up = glm::normalize(up);
    let xaxis = glm::normalize(&glm::cross(&up, &zaxis));
    let yaxis = glm::cross(&zaxis, &xaxis);
    #[rustfmt::skip]
    let translation = glm::Mat4::new(
        1.0, 0.0, 0.0, -position.x,
        0.0, 1.0, 0.0, -position.y,
        0.0, 0.0, 1.0, -position.z,
        0.0, 0.0, 0.0, 1.0
    );
    #[rustfmt::skip]
    let rotation= glm::Mat4::new(
          xaxis.x, yaxis.x, zaxis.x, 0.0,
          xaxis.y, yaxis.y, zaxis.y, 0.0,
          xaxis.z, yaxis.z, zaxis.z, 0.0,
          0.0, 0.0, 0.0, 1.0
    );
    rotation * translation
}
