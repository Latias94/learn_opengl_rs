use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_5_1_1() {
    let init_info = WindowInitInfo::builder()
        .title("Advanced Lighting | Press B to enable blinn".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 48] = [
    // position         // normals      // texcoords           
    10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
    -10.0, -0.5,  10.0,  0.0, 1.0, 0.0,   0.0,  0.0,
    -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,

    10.0, -0.5,  10.0,  0.0, 1.0, 0.0,  10.0,  0.0,
    -10.0, -0.5, -10.0,  0.0, 1.0, 0.0,   0.0, 10.0,
    10.0, -0.5, -10.0,  0.0, 1.0, 0.0,  10.0, 10.0
];

const LIGHT_POS: glm::Vec3 = glm::Vec3::new(0.0, 0.0, 0.0);

struct App {
    plane_vao: VertexArray,
    plane_vbo: Buffer,
    floor_texture: texture::Texture,
    shader: MyShader,
    camera: Camera,

    use_blinn: bool,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        let shader = MyShader::new_from_source(
            gl,
            include_str!("./shaders/_1_1_advanced_lighting.vs"),
            include_str!("./shaders/_1_1_advanced_lighting.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);

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
        let stride = 8 * size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 3, FLOAT, false, stride, 3 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(2, 2, FLOAT, false, stride, 6 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(2);

        gl.bind_buffer(ARRAY_BUFFER, None);

        // load textures
        // -------------
        let floor_texture = resources::load_texture(gl, "textures/wood.png")
            .await
            .expect("Failed to load texture");

        Self {
            plane_vao,
            plane_vbo,
            floor_texture,
            shader,
            camera,
            use_blinn: false,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // draw objects
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
        // set light uniforms
        self.shader.set_vec3(gl, "viewPos", &self.camera.position);
        self.shader.set_vec3(gl, "lightPos", &LIGHT_POS);
        self.shader
            .set_int(gl, "blinn", if self.use_blinn { 1 } else { 0 });

        // floor
        gl.bind_vertex_array(Some(self.plane_vao));
        self.floor_texture.bind(gl, 0);
        gl.draw_arrays(TRIANGLES, 0, 6);

        #[cfg(not(feature = "egui-support"))]
        log::info!("Blinn: {}", self.use_blinn);
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
    fn ui(
        &mut self,
        state: &crate::window::AppState,
        _gl_ctx: &crate::window::GLContext,
        egui_ctx: &egui::Context,
    ) {
        egui::Window::new("Info").show(egui_ctx, |ui| {
            ui.label(format!("FPS: {:.1}", 1.0 / state.render_delta_time));
            ui.label("Press B to enable blinn");
            ui.label(format!("Blinn: {}", self.use_blinn));
        });
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);

        self.use_blinn = input.key_held(winit::keyboard::KeyCode::KeyB);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.plane_vao);
        gl.delete_buffer(self.plane_vbo);
    }
}
