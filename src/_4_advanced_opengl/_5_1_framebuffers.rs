use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_5_1() {
    let init_info = WindowInitInfo::builder()
        .title("Framebuffers".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const VERTICES: [f32; 180] = [
    // pos           // texture Coords
    -0.5, -0.5, -0.5,  0.0, 0.0,
    0.5, -0.5, -0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5,  0.5,  0.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  1.0, 1.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0
];

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 30] = [
    // positions    texture Coords
    5.0, -0.5,  5.0,  2.0, 0.0,
    -5.0, -0.5,  5.0,  0.0, 0.0,
    -5.0, -0.5, -5.0,  0.0, 2.0,

    5.0, -0.5,  5.0,  2.0, 0.0,
    -5.0, -0.5, -5.0,  0.0, 2.0,
    5.0, -0.5, -5.0,  2.0, 2.0
];

#[rustfmt::skip]
const QUAD_VERTICES: [f32; 24] = [
    // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
    // positions    texture Coords
    -1.0,  1.0,  0.0, 1.0,
    -1.0, -1.0,  0.0, 0.0,
    1.0, -1.0,  1.0, 0.0,

    -1.0,  1.0,  0.0, 1.0,
    1.0, -1.0,  1.0, 0.0,
    1.0,  1.0,  1.0, 1.0
];

struct App {
    cube_vbo: Buffer,
    cube_vao: VertexArray,
    cube_texture: texture::Texture,

    plane_vbo: Buffer,
    plane_vao: VertexArray,
    plane_texture: texture::Texture,

    quad_vbo: Buffer,
    quad_vao: VertexArray,

    framebuffer: Framebuffer,
    texture_color_buffer: Texture,
    rbo: Renderbuffer,

    shader: MyShader,
    screen_shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();

        let shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_1_1_depth_testing.vs"),
            include_str!("shaders/_1_1_depth_testing.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let screen_shader = MyShader::new_from_source(
            gl,
            include_str!("shaders/_5_1_framebuffers_screen.vs"),
            include_str!("shaders/_5_1_framebuffers_screen.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);

        //  cube vao
        let cube_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(cube_vbo));
        gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

        let cube_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(cube_vao));
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);
        gl.bind_vertex_array(None);

        // plane vao
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
        gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);

        // screen quad vao
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
        gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 4 * size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            FLOAT,
            false,
            4 * size_of::<f32>() as i32,
            (2 * size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);

        // load texture
        let cube_texture = resources::load_texture(gl, "textures/container.jpg")
            .await
            .expect("Failed to load texture");
        let plane_texture = resources::load_texture(gl, "textures/metal.png")
            .await
            .expect("Failed to load texture");

        shader.use_shader(gl);
        shader.set_int(gl, "texture1", 0);

        screen_shader.use_shader(gl);
        screen_shader.set_int(gl, "screenTexture", 0);

        // framebuffer configuration
        // -------------------------
        let framebuffer = gl.create_framebuffer().expect("Create framebuffer");
        gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
        // create a color attachment texture
        let texture_color_buffer = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture_color_buffer));
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGB as i32,
            ctx.width() as i32,
            ctx.height() as i32,
            0,
            RGB,
            UNSIGNED_BYTE,
            None,
        );
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
        gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
        gl.framebuffer_texture_2d(
            FRAMEBUFFER,
            COLOR_ATTACHMENT0,
            TEXTURE_2D,
            Some(texture_color_buffer),
            0,
        );
        // create a renderbuffer object for depth and stencil attachment (we won't be sampling these)
        let rbo = gl.create_renderbuffer().expect("Create renderbuffer");
        gl.bind_renderbuffer(RENDERBUFFER, Some(rbo));
        // use a single renderbuffer object for both a depth AND stencil buffer.
        gl.renderbuffer_storage(
            RENDERBUFFER,
            DEPTH24_STENCIL8,
            ctx.width() as i32,
            ctx.height() as i32,
        );
        // now actually attach it
        gl.framebuffer_renderbuffer(
            FRAMEBUFFER,
            DEPTH_STENCIL_ATTACHMENT,
            RENDERBUFFER,
            Some(rbo),
        );
        // check if framebuffer is complete
        if gl.check_framebuffer_status(FRAMEBUFFER) != FRAMEBUFFER_COMPLETE {
            log::error!("Framebuffer is not complete!");
        }
        gl.bind_framebuffer(FRAMEBUFFER, None);

        // draw as wireframe
        // gl.polygon_mode(FRONT_AND_BACK, LINE);

        Self {
            cube_vbo,
            cube_vao,
            cube_texture,
            plane_vbo,
            plane_vao,
            plane_texture,
            quad_vbo,
            quad_vao,
            framebuffer,
            texture_color_buffer,
            rbo,
            shader,
            screen_shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        // render
        // ------
        // bind to framebuffer and draw scene as we normally would to color texture
        gl.bind_framebuffer(FRAMEBUFFER, Some(self.framebuffer));
        gl.enable(DEPTH_TEST); // enable depth testing (is disabled for rendering screen-space quad)

        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

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

        // cubes
        gl.bind_vertex_array(Some(self.cube_vao));
        self.cube_texture.bind(gl, 0);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(-1.0, 0.0, -1.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(2.0, 0.0, 0.0));
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 36);

        // plane
        gl.bind_vertex_array(Some(self.plane_vao));
        self.plane_texture.bind(gl, 0);
        let model = glm::Mat4::identity();
        self.shader.set_mat4(gl, "model", &model);
        gl.draw_arrays(TRIANGLES, 0, 6);

        // now bind back to default framebuffer and draw a quad plane with the attached framebuffer color texture
        gl.bind_framebuffer(FRAMEBUFFER, None);
        // disable depth test so screen-space quad isn't discarded due to depth test.
        gl.disable(DEPTH_TEST);
        // clear all relevant buffers
        // set clear color to white (not really necessary actually, since we won't be able to see behind the quad anyways)
        gl.clear_color(1.0, 1.0, 1.0, 1.0);
        gl.clear(COLOR_BUFFER_BIT);

        self.screen_shader.use_shader(gl);
        gl.bind_vertex_array(Some(self.quad_vao));
        // use the color attachment texture as the texture of the quad plane
        gl.bind_texture(TEXTURE_2D, Some(self.texture_color_buffer));
        gl.draw_arrays(TRIANGLES, 0, 6);
        gl.bind_vertex_array(None);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);

        gl.delete_buffer(self.cube_vbo);
        gl.delete_vertex_array(self.cube_vao);
        self.cube_texture.delete(gl);

        gl.delete_buffer(self.plane_vbo);
        gl.delete_vertex_array(self.plane_vao);
        self.plane_texture.delete(gl);

        gl.delete_buffer(self.quad_vbo);
        gl.delete_vertex_array(self.quad_vao);

        gl.delete_framebuffer(self.framebuffer);
        gl.delete_texture(self.texture_color_buffer);
        gl.delete_renderbuffer(self.rbo);
    }
}
