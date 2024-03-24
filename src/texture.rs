use glow::{
    Context, HasContext, LINEAR, LINEAR_MIPMAP_LINEAR, REPEAT, RGBA, TEXTURE_2D,
    TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNSIGNED_BYTE,
};
use image::GenericImageView;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextureType {
    Diffuse,
    Specular,
    Normal,
    Height,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Texture {
    raw: glow::Texture,
    file_name: String,
    ty: TextureType,
}

impl Texture {
    pub fn from_image(
        gl: &Context,
        img: &image::DynamicImage,
        file_name: &str,
        ty: TextureType,
    ) -> anyhow::Result<Self> {
        // let img = img.flipv();
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();
        let raw = unsafe {
            let texture = gl
                .create_texture()
                .map_err(|e| anyhow::anyhow!("Failed to create texture: {:?}", e))?;
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
            gl.bind_texture(TEXTURE_2D, None);

            texture
        };
        let file_name = file_name.to_string();
        Ok(Texture { raw, file_name, ty })
    }

    pub fn from_bytes(
        gl: &Context,
        bytes: &[u8],
        file_name: &str,
        ty: TextureType,
    ) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes).expect("Failed to load texture from bytes");
        Self::from_image(gl, &img, file_name, ty)
    }

    pub fn set_wrap_mode(&self, gl: &Context, wrap_s: i32, wrap_t: i32) {
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(self.raw));
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, wrap_s);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, wrap_t);
            gl.bind_texture(TEXTURE_2D, None);
        }
    }

    #[allow(dead_code)]
    pub fn set_filter_mode(&self, gl: &Context, min_filter: i32, mag_filter: i32) {
        unsafe {
            gl.bind_texture(TEXTURE_2D, Some(self.raw));
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, min_filter);
            gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, mag_filter);
            gl.bind_texture(TEXTURE_2D, None);
        }
    }

    pub fn ty(&self) -> TextureType {
        self.ty
    }

    pub fn raw(&self) -> glow::Texture {
        self.raw
    }

    #[allow(dead_code)]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_texture(self.raw);
        }
    }

    pub fn bind(&self, gl: &Context, slot: u32) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + slot);
            gl.bind_texture(TEXTURE_2D, Some(self.raw));
        }
    }
}

pub fn map_texture_type_to_string(ty: TextureType) -> String {
    match ty {
        TextureType::Diffuse => "texture_diffuse".to_string(),
        TextureType::Specular => "texture_specular".to_string(),
        TextureType::Normal => "texture_normal".to_string(),
        TextureType::Height => "texture_height".to_string(),
    }
}

#[allow(dead_code)]
pub fn map_string_to_texture_type(s: &str) -> TextureType {
    match s {
        "texture_diffuse" => TextureType::Diffuse,
        "texture_specular" => TextureType::Specular,
        "texture_normal" => TextureType::Normal,
        "texture_height" => TextureType::Height,
        _ => panic!("Unknown texture type"),
    }
}
