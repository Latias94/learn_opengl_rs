use crate::camera::Camera;
use crate::model::Model;
use crate::resources;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use chrono::Utc;
use glow::*;
use nalgebra_glm as glm;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_3_1_1() {
    let init_info = WindowInitInfo::builder()
        .title("Model Loading".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {
    our_shader: MyShader,
    camera: Camera,
    model: Model,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        gl.enable(DEPTH_TEST);

        let our_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/_1_1_model_loading.vs"),
            include_str!("./shaders/_1_1_model_loading.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        let start = Utc::now();
        let model = resources::load_obj(gl, "objects/backpack/backpack.obj")
            .await
            .expect("Failed to load model");
        let end = Utc::now();
        log::info!("Model loaded in {} ms", (end - start).num_milliseconds());
        #[cfg(debug_assertions)]
        log::info!("It is better to run this demo in release mode: `just rrun 3_1_1`");
        // TODO: Vertex deduplication

        Self {
            our_shader,
            camera,
            model,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        self.our_shader.use_shader(gl);
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
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

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.our_shader.delete(gl);
        self.model.delete(gl);
    }
}
