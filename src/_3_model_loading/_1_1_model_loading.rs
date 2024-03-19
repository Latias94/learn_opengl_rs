use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use anyhow::Result;
use glow::*;
use nalgebra_glm as glm;
use winit_input_helper::WinitInputHelper;

pub fn main_3_1_1() {
    let init_info = WindowInitInfo::builder()
        .title("Model Loading".to_string())
        .build();
    todo!();
    unsafe {
        run::<App>(init_info);
    }
}

struct App {
    cube_vao: Option<VertexArray>,
    light_vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    diffuse_map: Option<Texture>,
    specular_map: Option<Texture>,
    our_shader: MyShader,
    camera: crate::camera::Camera,
    current_style: u32,
}

impl Application for App {
    fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let our_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/_1_1_model_loading.vs"),
            include_str!("./shaders/_1_1_model_loading.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");

        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera = crate::camera::Camera::new_with_position(camera_pos);
        Self {
            cube_vao: None,
            light_vao: None,
            vbo: None,
            diffuse_map: None,
            specular_map: None,
            our_shader,
            camera,
            current_style: 0,
        }
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;

            gl.enable(DEPTH_TEST);
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;

            self.our_shader.use_shader(gl);
        }
    }

    fn resize(&mut self, ctx: &GLContext, width: u32, height: u32) {
        unsafe {
            let gl = &ctx.gl;
            gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    fn process_input(&mut self, _ctx: &GLContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
        if input.key_pressed(winit::keyboard::KeyCode::KeyQ) {
            self.current_style = (self.current_style + 1) % 4;
        }
    }

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;
        unsafe {
            self.our_shader.delete(gl);

            if let Some(vertex_array) = self.cube_vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(vertex_array) = self.light_vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(buffer) = self.vbo {
                gl.delete_buffer(buffer);
            }

            if let Some(texture) = self.diffuse_map {
                gl.delete_texture(texture);
            }

            if let Some(texture) = self.specular_map {
                gl.delete_texture(texture);
            }
        }
    }
}

fn load_texture_from_bytes(gl: &Context, bytes: &[u8]) -> Result<Texture> {
    let img = image::load_from_memory(bytes)?.flipv().to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();
    let texture = unsafe {
        let texture = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGBA as i32,
            width as i32,
            height as i32,
            0,
            RGBA,
            UNSIGNED_BYTE,
            Some(&data),
        );
        gl.generate_mipmap(TEXTURE_2D);

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);

        texture
    };
    Ok(texture)
}
