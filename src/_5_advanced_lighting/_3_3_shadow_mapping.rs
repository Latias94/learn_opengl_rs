use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_5_3_3() {
    let init_info = WindowInitInfo::builder()
        .title("Shadow Mapping".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 48] = [
    // position         // normals      // texcoords           
    25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
    -25.0, -0.5,  25.0,  0.0, 1.0, 0.0,   0.0,  0.0,
    -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,

    25.0, -0.5,  25.0,  0.0, 1.0, 0.0,  25.0,  0.0,
    -25.0, -0.5, -25.0,  0.0, 1.0, 0.0,   0.0, 25.0,
    25.0, -0.5, -25.0,  0.0, 1.0, 0.0,  25.0, 25.0
];

#[rustfmt::skip]
const CUBE_VERTICES: [f32; 288] = [
    // back face
    -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
    1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
    1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 0.0, // bottom-right         
    1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 1.0, 1.0, // top-right
    -1.0, -1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 0.0, // bottom-left
    -1.0,  1.0, -1.0,  0.0,  0.0, -1.0, 0.0, 1.0, // top-left
    // front face
    -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
    1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 0.0, // bottom-right
    1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
    1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 1.0, 1.0, // top-right
    -1.0,  1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0, // top-left
    -1.0, -1.0,  1.0,  0.0,  0.0,  1.0, 0.0, 0.0, // bottom-left
    // left face
    -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
    -1.0,  1.0, -1.0, -1.0,  0.0,  0.0, 1.0, 1.0, // top-left
    -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
    -1.0, -1.0, -1.0, -1.0,  0.0,  0.0, 0.0, 1.0, // bottom-left
    -1.0, -1.0,  1.0, -1.0,  0.0,  0.0, 0.0, 0.0, // bottom-right
    -1.0,  1.0,  1.0, -1.0,  0.0,  0.0, 1.0, 0.0, // top-right
    // right face
    1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
    1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
    1.0,  1.0, -1.0,  1.0,  0.0,  0.0, 1.0, 1.0, // top-right         
    1.0, -1.0, -1.0,  1.0,  0.0,  0.0, 0.0, 1.0, // bottom-right
    1.0,  1.0,  1.0,  1.0,  0.0,  0.0, 1.0, 0.0, // top-left
    1.0, -1.0,  1.0,  1.0,  0.0,  0.0, 0.0, 0.0, // bottom-left     
    // bottom face
    -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
    1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 1.0, 1.0, // top-left
    1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
    1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 1.0, 0.0, // bottom-left
    -1.0, -1.0,  1.0,  0.0, -1.0,  0.0, 0.0, 0.0, // bottom-right
    -1.0, -1.0, -1.0,  0.0, -1.0,  0.0, 0.0, 1.0, // top-right
    // top face
    -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
    1.0,  1.0, 1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
    1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 1.0, 1.0, // top-right     
    1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 1.0, 0.0, // bottom-right
    -1.0,  1.0, -1.0,  0.0,  1.0,  0.0, 0.0, 1.0, // top-left
    -1.0,  1.0,  1.0,  0.0,  1.0,  0.0, 0.0, 0.0  // bottom-left
];

#[rustfmt::skip]
const QUAD_VERTICES: [f32; 20] = [
    // positions     // texture Coords
    -1.0,  1.0, 0.0, 0.0, 1.0,
    -1.0, -1.0, 0.0, 0.0, 0.0,
    1.0,  1.0, 0.0, 1.0, 1.0,
    1.0, -1.0, 0.0, 1.0, 0.0,
];

const LIGHT_POS: glm::Vec3 = glm::Vec3::new(-2.0, 4.0, -1.0);
const SHADOW_WIDTH: i32 = 1024;
const SHADOW_HEIGHT: i32 = 1024;

struct App {
    plane_vao: VertexArray,
    plane_vbo: Buffer,

    cube_vao: VertexArray,
    cube_vbo: Buffer,

    quad_vao: VertexArray,
    quad_vbo: Buffer,

    depth_map_fbo: Framebuffer,
    depth_map: Texture,
    wood_texture: texture::Texture,
    shader: MyShader,
    simple_depth_shader: MyShader,
    debug_depth_quad_shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("./shaders/_3_2_shadow_mapping.vs"),
            include_str!("./shaders/_3_3_shadow_mapping.fs"),
            Some(ctx.suggested_shader_version()),
        )
            .expect("Failed to create program");

        let simple_depth_shader = MyShader::new_from_source(
            gl,
            include_str!("./shaders/_3_1_shadow_mapping_depth.vs"),
            include_str!("./shaders/_3_1_shadow_mapping_depth.fs"),
            Some(ctx.suggested_shader_version()),
        )
            .expect("Failed to create program");

        let debug_depth_quad_shader = MyShader::new_from_source(
            gl,
            include_str!("./shaders/_3_1_debug_quad.vs"),
            include_str!("./shaders/_3_1_debug_quad_depth.fs"),
            Some(ctx.suggested_shader_version()),
        )
            .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

        // PLANE
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
        gl.bind_vertex_array(None);

        // CUBE
        let cube_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(cube_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&CUBE_VERTICES),
            STATIC_DRAW,
        );

        let cube_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(cube_vao));
        let stride = 8 * size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 3, FLOAT, false, stride, 3 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(2, 2, FLOAT, false, stride, 6 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(2);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        // QUAD
        let quad_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(quad_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&QUAD_VERTICES),
            STATIC_DRAW,
        );

        let quad_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(quad_vao));
        let stride = 5 * size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, stride, 3 * size_of::<f32>() as i32);
        gl.enable_vertex_attrib_array(1);

        gl.bind_buffer(ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        // load textures
        // -------------
        let wood_texture = resources::load_texture(gl, "textures/wood.png")
            .await
            .expect("Failed to load texture");

        // configure depth map FBO
        // -----------------------
        let depth_map_fbo = gl.create_framebuffer().expect("Create framebuffer");
        let depth_map = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(depth_map));
        // In WebGL2 DEPTH_COMPONENT is not a valid internal format. Use DEPTH_COMPONENT16, DEPTH_COMPONENT24 or DEPTH_COMPONENT32F
        // https://webgl2fundamentals.org/webgl/lessons/webgl-render-to-texture.html
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let internal_format = DEPTH_COMPONENT24;
            } else {
                let internal_format = DEPTH_COMPONENT;
            }
        }
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            internal_format as i32,
            SHADOW_WIDTH,
            SHADOW_HEIGHT,
            0,
            DEPTH_COMPONENT,
            UNSIGNED_INT, // WebGL2 throw error when I use FLOAT
            None,
        );

        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
        #[cfg(not(target_arch = "wasm32"))]
        {
            // glow-0.13.1\src\web_sys.rs:3828:9:
            // Texture parameters for `&[f32]` are not supported yet
            let border_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            gl.tex_parameter_f32_slice(TEXTURE_2D, TEXTURE_BORDER_COLOR, &border_color);
        }
        // attach depth texture as FBO's depth buffer
        gl.bind_framebuffer(FRAMEBUFFER, Some(depth_map_fbo));
        gl.framebuffer_texture_2d(
            FRAMEBUFFER,
            DEPTH_ATTACHMENT,
            TEXTURE_2D,
            Some(depth_map),
            0,
        );
        gl.draw_buffer(NONE);
        gl.read_buffer(NONE);
        gl.bind_framebuffer(FRAMEBUFFER, None);

        // shader configuration
        // --------------------
        shader.use_shader(gl);
        shader.set_int(gl, "diffuseTexture", 0);
        shader.set_int(gl, "shadowMap", 1);

        debug_depth_quad_shader.use_shader(gl);
        debug_depth_quad_shader.set_int(gl, "depthMap", 0);

        Self {
            plane_vao,
            plane_vbo,
            cube_vao,
            cube_vbo,
            quad_vao,
            quad_vbo,
            depth_map_fbo,
            depth_map,
            wood_texture,
            shader,
            simple_depth_shader,
            debug_depth_quad_shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // 1. render depth of scene to texture (from light's perspective)
        // --------------------------------------------------------------
        const NEAR_PLANE: f32 = 1.0;
        const FAR_PLANE: f32 = 7.5;
        let light_projection = glm::ortho(-10.0, 10.0, -10.0, 10.0, NEAR_PLANE, FAR_PLANE);
        let light_view = glm::look_at(&LIGHT_POS, &glm::Vec3::zeros(), &glm::vec3(0.0, 1.0, 0.0));
        let light_space_matrix = light_projection * light_view;
        // render scene from light's point of view
        self.simple_depth_shader.use_shader(gl);
        self.simple_depth_shader
            .set_mat4(gl, "lightSpaceMatrix", &light_space_matrix);

        gl.viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
        gl.bind_framebuffer(FRAMEBUFFER, Some(self.depth_map_fbo));
        gl.clear(DEPTH_BUFFER_BIT);
        self.wood_texture.bind(gl, 0);
        self.render_scene(gl, &self.simple_depth_shader);
        gl.bind_framebuffer(FRAMEBUFFER, None);

        // reset viewport
        gl.viewport(0, 0, ctx.width() as i32, ctx.height() as i32);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // 2. render scene as normal using the generated depth/shadow map
        // --------------------------------------------------------------
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
        self.shader.set_vec3(gl, "viewPos", &self.camera.position());
        self.shader.set_vec3(gl, "lightPos", &LIGHT_POS);
        self.shader
            .set_mat4(gl, "lightSpaceMatrix", &light_space_matrix);
        self.wood_texture.bind(gl, 0);
        gl.active_texture(TEXTURE1);
        gl.bind_texture(TEXTURE_2D, Some(self.depth_map));
        self.render_scene(gl, &self.shader);

        // render Depth map to quad for visual debugging
        // ---------------------------------------------
        self.debug_depth_quad_shader.use_shader(gl);
        self.debug_depth_quad_shader
            .try_set_float(gl, "near_plane", NEAR_PLANE);
        self.debug_depth_quad_shader
            .try_set_float(gl, "far_plane", FAR_PLANE);
        gl.active_texture(TEXTURE0);
        gl.bind_texture(TEXTURE_2D, Some(self.depth_map));
        // self.render_quad(gl);
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
        });
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.wood_texture.delete(gl);

        self.shader.delete(gl);
        self.simple_depth_shader.delete(gl);
        self.debug_depth_quad_shader.delete(gl);

        gl.delete_texture(self.depth_map);
        gl.delete_framebuffer(self.depth_map_fbo);

        gl.delete_vertex_array(self.plane_vao);
        gl.delete_buffer(self.plane_vbo);

        gl.delete_vertex_array(self.cube_vao);
        gl.delete_buffer(self.cube_vbo);

        gl.delete_vertex_array(self.quad_vao);
        gl.delete_buffer(self.quad_vbo);
    }
}

