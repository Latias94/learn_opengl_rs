use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, AppContext, Application, WindowInitInfo};
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async unsafe fn main_2_3_2() {
    let init_info = WindowInitInfo::builder()
        .title("Materials Exercise 1".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
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

const LIGHT_POS: glm::Vec3 = glm::Vec3::new(1.2, 1.0, 2.0);

struct App {
    cube_vao: VertexArray,
    light_vao: VertexArray,
    vbo: Buffer,
    lighting_shader: MyShader,
    lighting_cube_shader: MyShader,
    camera: Camera,
}

impl Application for App {
    async unsafe fn new(ctx: &AppContext) -> Self {
        let gl = ctx.gl();
        let lighting_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/3.1.materials.vs"),
            include_str!("./shaders/3.1.materials.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let lighting_cube_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/1.1.light_cube.vs"),
            include_str!("./shaders/1.1.light_cube.fs"),
            Some(ctx.suggested_shader_version()),
        )
        .expect("Failed to create program");
        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera = Camera::new_with_position(camera_pos);

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

        Self {
            cube_vao,
            light_vao,
            vbo,
            lighting_shader,
            lighting_cube_shader,
            camera,
        }
    }

    unsafe fn render(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

        // be sure to activate shader when setting uniforms/drawing objects
        self.lighting_shader.use_shader(gl);
        self.lighting_shader
            .set_vec3(gl, "light.position", &LIGHT_POS);
        self.lighting_shader
            .set_vec3(gl, "viewPos", &self.camera.position());

        // light properties
        // note that all light colors are set at full intensity
        self.lighting_shader
            .set_vec3(gl, "light.ambient", &glm::vec3(1.0, 1.0, 1.0));
        self.lighting_shader
            .set_vec3(gl, "light.diffuse", &glm::vec3(1.0, 1.0, 1.0));
        self.lighting_shader
            .set_vec3(gl, "light.specular", &glm::vec3(1.0, 1.0, 1.0));

        // material properties
        self.lighting_shader
            .set_vec3(gl, "material.ambient", &glm::vec3(0.0, 0.1, 0.06));
        self.lighting_shader.set_vec3(
            gl,
            "material.diffuse",
            &glm::vec3(0.0, 0.509_803_9, 0.509_803_9),
        );
        // specular lighting doesn't have full effect on this object's material
        self.lighting_shader.set_vec3(
            gl,
            "material.specular",
            &glm::vec3(0.50196078, 0.50196078, 0.50196078),
        );
        self.lighting_shader
            .set_float(gl, "material.shininess", 32.0);

        // view/projection transformations
        let projection = glm::perspective(
            ctx.width() as f32 / ctx.height() as f32,
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

        gl.bind_vertex_array(Some(self.cube_vao));
        gl.draw_arrays(TRIANGLES, 0, 36);

        // draw the lamp object
        self.lighting_cube_shader.use_shader(gl);
        self.lighting_cube_shader
            .set_mat4(gl, "projection", &projection);
        self.lighting_cube_shader.set_mat4(gl, "view", &view);
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &LIGHT_POS);
        model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2)); // a smaller cube
        self.lighting_cube_shader.set_mat4(gl, "model", &model);

        gl.bind_vertex_array(Some(self.light_vao));
        gl.draw_arrays(TRIANGLES, 0, 36);
    }

    unsafe fn process_input(&mut self, _ctx: &AppContext, input: &WinitInputHelper) {
        self.camera.process_keyboard_with_input(input);
        self.camera.process_mouse_with_input(input, true);
    }

    unsafe fn exit(&mut self, ctx: &AppContext) {
        let gl = ctx.gl();

        self.lighting_shader.delete(gl);
        self.lighting_cube_shader.delete(gl);

        gl.delete_vertex_array(self.cube_vao);

        gl.delete_vertex_array(self.light_vao);

        gl.delete_buffer(self.vbo);
    }
}
