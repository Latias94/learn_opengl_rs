use crate::model::Model;
use crate::resources;
use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use winit_input_helper::WinitInputHelper;

pub async fn main_3_1_1() {
    let init_info = WindowInitInfo::builder()
        .title("Model Loading".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {
    our_shader: MyShader,
    camera: crate::camera::Camera,
    model: Model,
}

impl Application for App {
    async fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        unsafe {
            gl.enable(DEPTH_TEST);
        }

        let our_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/_1_1_model_loading.vs"),
            include_str!("./shaders/_1_1_model_loading.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");

        let camera = crate::camera::Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        let model = resources::load_obj(gl, "objects/backpack/backpack.obj")
            .await
            .expect("Failed to load model");
        log::info!("Model loaded");

        Self {
            our_shader,
            camera,
            model,
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;

        unsafe {
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }

        self.our_shader.use_shader(gl);
        let projection = glm::perspective(
            ctx.width as f32 / ctx.height as f32,
            self.camera.zoom().to_radians(),
            0.1,
            100.0,
        );
        let view = self.camera.view_matrix();
        self.our_shader.set_mat4(gl, "projection", &projection);
        self.our_shader.set_mat4(gl, "view", &view);

        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::Vec3::zeros()); // translate it down so it's at the center of the scene
        model = glm::scale(&model, &glm::vec3(1.0, 1.0, 1.0)); // it's a bit too big for our scene, so scale it down
        self.our_shader.set_mat4(gl, "model", &model);

        self.model.draw(gl, &self.our_shader);
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
    }

    fn exit(&mut self, ctx: &GLContext) {
        let gl = &ctx.gl;

        self.our_shader.delete(gl);
        self.model.delete(gl);
    }
}