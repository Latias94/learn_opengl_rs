use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use chrono::Utc;
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;

pub fn main_1_5_3() {
    let init_info = WindowInitInfo::builder()
        .title("Transformations Exercise 2".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
    }
}

// rectangle, pos tex_coord
const VERTICES: [f32; 20] = [
    // pos         tex_coord
    0.5, 0.5, 0.0, 1.0, 1.0, // upper left
    0.5, -0.5, 0.0, 1.0, 0.0, // lower left
    -0.5, -0.5, 0.0, 0.0, 0.0, // lower right
    -0.5, 0.5, 0.0, 0.0, 1.0, // upper right
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

struct App {
    vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    texture_1: Option<Texture>,
    texture_2: Option<Texture>,
    shader: MyShader,
    start: chrono::DateTime<Utc>,
}

impl Application for App {
    fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/5.1.transform.vs"),
            include_str!("./shaders/5.1.transform.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");
        Self {
            shader,
            vao: None,
            vbo: None,
            texture_1: None,
            texture_2: None,
            start: Utc::now(),
        }
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            let vbo = gl.create_buffer().expect("Cannot create vbo buffer");
            let ebo = gl.create_buffer().expect("Cannot create ebo buffer");

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&INDICES),
                STATIC_DRAW,
            );

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
            // set the texture wrapping parameters
            // set texture wrapping to GL_REPEAT (default wrapping method)
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
            // set texture filtering parameters
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

            // load image, create texture and generate mipmaps
            // webgl doesn't support loading image from file, so we use include_bytes! to load image
            // `flipv()` to flip loaded texture's on the y-axis.
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

            // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
            // -------------------------------------------------------------------------------------------
            self.shader.use_shader(gl);
            self.shader.set_int(gl, "texture1", 0);
            self.shader.set_int(gl, "texture2", 1);

            gl.bind_buffer(ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            gl.delete_buffer(ebo);

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
            gl.clear(COLOR_BUFFER_BIT);

            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, self.texture_1);

            gl.active_texture(TEXTURE1);
            gl.bind_texture(TEXTURE_2D, self.texture_2);

            // create translations
            let mut transform = glm::Mat4::identity();
            let now = Utc::now();
            let duration = now - self.start;
            let angle = duration.num_milliseconds() as f32 / 1000.0;
            transform = glm::translate(&transform, &glm::vec3(0.5, -0.5, 0.0));
            transform = glm::rotate(&transform, angle, &glm::Vec3::z());

            let transform_loc = gl.get_uniform_location(self.shader.program(), "transform");
            gl.uniform_matrix_4_f32_slice(transform_loc.as_ref(), false, transform.as_slice());

            gl.bind_vertex_array(self.vao);
            gl.draw_elements(
                // mode, count, type, indices
                TRIANGLES,    // mode
                6,            // count
                UNSIGNED_INT, // type
                0,            // indices
            );

            // second transformation
            transform = glm::Mat4::identity();
            transform = glm::translate(&transform, &glm::vec3(-0.5, 0.5, 0.0));
            let scale_amount = (duration.num_milliseconds() as f32 / 1000.0).sin() / 2.0 + 0.5;
            transform = glm::scale(
                &transform,
                &glm::vec3(scale_amount, scale_amount, scale_amount),
            );

            let transform_loc = gl.get_uniform_location(self.shader.program(), "transform");
            gl.uniform_matrix_4_f32_slice(transform_loc.as_ref(), false, transform.as_slice());

            gl.draw_elements(
                // mode, count, type, indices
                TRIANGLES,    // mode
                6,            // count
                UNSIGNED_INT, // type
                0,            // indices
            );
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
