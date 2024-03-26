use crate::camera::Camera;
use crate::resources::load_binary;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use image::GenericImageView;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_5_2_1() {
    let init_info = WindowInitInfo::builder()
        .title("Gamma Correction | Press Space to enable gamma".to_string())
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

const LIGHT_POSITIONS: [glm::Vec3; 4] = [
    glm::Vec3::new(-3.0, 0.0, 0.0),
    glm::Vec3::new(-1.0, 0.0, 0.0),
    glm::Vec3::new(1.0, 0.0, 0.0),
    glm::Vec3::new(3.0, 0.0, 0.0),
];

const LIGHT_COLORS: [glm::Vec3; 4] = [
    glm::Vec3::new(0.25, 0.25, 0.25),
    glm::Vec3::new(0.50, 0.50, 0.50),
    glm::Vec3::new(0.75, 0.75, 0.75),
    glm::Vec3::new(1.00, 1.00, 1.00),
];

struct App {
    plane_vao: VertexArray,
    plane_vbo: Buffer,
    floor_texture: Texture,
    floor_texture_gamma_corrected: Texture,
    shader: MyShader,
    camera: Camera,

    gamma_enabled: bool,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("./shaders/_2_1_gamma_correction.vs"),
            include_str!("./shaders/_2_1_gamma_correction.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

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
        let floor_texture = load_texture(gl, "textures/wood.png", false)
            .await
            .expect("Failed to load texture");
        let floor_texture_gamma_corrected = load_texture(gl, "textures/wood.png", true)
            .await
            .expect("Failed to load texture");

        shader.use_shader(gl);
        shader.set_int(gl, "floorTexture", 0);

        Self {
            plane_vao,
            plane_vbo,
            floor_texture,
            floor_texture_gamma_corrected,
            shader,
            camera,
            gamma_enabled: false,
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
        let program = self.shader.program();
        let light_positions_loc = gl.get_uniform_location(program, "lightPositions");
        let light_colors_loc = gl.get_uniform_location(program, "lightColors");
        gl.uniform_3_f32_slice(
            light_positions_loc.as_ref(),
            bytemuck::cast_slice(&LIGHT_POSITIONS),
        );
        gl.uniform_3_f32_slice(
            light_colors_loc.as_ref(),
            bytemuck::cast_slice(&LIGHT_COLORS),
        );

        self.shader.set_vec3(gl, "viewPos", &self.camera.position);
        self.shader
            .set_int(gl, "gamma", if self.gamma_enabled { 1 } else { 0 });

        // floor
        gl.bind_vertex_array(Some(self.plane_vao));
        gl.active_texture(TEXTURE0);
        gl.bind_texture(
            TEXTURE_2D,
            Some(if self.gamma_enabled {
                self.floor_texture_gamma_corrected
            } else {
                self.floor_texture
            }),
        );
        gl.draw_arrays(TRIANGLES, 0, 6);

        #[cfg(not(feature = "egui-support"))]
        log::info!("Gamma Enabled: {}", self.gamma_enabled);
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
            ui.label("Press Space to enable gamma");
            ui.label(format!("Gamma Enabled: {}", self.gamma_enabled));
        });
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);

        self.gamma_enabled = input.key_held(winit::keyboard::KeyCode::Space);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_vertex_array(self.plane_vao);
        gl.delete_buffer(self.plane_vbo);
    }
}

async fn load_texture(
    gl: &Context,
    file_name: &str,
    gamma_correction: bool,
) -> anyhow::Result<Texture> {
    log::info!("Loading texture file_name: {}", file_name);
    let bytes = load_binary(file_name).await?;
    let img = image::load_from_memory(&bytes).expect("Failed to load texture from bytes");

    let (width, height) = img.dimensions();
    let data = img.into_rgba8();
    let internal_format = if gamma_correction { SRGB8_ALPHA8 } else { RGBA };
    let data_format = RGBA;

    let texture = unsafe {
        let texture = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            internal_format as i32,
            width as i32,
            height as i32,
            0,
            data_format,
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