impl App {
    unsafe fn render_scene(&self, gl: &Context, shader: &MyShader) {
        // floor
        let model = glm::Mat4::identity();
        shader.set_mat4(gl, "model", &model);
        gl.bind_vertex_array(Some(self.plane_vao));
        gl.draw_arrays(TRIANGLES, 0, 6);
        // cubes
        let mut model = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 1.5, 0.0));
        model = glm::scale(&model, &glm::vec3(0.5, 0.5, 0.5));
        shader.set_mat4(gl, "model", &model);
        self.render_cube(gl);

        model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 1.0));
        model = glm::scale(&model, &glm::vec3(0.5, 0.5, 0.5));
        shader.set_mat4(gl, "model", &model);
        self.render_cube(gl);

        model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, 2.0));
        model = glm::rotate(
            &model,
            60.0_f32.to_radians(),
            &glm::vec3(1.0, 0.0, 1.0).normalize(),
        );
        model = glm::scale(&model, &glm::vec3(0.25, 0.25, 0.25));
        shader.set_mat4(gl, "model", &model);
        self.render_cube(gl);
    }

    // render_cube() renders a 1x1 3D cube in NDC.
    // -------------------------------------------------
    unsafe fn render_cube(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.cube_vao));
        gl.draw_arrays(TRIANGLES, 0, 36);
        gl.bind_vertex_array(None);
    }

    // render_quad() renders a 1x1 XY quad in NDC
    // -----------------------------------------
    #[allow(dead_code)]
    unsafe fn render_quad(&self, gl: &Context) {
        gl.bind_vertex_array(Some(self.quad_vao));
        gl.draw_arrays(TRIANGLE_STRIP, 0, 4);
        gl.bind_vertex_array(None);
    }
}
