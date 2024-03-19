use crate::mesh::{Mesh, Texture, Vertex};
use crate::shader::MyShader;
use glow::Context;
use nalgebra_glm as glm;
use russimp::material::TextureType;

pub struct Model {
    meshes: Vec<Mesh>,
    directory: String,
}

impl Model {
    pub fn new(gl: &Context, path: &str) -> Self {
        todo!()
    }

    pub fn new_from_obj(gl: &Context, path: &str) -> Self {
        let opts = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        };

        let (models, directory) = tobj::load_obj(path, &opts).expect("Failed to load OBJ model");
        // let mut meshes = Vec::new();
        // for model in models {
        //     let mesh = Mesh::new(&model);
        //     meshes.push(mesh);
        // }
        // Self { meshes, directory }
        todo!()
    }

    pub fn load_model(&mut self, gl: &Context, path: &str) {
        let scene =
            russimp::scene::Scene::from_file(path, vec![russimp::scene::PostProcess::Triangulate])
                .expect("Failed to load model");

        if scene.flags & russimp::sys::AI_SCENE_FLAGS_INCOMPLETE != 0 {
            panic!("Scene is incomplete");
        }

        if scene.root.is_none() {
            panic!("Scene has no root");
        }
        self.directory = path[0..path.rfind('/').unwrap() + 1].to_string();
        self.process_node(gl, &scene.root.as_ref().unwrap(), &scene);
    }

    fn process_node(
        &mut self,
        gl: &Context,
        node: &russimp::node::Node,
        scene: &russimp::scene::Scene,
    ) {
        for index in &node.meshes {
            let mesh = &scene.meshes[*index as usize];
            let mesh = self.process_mesh(gl, mesh, scene);
            self.meshes.push(mesh);
        }
        todo!()
    }

    fn process_mesh(
        &self,
        gl: &Context,
        mesh: &russimp::mesh::Mesh,
        scene: &russimp::scene::Scene,
    ) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut textures = Vec::new();
        for (i, vert) in mesh.vertices.iter().enumerate() {
            let mut vertex = Vertex::default();
            vertex.position = glm::vec3(vert.x, vert.y, vert.z);
            let normal = &mesh.normals[i];
            vertex.normal = glm::vec3(normal.x, normal.y, normal.z);
            if let Some(tex_coords) = &mesh.texture_coords[0] {
                let tex_coord = tex_coords[i];
                vertex.tex_coords = glm::vec2(tex_coord.x, tex_coord.y);
            } else {
                vertex.tex_coords = glm::vec2(0.0, 0.0);
            }

            vertices.push(vertex);
        }

        for face in &mesh.faces {
            face.0.iter().for_each(|index| indices.push(*index));
        }

        let material = &scene.materials[mesh.material_index as usize];
        let mut diffuse_maps =
            self.load_material_textures(gl, material, TextureType::Diffuse, "texture_diffuse");
        textures.append(&mut diffuse_maps);
        let mut specular_maps =
            self.load_material_textures(gl, material, TextureType::Specular, "texture_specular");
        textures.append(&mut specular_maps);

        Mesh::new(gl, vertices, indices, textures)
    }

    fn load_material_textures(
        &self,
        gl: &Context,
        material: &russimp::material::Material,
        texture_type: TextureType,
        ty: &str,
    ) -> Vec<Texture> {
        let mut textures = Vec::new();
        for (i, (texture_type, texture)) in material.textures.iter().enumerate() {
            // if texture.0 == ty {
            // let path = format!("{}{}", self.directory, texture.1);
            // let texture = Texture::new(gl, &path, ty);
            // textures.push(texture);
            // }
        }
        textures
    }

    pub fn draw(&self, gl: &Context, shader: &MyShader) {
        for mesh in &self.meshes {
            mesh.draw(gl, shader);
        }
    }
}
