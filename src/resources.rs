use crate::mesh::{Material, Mesh, Vertex};
use crate::model::Model;
use crate::texture::{Texture, TextureType};
use cfg_if::cfg_if;
use glow::Context;
use nalgebra_glm as glm;
use std::io::{BufReader, Cursor};
use std::path::Path;

#[cfg(target_arch = "wasm32")]
fn format_url(file_name: &str) -> reqwest::Url {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let base = reqwest::Url::parse(&format!(
        "{}/{}/",
        location.origin().unwrap(),
        option_env!("RES_PATH").unwrap_or("resources"),
    ))
    .unwrap();
    base.join(file_name).unwrap()
}

#[allow(dead_code)]
pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let txt = reqwest::get(url)
                .await?
                .text()
                .await?;
        } else {
            let path = Path::new(env!("OUT_DIR"))
                .join("resources")
                .join(file_name);
            let txt = std::fs::read_to_string(&path);
            if let Err(e) = &txt {
                log::error!("Failed to load file. path {:?}, reason: {:?}", path, e);
            }
            let txt = txt?;
        }
    }

    Ok(txt)
}

#[allow(dead_code)]
pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let data = reqwest::get(url)
                .await?
                .bytes()
                .await?
                .to_vec();
        } else {
            let path = Path::new(env!("OUT_DIR"))
                .join("resources")
                .join(file_name);
            let data = std::fs::read(&path);
            if let Err(e) = &data {
                log::error!("Failed to load file. path {:?}, reason: {:?}", path, e);
            }
            let data = data?;
        }
    }

    Ok(data)
}

pub async fn load_texture_with_type(
    gl: &Context,
    file_name: &str,
    ty: TextureType,
) -> anyhow::Result<Texture> {
    log::info!("Loading texture ty: {:?}, file_name: {}", ty, file_name);
    let data = load_binary(file_name).await?;
    Texture::from_bytes(gl, &data, file_name, ty)
}

pub async fn load_texture(gl: &Context, file_name: &str) -> anyhow::Result<Texture> {
    log::info!("Loading texture file_name: {}", file_name);
    let data = load_binary(file_name).await?;
    Texture::from_bytes(gl, &data, file_name, TextureType::Diffuse)
}

pub async fn load_obj(gl: &Context, file_name: &str) -> anyhow::Result<Model> {
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let model_directory_path = Path::new(file_name)
        .parent()
        .map(|p| p.to_str().unwrap_or(""))
        .unwrap_or("");
    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |p| async move {
            let material_relative_path = format!("{}/{}", model_directory_path, p);
            let mat_text = load_string(&material_relative_path).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let mut textures = Vec::new();
        if let Some(p) = m.diffuse_texture {
            let path = format!("{}/{}", model_directory_path, p);
            let diffuse = load_texture_with_type(gl, &path, TextureType::Diffuse).await?;
            textures.push(diffuse);
        }
        // if let Some(p) = m.specular_texture {
        //     let path = format!("{}/{}", model_directory_path, p);
        //     let specular = load_texture(gl, &path, TextureType::Specular).await?;
        //     textures.push(specular);
        // }
        // if let Some(p) = m.normal_texture {
        //     let path = format!("{}/{}", model_directory_path, p);
        //     let normal = load_texture(gl, &path, TextureType::Normal).await?;
        //     textures.push(normal);
        // }

        materials.push(Material {
            name: m.name,
            textures,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| Vertex {
                    position: glm::vec3(
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ),
                    tex_coords: glm::vec2(m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]),
                    normal: glm::vec3(
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ),
                })
                .collect::<Vec<_>>();

            Mesh::new(
                gl,
                &m.name,
                vertices,
                m.mesh.indices,
                m.mesh.material_id.unwrap_or(0),
            )
        })
        .collect::<Vec<_>>();

    Ok(Model { meshes, materials })
}
