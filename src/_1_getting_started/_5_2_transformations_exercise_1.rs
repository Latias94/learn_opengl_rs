use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;

pub async fn main_1_5_2() {
    let init_info = WindowInitInfo::builder()
        .title("Transformations Exercise 2".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
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
    vao: VertexArray,
    vbo: Buffer,
    texture_1: Texture,
    texture_2: Texture,
    shader: MyShader,
}

impl Application for App {
    async fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/5.1.transform.vs"),
            include_str!("./shaders/5.1.transform.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");

        unsafe {
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
            shader.use_shader(gl);
            shader.set_int(gl, "texture1", 0);
            shader.set_int(gl, "texture2", 1);

            gl.bind_buffer(ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            gl.delete_buffer(ebo);

            Self {
                shader,
                vao,
                vbo,
                texture_1,
                texture_2,
            }
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(COLOR_BUFFER_BIT);

            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, Some(self.texture_1));

            gl.active_texture(TEXTURE1);
            gl.bind_texture(TEXTURE_2D, Some(self.texture_2));

            // create translations
            let mut transform = glm::Mat4::identity();
            let angle = ctx.render_delta_time;
            transform = glm::rotate(&transform, angle, &glm::Vec3::z());
            transform = glm::translate(&transform, &glm::vec3(0.5, -0.5, 0.0));

            self.shader.use_shader(gl);

            let transform_loc = gl.get_uniform_location(self.shader.program(), "transform");
            gl.uniform_matrix_4_f32_slice(transform_loc.as_ref(), false, transform.as_slice());

            gl.bind_vertex_array(Some(self.vao));
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

            gl.delete_vertex_array(self.vao);

            gl.delete_buffer(self.vbo);

            gl.delete_texture(self.texture_1);

            gl.delete_texture(self.texture_2);
        }
    }
}
