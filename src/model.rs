use crate::mesh::{Material, Mesh};
use crate::shader::MyShader;
use glow::Context;

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl Model {
    pub fn draw(&self, gl: &Context, shader: &MyShader) {
        for mesh in &self.meshes {
            mesh.draw(gl, &self.materials, shader);
        }
    }

    pub fn delete(&self, gl: &Context) {
        for mesh in &self.meshes {
            mesh.delete(gl);
        }
    }
}
