use crate::camera::Camera;
use crate::shader::MyShader;
use crate::window::{run, Application, GLContext, WindowInitInfo};
use anyhow::Result;
use glow::*;
use nalgebra_glm as glm;
use std::mem::size_of;
use winit_input_helper::WinitInputHelper;

pub async fn main_2_5_3() {
    let init_info = WindowInitInfo::builder()
        .title("Light Casters Spot".to_string())
        .build();
    unsafe {
        run::<App>(init_info).await;
    }
}

// set up vertex data (and buffer(s)) and configure vertex attributes
// ------------------------------------------------------------------
#[rustfmt::skip]
const VERTICES: [f32; 288] = [
    // pos             normal           tex_coord
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,
    0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0,  1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  1.0,
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0,  0.0,

    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,
    0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0,  1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0,  0.0,

    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0,  1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0,  0.0,

    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,
    0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0,  1.0,
    0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0,  0.0,
    0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0,  0.0,

    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,
    0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0,  1.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0,  1.0,

    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0,
    0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0,  1.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
    0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0,  1.0
];

const CUBE_POSITIONS: [glm::Vec3; 10] = [
    glm::Vec3::new(0.0, 0.0, 0.0),
    glm::Vec3::new(2.0, 5.0, -15.0),
    glm::Vec3::new(-1.5, -2.2, -2.5),
    glm::Vec3::new(-3.8, -2.0, -12.3),
    glm::Vec3::new(2.4, -0.4, -3.5),
    glm::Vec3::new(-1.7, 3.0, -7.5),
    glm::Vec3::new(1.3, -2.0, -2.5),
    glm::Vec3::new(1.5, 2.0, -2.5),
    glm::Vec3::new(1.5, 0.2, -1.5),
    glm::Vec3::new(-1.3, 1.0, -1.5),
];

struct App {
    cube_vao: VertexArray,
    light_vao: VertexArray,
    vbo: Buffer,
    diffuse_map: Texture,
    specular_map: Texture,
    lighting_shader: MyShader,
    lighting_cube_shader: MyShader,
    camera: crate::camera::Camera,
}

impl Application for App {
    async fn new(ctx: &GLContext) -> Self {
        let gl = &ctx.gl;
        let lighting_shader = MyShader::new_from_source(
            gl,
            // embedded shader
            include_str!("./shaders/4.1.lighting_maps.vs"),
            include_str!("./shaders/5.3.light_casters.fs"),
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
        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera = Camera::new_with_position(camera_pos);

        unsafe {
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
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 8 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);
            // normal attribute
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                FLOAT,
                false,
                8 * size_of::<f32>() as i32,
                3 * size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(1);
            // texture coord attribute
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                FLOAT,
                false,
                8 * size_of::<f32>() as i32,
                6 * size_of::<f32>() as i32,
            );
            gl.enable_vertex_attrib_array(2);

            // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
            let light_vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(light_vao));
            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            // note that we update the lamp's position attribute's stride to reflect the updated buffer data
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 8 * size_of::<f32>() as i32, 0);
            gl.enable_vertex_attrib_array(0);

            // load textures
            let diffuse_map = load_texture_from_bytes(
                gl,
                include_bytes!("../../resources/textures/container2.png"),
            )
            .expect("Failed to load texture");

            lighting_shader.use_shader(gl);
            lighting_shader.set_int(gl, "material.diffuse", 0);

            let specular_map = load_texture_from_bytes(
                gl,
                include_bytes!("../../resources/textures/container2_specular.png"),
            )
            .expect("Failed to load texture");
            lighting_shader.set_int(gl, "material.specular", 1);

            Self {
                cube_vao,
                light_vao,
                vbo,
                lighting_shader,
                lighting_cube_shader,
                camera,
                diffuse_map,
                specular_map,
            }
        }
    }

    fn render(&mut self, ctx: &GLContext) {
        unsafe {
            let gl = &ctx.gl;
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

            // be sure to activate shader when setting uniforms/drawing objects
            self.lighting_shader.use_shader(gl);
            self.lighting_shader
                .set_vec3(gl, "light.position", &self.camera.position());
            self.lighting_shader
                .set_vec3(gl, "light.direction", &self.camera.front());
            self.lighting_shader
                .set_float(gl, "light.cutOff", 12.5_f32.to_radians().cos());
            self.lighting_shader
                .set_vec3(gl, "viewPos", &self.camera.position());

            // light properties
            self.lighting_shader
                .set_vec3(gl, "light.ambient", &glm::vec3(0.2, 0.2, 0.2));
            self.lighting_shader
                .set_vec3(gl, "light.diffuse", &glm::vec3(0.5, 0.5, 0.5));
            self.lighting_shader
                .set_vec3(gl, "light.specular", &glm::vec3(1.0, 1.0, 1.0));
            self.lighting_shader.set_float(gl, "light.constant", 1.0);
            self.lighting_shader.set_float(gl, "light.linear", 0.09);
            self.lighting_shader.set_float(gl, "light.quadratic", 0.032);

            // material properties
            self.lighting_shader
                .set_float(gl, "material.shininess", 32.0);

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

            // bind diffuse map
            gl.active_texture(TEXTURE0);
            gl.bind_texture(TEXTURE_2D, Some(self.diffuse_map));
            // bind specular map
            gl.active_texture(TEXTURE1);
            gl.bind_texture(TEXTURE_2D, Some(self.specular_map));

            gl.bind_vertex_array(Some(self.cube_vao));

            for (i, pos) in CUBE_POSITIONS.iter().enumerate() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, pos);
                let angle = 20.0 * i as f32;
                model = glm::rotate(&model, angle.to_radians(), &glm::vec3(1.0, 0.3, 0.5));
                self.lighting_shader.set_mat4(gl, "model", &model);

                gl.draw_arrays(TRIANGLES, 0, 36);
            }
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

            gl.delete_vertex_array(self.cube_vao);

            gl.delete_vertex_array(self.light_vao);

            gl.delete_buffer(self.vbo);

            gl.delete_texture(self.diffuse_map);

            gl.delete_texture(self.specular_map);
        }
    }
}

fn load_texture_from_bytes(gl: &Context, bytes: &[u8]) -> Result<Texture> {
    let img = image::load_from_memory(bytes)?.flipv().to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();
    let texture = unsafe {
        let texture = gl.create_texture().expect("Create texture");
        gl.bind_texture(TEXTURE_2D, Some(texture));
        gl.tex_image_2d(
            TEXTURE_2D,
            0,
            RGBA as i32,
            width as i32,
            height as i32,
            0,
            RGBA,
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
