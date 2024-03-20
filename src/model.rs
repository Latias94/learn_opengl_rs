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

impl Model {
    // not using russimp for now
    // pub fn new(gl: &Context, path: &str) -> Self {
    //     let mut model = Self {
    //         meshes: Vec::new(),
    //         directory: path[0..path.rfind('/').unwrap() + 1].to_string(),
    //     };
    //
    //     model.load_model(gl, path);
    //
    //     log::info!("Model loaded: {:?}", model.meshes.len());
    //
    //     model
    // }
    //
    // pub fn load_model(&mut self, gl: &Context, path: &str) {
    //     let scene =
    //         russimp::scene::Scene::from_file(path, vec![russimp::scene::PostProcess::Triangulate])
    //             .expect("Failed to load model");
    //
    //     if scene.flags & russimp::sys::AI_SCENE_FLAGS_INCOMPLETE != 0 {
    //         panic!("Scene is incomplete");
    //     }
    //
    //     if scene.root.is_none() {
    //         panic!("Scene has no root");
    //     }
    //     self.process_node(gl, scene.root.as_ref().unwrap(), &scene);
    // }
    //
    // fn process_node(
    //     &mut self,
    //     gl: &Context,
    //     node: &russimp::node::Node,
    //     scene: &russimp::scene::Scene,
    // ) {
    //     for index in &node.meshes {
    //         let mesh = &scene.meshes[*index as usize];
    //         let mesh = self.process_mesh(gl, mesh, scene);
    //         self.meshes.push(mesh);
    //     }
    //     node.children
    //         .borrow()
    //         .iter()
    //         .for_each(|child| self.process_node(gl, child, scene));
    // }
    //
    // fn process_mesh(
    //     &self,
    //     gl: &Context,
    //     mesh: &russimp::mesh::Mesh,
    //     scene: &russimp::scene::Scene,
    // ) -> Mesh {
    //     let mut vertices = Vec::new();
    //     let mut indices = Vec::new();
    //     let mut textures = Vec::new();
    //     for (i, vert) in mesh.vertices.iter().enumerate() {
    //         let normal = &mesh.normals[i];
    //         let tex_coords = if let Some(tex_coords) = &mesh.texture_coords[0] {
    //             let tex_coord = tex_coords[i];
    //             glm::vec2(tex_coord.x, tex_coord.y)
    //         } else {
    //             glm::vec2(0.0, 0.0)
    //         };
    //         let vertex = Vertex {
    //             position: glm::vec3(vert.x, vert.y, vert.z),
    //             normal: glm::vec3(normal.x, normal.y, normal.z),
    //             tex_coords,
    //         };
    //
    //         vertices.push(vertex);
    //     }
    //
    //     for face in &mesh.faces {
    //         face.0.iter().for_each(|index| indices.push(*index));
    //     }
    //
    //     let material = &scene.materials[mesh.material_index as usize];
    //     let mut diffuse_maps = self.load_material_textures(gl, material, TextureType::Diffuse);
    //     textures.append(&mut diffuse_maps);
    //     let mut specular_maps = self.load_material_textures(gl, material, TextureType::Specular);
    //     textures.append(&mut specular_maps);
    //     let mut normal_maps = self.load_material_textures(gl, material, TextureType::Normals);
    //     textures.append(&mut normal_maps);
    //     let mut height_maps = self.load_material_textures(gl, material, TextureType::Height);
    //     textures.append(&mut height_maps);
    //
    //     Mesh::new(gl, vertices, indices, textures)
    // }
    //
    // fn load_material_textures(
    //     &self,
    //     gl: &Context,
    //     material: &russimp::material::Material,
    //     texture_type: TextureType,
    // ) -> Vec<Texture> {
    //     let mut textures = Vec::new();
    //
    //     log::info!("texture: {:?}", material.textures);
    //     if !material.textures.contains_key(&texture_type) {
    //         log::info!("No texture for {:?}", texture_type);
    //         return textures;
    //     }
    //     let texture = material.textures[&texture_type].borrow();
    //     let ty = Self::map_ai_to_texture_ty(texture_type);
    //     let texture = Texture::new(gl, texture.filename.clone(), ty);
    //     textures.push(texture);
    //
    //     textures
    // }
    //
    // fn map_ai_to_texture_ty(ai_ty: TextureType) -> TextureTy {
    //     match ai_ty {
    //         TextureType::Diffuse => TextureTy::Diffuse,
    //         TextureType::Specular => TextureTy::Specular,
    //         TextureType::Normals => TextureTy::Normal,
    //         TextureType::Height => TextureTy::Height,
    //         _ => panic!("Unknown texture type"),
    //     }
    // }
}
