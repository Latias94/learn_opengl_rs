use crate::camera::Camera;
use crate::model::Model;
use crate::resources;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use rand::Rng;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_10_3() {
    let init_info = WindowInitInfo::builder()
        .title("Asteroids Instanced".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

struct App {
    rock: Model,
    planet: Model,
    model_matrices: Vec<glm::Mat4>,
    buffer: Buffer,

    asteroid_shader: MyShader,
    planet_shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let asteroid_shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_10_3_asteroids.vs"),
            include_str!("shaders/_10_3_asteroids.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let planet_shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_10_3_planet.vs"),
            include_str!("shaders/_10_3_planet.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let mut camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 155.0));
        camera.set_speed(25.0);

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
        let amount = 100000;
        let model_matrices = generate_matrices(amount);
        let buffer = create_buffer(gl, &rock, &model_matrices);

        Self {
            rock,
            planet,
            model_matrices,
            buffer,
            asteroid_shader,
            planet_shader,
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
        self.asteroid_shader.use_shader(gl);
        self.asteroid_shader.set_mat4(gl, "projection", &projection);
        self.asteroid_shader.set_mat4(gl, "view", &view);
        self.planet_shader.use_shader(gl);
        self.planet_shader.set_mat4(gl, "projection", &projection);
        self.planet_shader.set_mat4(gl, "view", &view);

        // draw planet
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::vec3(0.0, -3.0, 0.0));
        model = glm::scale(&model, &glm::vec3(4.0, 4.0, 4.0));
        self.planet_shader.set_mat4(gl, "model", &model);
        self.planet.draw(gl, &self.planet_shader);

        // draw meteorites
        self.asteroid_shader.use_shader(gl);
        gl.active_texture(TEXTURE0);
        self.asteroid_shader.set_int(gl, "texture_diffuse1", 0);
        gl.bind_texture(TEXTURE_2D, Some(self.rock.materials[0].textures[0].raw()));
        for mesh in &self.rock.meshes {
            gl.bind_vertex_array(Some(mesh.vao));
            gl.draw_elements_instanced(
                TRIANGLES,
                mesh.indices.len() as i32,
                UNSIGNED_INT,
                0,
                self.model_matrices.len() as i32,
            );
            gl.bind_vertex_array(None);
        }
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
    fn ui(
        &mut self,
        state: &crate::window::AppState,
        gl_ctx: &crate::window::GLContext,
        egui_ctx: &egui::Context,
    ) {
        // show fps by default
        egui::Window::new("Info").show(egui_ctx, |ui| {
            ui.label(format!("FPS: {:.1}", 1.0 / state.render_delta_time));
            // slider to control asteroid count
            let mut amount = self.model_matrices.len() as f32;
            ui.add(
                egui::Slider::new(&mut amount, 100000.0..=1000000.0)
                    .text("Asteroid count")
                    .step_by(100000.0),
            );
            if amount != self.model_matrices.len() as f32 {
                let gl = &gl_ctx.gl;
                unsafe {
                    gl.delete_buffer(self.buffer);
                }
                self.model_matrices = generate_matrices(amount as usize);
                self.buffer = unsafe { create_buffer(gl, &self.rock, &self.model_matrices) };
            }
        });
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.asteroid_shader.delete(gl);
        self.planet_shader.delete(gl);
        gl.delete_buffer(self.buffer);

        self.rock.delete(gl);
        self.planet.delete(gl);
    }
}

unsafe fn create_buffer(gl: &Context, rock: &Model, model_matrices: &[glm::Mat4]) -> Buffer {
    // configure instanced array
    // -------------------------
    let buffer = gl.create_buffer().expect("Create buffer failed");
    gl.bind_buffer(ARRAY_BUFFER, Some(buffer));
    gl.buffer_data_u8_slice(
        ARRAY_BUFFER,
        bytemuck::cast_slice(model_matrices),
        STATIC_DRAW,
    );

    // set transformation matrices as an instance vertex attribute (with divisor 1)
    // note: we're cheating a little by taking the, now publicly declared, VAO of the model's mesh(es) and adding new vertexAttribPointers
    // normally you'd want to do this in a more organized fashion, but for learning purposes this will do.
    // -----------------------------------------------------------------------------------------------------------------------------------
    for mesh in &rock.meshes {
        let vao = mesh.vao;
        gl.bind_vertex_array(Some(vao));
        // 4 times vec4
        let stride = std::mem::size_of::<glm::Mat4>() as i32;
        let vec4_size = std::mem::size_of::<glm::Vec4>() as i32;
        // set attribute pointers for matrix (4 times vec4)
        // --------------------------------------------------
        // first column
        gl.vertex_attrib_pointer_f32(3, 4, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(3);
        // second column
        gl.vertex_attrib_pointer_f32(4, 4, FLOAT, false, stride, vec4_size);
        gl.enable_vertex_attrib_array(4);
        // third column
        gl.vertex_attrib_pointer_f32(5, 4, FLOAT, false, stride, 2 * vec4_size);
        gl.enable_vertex_attrib_array(5);
        // fourth column
        gl.vertex_attrib_pointer_f32(6, 4, FLOAT, false, stride, 3 * vec4_size);
        gl.enable_vertex_attrib_array(6);

        gl.vertex_attrib_divisor(3, 1);
        gl.vertex_attrib_divisor(4, 1);
        gl.vertex_attrib_divisor(5, 1);
        gl.vertex_attrib_divisor(6, 1);

        gl.bind_vertex_array(None);
    }
    buffer
}

pub fn generate_matrices(amount: usize) -> Vec<glm::Mat4> {
    let mut model_matrices = Vec::with_capacity(amount);
    let radius = 150.0;
    let offset = 25.0;
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
