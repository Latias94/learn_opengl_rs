use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use crate::{resources, texture};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_4_4_1() {
    let init_info = WindowInitInfo::builder()
        .title("Face Culling Exercise 1".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

#[rustfmt::skip]
const VERTICES: [f32; 180] = [
    // back face
    -0.5, -0.5, -0.5,  0.0, 0.0, // bottom-left
    0.5, -0.5, -0.5,  1.0, 0.0, // bottom-right    
    0.5,  0.5, -0.5,  1.0, 1.0, // top-right              
    0.5,  0.5, -0.5,  1.0, 1.0, // top-right
    -0.5,  0.5, -0.5,  0.0, 1.0, // top-left
    -0.5, -0.5, -0.5,  0.0, 0.0, // bottom-left                
    // front face
    -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left
    0.5,  0.5,  0.5,  1.0, 1.0, // top-right
    0.5, -0.5,  0.5,  1.0, 0.0, // bottom-right        
    0.5,  0.5,  0.5,  1.0, 1.0, // top-right
    -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left
    -0.5,  0.5,  0.5,  0.0, 1.0, // top-left        
    // left face
    -0.5,  0.5,  0.5,  1.0, 0.0, // top-right
    -0.5, -0.5, -0.5,  0.0, 1.0, // bottom-left
    -0.5,  0.5, -0.5,  1.0, 1.0, // top-left       
    -0.5, -0.5, -0.5,  0.0, 1.0, // bottom-left
    -0.5,  0.5,  0.5,  1.0, 0.0, // top-right
    -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-right
    // right face
    0.5,  0.5,  0.5,  1.0, 0.0, // top-left
    0.5,  0.5, -0.5,  1.0, 1.0, // top-right      
    0.5, -0.5, -0.5,  0.0, 1.0, // bottom-right          
    0.5, -0.5, -0.5,  0.0, 1.0, // bottom-right
    0.5, -0.5,  0.5,  0.0, 0.0, // bottom-left
    0.5,  0.5,  0.5,  1.0, 0.0, // top-left
    // bottom face          
    -0.5, -0.5, -0.5,  0.0, 1.0, // top-right
    0.5, -0.5,  0.5,  1.0, 0.0, // bottom-left
    0.5, -0.5, -0.5,  1.0, 1.0, // top-left        
    0.5, -0.5,  0.5,  1.0, 0.0, // bottom-left
    -0.5, -0.5, -0.5,  0.0, 1.0, // top-right
    -0.5, -0.5,  0.5,  0.0, 0.0, // bottom-right
    // top face
    -0.5,  0.5, -0.5,  0.0, 1.0, // top-left
    0.5,  0.5, -0.5,  1.0, 1.0, // top-right
    0.5,  0.5,  0.5,  1.0, 0.0, // bottom-right                 
    0.5,  0.5,  0.5,  1.0, 0.0, // bottom-right
    -0.5,  0.5,  0.5,  0.0, 0.0, // bottom-left  
    -0.5,  0.5, -0.5,  0.0, 1.0  // top-left
];

#[rustfmt::skip]
const PLANE_VERTICES: [f32; 30] = [
    // positions    texture Coords
    //  (note we set these higher than 1 (together with GL_REPEAT as texture wrapping mode). this will cause the floor texture to repeat)
    5.0, -0.5, 5.0, 2.0, 0.0,
    -5.0, -0.5, 5.0, 0.0, 0.0,
    -5.0, -0.5, -5.0, 0.0, 2.0,

    5.0, -0.5, 5.0, 2.0, 0.0,
    -5.0, -0.5, -5.0, 0.0, 2.0,
    5.0, -0.5, -5.0, 2.0, 2.0
];

#[rustfmt::skip]
const TRANSPARENT_VERTICES: [f32; 30] = [
    // positions  // texture Coords
    0.0, 0.5, 0.0, 0.0, 1.0,
    0.0, -0.5, 0.0, 0.0, 0.0,
    1.0, -0.5, 0.0, 1.0, 0.0,

    0.0, 0.5, 0.0, 0.0, 1.0,
    1.0, -0.5, 0.0, 1.0, 0.0,
    1.0, 0.5, 0.0 , 1.0, 1.0
];

const WINDOWS_POSITION: [glm::Vec3; 5] = [
    glm::Vec3::new(-1.5, 0.0, -0.48),
    glm::Vec3::new(1.5, 0.0, 0.51),
    glm::Vec3::new(0.0, 0.0, 0.7),
    glm::Vec3::new(-0.3, 0.0, -2.3),
    glm::Vec3::new(0.5, 0.0, -0.6),
];

struct App {
    cube_vbo: Buffer,
    cube_vao: VertexArray,
    cube_texture: texture::Texture,

    plane_vbo: Buffer,
    plane_vao: VertexArray,
    plane_texture: texture::Texture,

    transparent_vbo: Buffer,
    transparent_vao: VertexArray,
    transparent_texture: texture::Texture,

    shader: MyShader,
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

        let camera = Camera::new_with_position(glm::vec3(0.0, 0.0, 3.0));

        gl.enable(DEPTH_TEST);
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);

        gl.enable(CULL_FACE);
        gl.front_face(CW);

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

        // vegetation vao
        let transparent_vbo = gl.create_buffer().expect("Cannot create vbo buffer");
        gl.bind_buffer(ARRAY_BUFFER, Some(transparent_vbo));
        gl.buffer_data_u8_slice(
            ARRAY_BUFFER,
            bytemuck::cast_slice(&TRANSPARENT_VERTICES),
            STATIC_DRAW,
        );

        let transparent_vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(transparent_vao));
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

        // load texture
        let cube_texture = resources::load_texture(gl, "textures/marble.jpg")
            .await
            .expect("Failed to load texture");
        let plane_texture = resources::load_texture(gl, "textures/metal.png")
            .await
            .expect("Failed to load texture");
        let transparent_texture =
            resources::load_texture(gl, "textures/blending_transparent_window.png")
                .await
                .expect("Failed to load texture");
        transparent_texture.set_wrap_mode(gl, CLAMP_TO_EDGE as i32, CLAMP_TO_EDGE as i32);

        shader.use_shader(gl);
        shader.set_int(gl, "texture1", 0);

        Self {
            cube_vbo,
            cube_vao,
            cube_texture,
            plane_vbo,
            plane_vao,
            plane_texture,
            transparent_vbo,
            transparent_vao,
            transparent_texture,
            shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        let mut distances: Vec<(f32, glm::Vec3)> = WINDOWS_POSITION
            .iter()
            .map(|&position| {
                let distance = glm::distance(&self.camera.position(), &position);
                (distance, position)
            })
            .collect();
        distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Greater));

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

        // vegetation
        gl.bind_vertex_array(Some(self.transparent_vao));
        self.transparent_texture.bind(gl, 0);

        for &(_, position) in distances.iter() {
            let model = glm::translate(&glm::Mat4::identity(), &position);
            self.shader.set_mat4(gl, "model", &model);
            gl.draw_arrays(TRIANGLES, 0, 6);
        }
        // for &position in WINDOWS_POSITION.iter() {
        //     let model = glm::translate(&glm::Mat4::identity(), &position);
        //     self.shader.set_mat4(gl, "model", &model);
        //     gl.draw_arrays(TRIANGLES, 0, 6);
        // }

        gl.bind_vertex_array(None);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.shader.delete(gl);
        unsafe {
            gl.delete_buffer(self.cube_vbo);
            gl.delete_vertex_array(self.cube_vao);
            self.cube_texture.delete(gl);

            gl.delete_buffer(self.plane_vbo);
            gl.delete_vertex_array(self.plane_vao);
            self.plane_texture.delete(gl);

            gl.delete_buffer(self.transparent_vbo);
            gl.delete_vertex_array(self.transparent_vao);
            self.transparent_texture.delete(gl);
        }
    }
}
