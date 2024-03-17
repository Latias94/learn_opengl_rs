use glow::{Context, HasContext, Program, FRAGMENT_SHADER, VERTEX_SHADER};
use std::path::Path;

pub struct MyShader {
    program: Program,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

impl MyShader {
    pub fn new<P: AsRef<Path>>(
        gl: &Context,
        vertex_path: P,
        fragment_path: P,
        shader_version: Option<&str>,
    ) -> Result<Self, String> {
        let vertex_shader =
            std::fs::read_to_string(vertex_path).expect("Failed to read vertex shader");
        let fragment_shader =
            std::fs::read_to_string(fragment_path).expect("Failed to read fragment shader");

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

        let program = unsafe { gl.create_program().expect("Cannot create program") };

        let (vertex, fragment) = (
            Self::compile_shader(&gl, VERTEX_SHADER, &vertex_shader)?,
            Self::compile_shader(&gl, FRAGMENT_SHADER, &fragment_shader)?,
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

    pub fn set_bool(&self, gl: &Context, name: &str, value: bool) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_i32(Some(&location), value as i32);
        }
    }

    pub fn set_int(&self, gl: &Context, name: &str, value: i32) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_i32(Some(&location), value);
        }
    }

    pub fn set_float(&self, gl: &Context, name: &str, value: f32) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name).unwrap();
            gl.uniform_1_f32(Some(&location), value);
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

        // if fragment shader dont have precision defined, insert it to second line
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