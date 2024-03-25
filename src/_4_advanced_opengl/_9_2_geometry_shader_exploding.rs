use crate::camera::Camera;
use crate::model::Model;
use crate::resources;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_9_2() {
    let init_info = WindowInitInfo::builder()
        .title("Geometry Shader Exploding".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {
    model: Model,
    shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_with_geometry_from_source(
            gl,
            include_str!("shaders/_9_2_geometry_shader.vs"),
            include_str!("shaders/_9_2_geometry_shader.fs"),
            include_str!("shaders/_9_2_geometry_shader.gs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.5, 3.0));

        gl.enable(DEPTH_TEST);

        let model = resources::load_obj(gl, "objects/nanosuit/nanosuit.obj")
            .await
            .expect("Failed to load model");

        Self {
            model,
            shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
            45.0f32.to_radians(),
            0.1,
            100.0,
        );
        let view = self.camera.view_matrix();
        self.shader.use_shader(gl);
        self.shader.set_mat4(gl, "projection", &projection);
        self.shader.set_mat4(gl, "view", &view);
        let scale = 0.1f32;
        let model = glm::scale(&glm::Mat4::identity(), &glm::vec3(scale, scale, scale));
        self.shader.set_mat4(gl, "model", &model);

        // we have scaled model, so we need to scale explode magnitude as well
        self.shader.set_float(gl, "scale", scale);
        self.shader.set_float(gl, "time", ctx.elapsed_time_secs());

        self.model.draw(gl, &self.shader);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);
        self.model.delete(gl);
    }
}
