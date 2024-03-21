use crate::shader::MyShader;
use crate::window::{run, AppContext, AppState, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use std::mem::size_of;
use std::time::Duration;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_9_1_1() {
    let init_info = WindowInitInfo::builder().title("egui".to_string()).build();
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
    texture_1: texture::Texture,
    texture_2: texture::Texture,
    shader: MyShader,
    mix_value: f32,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        #[cfg(target_arch = "wasm32")]
        log::info!("Not implemented for web yet.");

        let gl = ctx.gl();
        let shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/4.1.texture.vs"),
            include_str!("shaders/4.6.texture.fs"),
            Some(ctx.suggested_shader_version()),
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

        // texture 1
        // ---------
        let texture_1 = resources::load_texture(gl, "textures/container.jpg")
            .await
            .expect("Failed to load image");

        // texture 2
        // ---------
        let texture_2 = resources::load_texture(gl, "textures/awesomeface.png")
            .await
            .expect("Failed to load image");

        // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
        // -------------------------------------------------------------------------------------------
        shader.use_shader(gl);
        // either set it manually like so:
        let location = gl
            .get_uniform_location(shader.program(), "texture1")
            .unwrap();
        gl.uniform_1_i32(Some(&location), 0);
        // or set it via the texture class
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
            mix_value: 0.2,
        }
    }

    fn ui(&mut self, _state: &AppState, egui_ctx: &egui::Context) {
        egui::Window::new("Hello world").show(egui_ctx, |ui| {
            ui.label("Hello World!");
            // change mix value
            ui.add(egui::Slider::new(&mut self.mix_value, 0.0..=1.0).text("mix value"));
        });
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.2, 0.3, 0.3, 1.0);
        gl.clear(COLOR_BUFFER_BIT);

        self.texture_1.bind(gl, 0);
        self.texture_2.bind(gl, 1);

        self.shader.use_shader(gl);
        self.shader.set_float(gl, "mixValue", self.mix_value);

        // seeing as we only have a single VAO there's no need to bind it every time,
        // but we'll do so to keep things a bit more organized
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_elements(TRIANGLES, 6, UNSIGNED_INT, 0);
    }

    unsafe fn resize(&mut self, ctx: &AppContext, width: u32, height: u32) {
        let gl = ctx.gl();
        gl.viewport(0, 0, width as i32, height as i32);
    }
    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        let delta_time = input.delta_time().unwrap_or(Duration::new(0, 0));
        let delta_time = delta_time.as_secs_f32();
        if input.key_held(KeyCode::ArrowUp) || input.key_held(KeyCode::KeyW) {
            self.mix_value += 0.5 * delta_time;
            if self.mix_value > 1.0 {
                self.mix_value = 1.0;
            }
        } else if input.key_held(KeyCode::ArrowDown) || input.key_held(KeyCode::KeyS) {
            self.mix_value -= 0.5 * delta_time;
            if self.mix_value < 0.0 {
                self.mix_value = 0.0;
            }
        }
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo);

        self.texture_1.delete(gl);
        self.texture_2.delete(gl);
    }
}
