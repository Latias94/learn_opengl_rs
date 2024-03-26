use crate::camera::Camera;
use crate::model::Model;
use crate::resources;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use rand::Rng;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_10_2() {
    let init_info = WindowInitInfo::builder()
        .title("Asteroids".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {
    rock: Model,
    planet: Model,
    model_matrices: Vec<glm::Mat4>,

    shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_10_2_instancing.vs"),
            include_str!("shaders/_10_2_instancing.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 55.0));

        gl.enable(DEPTH_TEST);

        // load models
        // -----------
        let rock = resources::load_obj(gl, "objects/rock/rock.obj")
            .await
            .expect("Failed to load model");
        let planet = resources::load_obj(gl, "objects/planet/planet.obj")
            .await
            .expect("Failed to load model");

        // generate a large list of semi-random model transformation matrices
        // ------------------------------------------------------------------
        let amount = 2000;
        let model_matrices = generate_matrices(amount);

        Self {
            rock,
            planet,
            model_matrices,
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
            1000.0, // change far plane to 1000.0
        );
        let view = self.camera.view_matrix();
        self.shader.use_shader(gl);
        self.shader.set_mat4(gl, "projection", &projection);
        self.shader.set_mat4(gl, "view", &view);

        // draw planet
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, -3.0, 0.0));
        model = glm::scale(&model, &glm::vec3(4.0, 4.0, 4.0));
        self.shader.set_mat4(gl, "model", &model);
        self.planet.draw(gl, &self.shader);

        // draw meteorites
        for model in &self.model_matrices {
            self.shader.set_mat4(gl, "model", model);
            self.rock.draw(gl, &self.shader);
        }

        gl.bind_vertex_array(None);
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
            // slider to control asteroid count
            let mut amount = self.model_matrices.len() as f32;
            ui.add(
                egui::Slider::new(&mut amount, 1000.0..=10000.0)
                    .text("Asteroid count")
                    .step_by(1000.0),
            );
            if amount != self.model_matrices.len() as f32 {
                self.model_matrices = generate_matrices(amount as usize);
            }
        });
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        self.rock.delete(gl);
        self.planet.delete(gl);
    }
}

pub fn generate_matrices(amount: usize) -> Vec<glm::Mat4> {
    let mut model_matrices = Vec::with_capacity(amount);
    let radius = 50.0;
    let offset = 2.5;
    // initialize random seed
    let mut rng = rand::thread_rng();

    for i in 0..amount {
        let mut model = glm::Mat4::identity();
        // 1. translation: displace along circle with 'radius' in range [-offset, offset]
        let angle = (i as f32) / (amount as f32) * 360.0;
        let displacement = (rng.gen_range(0..(2 * offset as i32 * 100)) as f32) / 100.0 - offset;
        let x = angle.to_radians().sin() * radius + displacement;
        let displacement = (rng.gen_range(0..(2 * offset as i32 * 100)) as f32) / 100.0 - offset;
        let y = displacement * 0.4; // keep height of asteroid field smaller compared to width of x and z
        let displacement = (rng.gen_range(0..(2 * offset as i32 * 100)) as f32) / 100.0 - offset;
        let z = angle.to_radians().cos() * radius + displacement;
        model = glm::translate(&model, &glm::vec3(x, y, z));

        // 2. scale: Scale between 0.05 and 0.25f
        let scale = (rng.gen_range(0..20) as f32) / 100.0 + 0.05;
        model = glm::scale(&model, &glm::vec3(scale, scale, scale));

        // 3. rotation: add random rotation around a (semi)randomly picked rotation axis vector
        let rot_angle = (rng.gen_range(0..360) as f32).to_radians();
        model = glm::rotate(&model, rot_angle, &glm::vec3(0.4, 0.6, 0.8));

        // 4. now add to list of matrices
        model_matrices.push(model);
    }

    model_matrices
}
