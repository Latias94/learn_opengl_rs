use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub fn main_2_2_3() {
    let init_info = WindowInitInfo::builder()
        .title("Basic Lighting Exercise 1".to_string())
        .build();
    unsafe {
        run::<App>(init_info);
    }
}

#[rustfmt::skip]
const VERTICES: [f32; 216] = [
    // pos           normal
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
    0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
    0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
    0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
    0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
    0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
];

const CAMERA_UP: glm::Vec3 = glm::Vec3::new(0.0, 1.0, 0.0);

struct App {
    cube_vao: Option<VertexArray>,
    light_vao: Option<VertexArray>,
    vbo: Option<Buffer>,
    lighting_shader: MyShader,
    lighting_cube_shader: MyShader,
    camera: crate::camera::Camera,
}

impl Application for App {
    fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let lighting_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/2.1.basic_lighting.vs"),
            include_str!("./shaders/2.2.basic_lighting.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");
        let lighting_cube_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/1.1.light_cube.vs"),
            include_str!("./shaders/1.1.light_cube.fs"),
            Some(ctx.suggested_shader_version),
        )
        .expect("Failed to create program");
        let yaw = -90.0f32;
        let camera_pos = glm::vec3(0.0, 0.0, 4.0);
        let pitch = 0.0f32;
        let camera = crate::camera::Camera::new(camera_pos, CAMERA_UP, yaw, pitch);
        Self {
            cube_vao: None,
            light_vao: None,
            vbo: None,
            lighting_shader,
            lighting_cube_shader,
            camera,
        }
    }

    fn init(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;

            gl.enable(DEPTH_TEST);

            // first, configure the cube's VAO (and VBO)
            let vbo = gl.create_buffer().expect("Cannot create vbo buffer");
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, bytemuck::cast_slice(&VERTICES), STATIC_DRAW);

            let cube_vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(cube_vao));
            // position attribute
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 6 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);
            // normal attribute
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                FLOAT,
                false,
                6 * size_of::<f32>() as i32,
                3 * size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(1);

            // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
            let light_vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(light_vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            // note that we update the lamp's position attribute's stride to reflect the updated buffer data
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 6 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);

            self.cube_vao = Some(cube_vao);
            self.light_vao = Some(light_vao);
            self.vbo = Some(vbo);
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            let mut light_pos = glm::Vec3::new(1.2, 1.0, 2.0);
            // change the light's position values over time (can be done anywhere in the render loop actually,
            // but try to do it at least before using the light source positions)
            let duration = ctx.last_update_time - ctx.start;
            let time = duration.num_milliseconds() as f32 / 1000.0;
            light_pos.x = 1.0 + (time.sin() * 2.0);
            light_pos.y = time.sin() / 2.0 * 1.0;

            self.lighting_shader.use_shader(gl);
            self.lighting_shader
                .set_vec3(gl, "objectColor", &glm::vec3(1.0, 0.5, 0.31));
            self.lighting_shader
                .set_vec3(gl, "lightColor", &glm::vec3(1.0, 1.0, 1.0));
            self.lighting_shader.set_vec3(gl, "lightPos", &light_pos);
            self.lighting_shader
                .set_vec3(gl, "viewPos", &self.camera.position());

            // view/projection transformations
            let projection = glm::perspective(
                ctx.width as f32 / ctx.height as f32,
                self.camera.zoom().to_radians(),
                0.1,
                100.0,
            );
            let view = self.camera.view_matrix();
            self.lighting_shader.set_mat4(gl, "projection", &projection);
            self.lighting_shader.set_mat4(gl, "view", &view);

            // world transformation
            let model = glm::Mat4::identity();
            self.lighting_shader.set_mat4(gl, "model", &model);

            gl.bind_vertex_array(self.cube_vao);
            gl.draw_arrays(TRIANGLES, 0, 36);

            // draw the lamp object
            self.lighting_cube_shader.use_shader(gl);
            self.lighting_cube_shader
                .set_mat4(gl, "projection", &projection);
            self.lighting_cube_shader.set_mat4(gl, "view", &view);
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &light_pos);
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2)); // a smaller cube
            self.lighting_cube_shader.set_mat4(gl, "model", &model);

            gl.bind_vertex_array(self.light_vao);
            gl.draw_arrays(TRIANGLES, 0, 36);
        }
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
        unsafe {
            self.lighting_shader.delete(gl);
            self.lighting_cube_shader.delete(gl);

            if let Some(vertex_array) = self.cube_vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(vertex_array) = self.light_vao {
                gl.delete_vertex_array(vertex_array);
            }

            if let Some(buffer) = self.vbo {
                gl.delete_buffer(buffer);
            }
        }
    }
}
