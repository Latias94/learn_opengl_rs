use crate::resources;
use glow::{Context, HasContext, Program, FRAGMENT_SHADER, VERTEX_SHADER};

pub struct MyShader {
    program: Program,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
    #[allow(dead_code)]
    Compute,
}

impl MyShader {
    #[allow(dead_code)]
    pub async fn new(
        gl: &Context,
        vertex_path: &str,
        fragment_path: &str,
        shader_version: Option<&str>,
    ) -> Result<Self, String> {
        let vertex_shader = resources::load_string(vertex_path)
            .await
            .expect("Failed to load vertex shader");
        let fragment_shader = resources::load_string(fragment_path)
            .await
            .expect("Failed to load fragment shader");

        Self::new_from_source(gl, &vertex_shader, &fragment_shader, shader_version)
    }

    pub fn new_from_source(
        gl: &Context,
        vertex_shader: &str,
        fragment_shader: &str,
        shader_version: Option<&str>,
    ) -> Result<Self, String> {
        let vertex_shader =
            Self::modify_shader_to_support_webgl(vertex_shader, shader_version, ShaderType::Vertex);
        let fragment_shader = Self::modify_shader_to_support_webgl(
            fragment_shader,
            shader_version,
            ShaderType::Fragment,
        );

        #[cfg(target_arch = "wasm32")]
        {
            log::info!("vs: \n{}\n\nfs: \n{}", vertex_shader, fragment_shader);
        }
        let program = unsafe { gl.create_program().expect("Failed to create program") };

        let (vertex, fragment) = (
            Self::compile_shader(gl, VERTEX_SHADER, &vertex_shader)?,
            Self::compile_shader(gl, FRAGMENT_SHADER, &fragment_shader)?,
        );

        unsafe {
            gl.attach_shader(program, vertex);
            gl.attach_shader(program, fragment);
            gl.link_program(program);
        }

        if !unsafe { gl.get_program_link_status(program) } {
            return Err(unsafe { gl.get_program_info_log(program) });
        }

        unsafe {
            gl.detach_shader(program, vertex);
            gl.detach_shader(program, fragment);
            gl.delete_shader(vertex);
            gl.delete_shader(fragment);
        }

        Ok(Self { program })
    }

    pub fn use_shader(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    #[allow(dead_code)]
    pub fn set_bool(&self, gl: &Context, name: &str, value: bool) {
        unsafe {
            let location = gl
                .get_uniform_location(self.program, name)
                .unwrap_or_else(|| panic!("Cannot find uniform location {}", name));
            gl.uniform_1_i32(Some(&location), value as i32);
        }
    }

    pub fn set_int(&self, gl: &Context, name: &str, value: i32) {
        unsafe {
            let location = gl
                .get_uniform_location(self.program, name)
                .unwrap_or_else(|| panic!("Cannot find uniform location {}", name));
            gl.uniform_1_i32(Some(&location), value);
        }
    }

    pub fn set_float(&self, gl: &Context, name: &str, value: f32) {
        unsafe {
            let location = gl
                .get_uniform_location(self.program, name)
                .unwrap_or_else(|| panic!("Cannot find uniform location {}", name));
            gl.uniform_1_f32(Some(&location), value);
        }
    }

    pub fn set_mat4(&self, gl: &Context, name: &str, value: &nalgebra_glm::Mat4) {
        unsafe {
            let location = gl
                .get_uniform_location(self.program, name)
                .unwrap_or_else(|| panic!("Cannot find uniform location {}", name));
            gl.uniform_matrix_4_f32_slice(Some(&location), false, value.as_slice());
        }
    }

    pub fn set_vec3(&self, gl: &Context, name: &str, value: &nalgebra_glm::Vec3) {
        unsafe {
            let location = gl
                .get_uniform_location(self.program, name)
                .unwrap_or_else(|| panic!("Cannot find uniform location {}", name));
            gl.uniform_3_f32(Some(&location), value.x, value.y, value.z);
        }
    }

    pub fn compile_shader(
        gl: &Context,
        shader_type: u32,
        source: &str,
    ) -> Result<glow::Shader, String> {
        let shader = unsafe { gl.create_shader(shader_type).expect("Cannot create shader") };
        unsafe {
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
        }

        if !unsafe { gl.get_shader_compile_status(shader) } {
            return Err(unsafe { gl.get_shader_info_log(shader) });
        }

        Ok(shader)
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_program(self.program);
        }
    }

    /// for better compatibility with WebGL
    /// if first line is #version and provided shader_version is not empty, replace it
    pub fn modify_shader_to_support_webgl(
        shader_source: &str,
        shader_version: Option<&str>,
        shader_type: ShaderType,
    ) -> String {
        let shader_version = shader_version.unwrap_or("");
        // if first line is #version, replace it
        let shader_source = if !shader_version.is_empty() && shader_source.starts_with("#version") {
            let mut lines = shader_source.lines();
            let _first_line = lines.next().unwrap();
            let rest = lines.collect::<Vec<&str>>().join("\n");
            format!("{}\n{}", shader_version, rest)
        } else {
            shader_source.to_string()
        };

        // if fragment shader don't have precision defined, insert it to the second line
        let shader_source =
            if shader_type == ShaderType::Fragment && !shader_source.contains("precision") {
                let mut lines = shader_source.lines();
                let first_line = lines.next().unwrap();
                let rest = lines.collect::<Vec<&str>>().join("\n");
                format!("{}\nprecision mediump float;\n{}", first_line, rest)
            } else {
                shader_source
            };

        shader_source
    }

    pub fn program(&self) -> Program {
        self.program
    }
}
