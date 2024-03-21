use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use image::GenericImageView;
use std::mem::size_of;

pub async unsafe fn main_1_4_1() {
    let init_info = WindowInitInfo::builder()
        .title("Textures".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

// rectangle, pos color tex_coord
const VERTICES: [f32; 32] = [
    // pos           color        tex_coord
    0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // upper left
    0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // lower left
    -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // lower right
    -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // upper right
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

struct App {
    vao: VertexArray,
    vbo: Buffer,
    texture: Texture,
    shader: MyShader,
}

impl Application for App {
    async unsafe fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/4.1.texture.vs"),
            include_str!("./shaders/4.1.texture.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");

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

        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 8 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_f32(
            1,
            3,
            FLOAT,
            false,
            8 * size_of::<f32>() as i32,
            3 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(1);

        gl.vertex_attrib_pointer_f32(
            2,
            2,
            FLOAT,
            false,
            8 * size_of::<f32>() as i32,
            6 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(2);

        // load texture
        let texture = gl.create_texture().expect("Cannot create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture));
        // set the texture wrapping parameters
        // set texture wrapping to GL_REPEAT (default wrapping method)
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        // set texture filtering parameters
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        // load image, create texture and generate mipmaps
        // webgl doesn't support loading image from file, so we use include_bytes! to load image
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

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        gl.delete_buffer(ebo);

        Self {
            shader,
            vao,
            vbo,
            texture,
        }
    }

    unsafe fn render(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        gl.clear(COLOR_BUFFER_BIT);

        gl.bind_texture(TEXTURE_2D, Some(self.texture));

        self.shader.use_shader(gl);

        // seeing as we only have a single VAO there's no need to bind it every time,
        // but we'll do so to keep things a bit more organized
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
    }

    unsafe fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        let gl = &ctx.gl;
        gl.viewport(0, 0, width as i32, height as i32);
    }

    unsafe fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;

        self.shader.delete(gl);

        gl.delete_vertex_array(self.vao);

        gl.delete_buffer(self.vbo);

        gl.delete_texture(self.texture);
    }
}
