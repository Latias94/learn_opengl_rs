use crate::shader::MyShader;
use bytemuck::{offset_of, Pod, Zeroable};
use glow::{Buffer, Context, HasContext, VertexArray};
use nalgebra_glm as glm;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

pub struct Texture {
    pub raw: glow::Texture,
    pub ty: String,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
}

impl Mesh {
    pub fn new(
        gl: &Context,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        textures: Vec<Texture>,
    ) -> Self {
        let vao = unsafe {
            gl.create_vertex_array()
                .expect("Cannot create vertex array")
        };
        let vbo = unsafe { gl.create_buffer().expect("Cannot create buffer") };
        let ebo = unsafe { gl.create_buffer().expect("Cannot create buffer") };
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo,
        };
        mesh.setup_mesh(gl);
        mesh
    }

    fn setup_mesh(&mut self, gl: &Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&self.vertices),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.indices),
                glow::STATIC_DRAW,
            );

            let stride = std::mem::size_of::<Vertex>() as i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                stride,
                offset_of!(Vertex, position) as i32,
            );

            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                offset_of!(Vertex, normal) as i32,
            );

            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                stride,
                offset_of!(Vertex, tex_coords) as i32,
            );

            gl.bind_vertex_array(None);
        }
    }

    pub fn draw(&self, gl: &Context, shader: &MyShader) {
        unsafe {
            let mut diffuse_nr = 1;
            let mut specular_nr = 1;
            let mut normal_nr = 1;
            let mut height_nr = 1;
            for (i, texture) in self.textures.iter().enumerate() {
                gl.active_texture(glow::TEXTURE0 + i as u32);
                let name = match texture.ty.as_str() {
                    "texture_diffuse" => {
                        diffuse_nr += 1;
                        format!("texture_diffuse{}", diffuse_nr)
                    }
                    "texture_specular" => {
                        specular_nr += 1;
                        format!("texture_specular{}", specular_nr)
                    }
                    "texture_normal" => {
                        normal_nr += 1;
                        format!("texture_normal{}", normal_nr)
                    }
                    "texture_height" => {
                        height_nr += 1;
                        format!("texture_height{}", height_nr)
                    }
                    _ => panic!("Unknown texture type"),
                };
                shader.set_int(gl, &format!("material.{}", name), i as i32);
                gl.create_texture().expect("Create texture");
                gl.bind_texture(glow::TEXTURE_2D, Some(texture.raw));
            }
            gl.active_texture(glow::TEXTURE0);

            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(
                glow::TRIANGLES,
                self.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );
            gl.bind_vertex_array(None);
        }
    }

    pub fn clear(&self, gl: &Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}
