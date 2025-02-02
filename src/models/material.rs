use image::RgbImage;
use serde::Deserialize;
use crate::models::color::Color;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MaterialSolid {
    pub color: Color,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MaterialTextured {
    pub texture: Texture,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Material {
    Solid(MaterialSolid),
    Textured(MaterialTextured),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Phong {
    pub ka: f64,         // Ambient coefficient
    pub kd: f64,         // Diffuse coefficient
    pub ks: f64,         // Specular coefficient
    pub exponent: f64,   // Shininess exponent
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Reflectance {
    pub r: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Transmittance {
    pub t: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Refraction {
    pub iof: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Texture {
    pub name: String,
    #[serde(skip)]
    pub data: Option<RgbImage>,
}

impl Material {
    pub fn color(&self) -> Color {
        let black = Color::new(0.0, 0.0, 0.0);

        match self {
            Material::Solid(s) => s.color,
            Material::Textured(_) => black,
        }
    }

    pub fn texture(&self) -> &str {
        match self {
            Material::Solid(_) => "No Texture!",
            Material::Textured(t) => &t.texture.name
        }
    }

    pub fn phong(&self) -> &Phong {
        match self {
            Material::Solid(s) => &s.phong,
            Material::Textured(t) => &t.phong,
        }
    }

    pub fn reflectance(&self) -> &Reflectance {
        match self {
            Material::Solid(s) => &s.reflectance,
            Material::Textured(t) => &t.reflectance,
        }
    }

    pub fn transmittance(&self) -> &Transmittance {
        match self {
            Material::Solid(s) => &s.transmittance,
            Material::Textured(t) => &t.transmittance,
        }
    }

    pub fn refraction(&self) -> &Refraction {
        match self {
            Material::Solid(s) => &s.refraction,
            Material::Textured(t) => &t.refraction,
        }
    }
}

use std::path::{Path, PathBuf};
use image::ImageError;

impl Texture {
    /// Loads the texture from the assets/textures directory as an RgbImage.
    pub fn load(&mut self, base_path: &Path) -> Result<(), ImageError> {
        let texture_path: PathBuf = base_path.join("assets/textures").join(&self.name);
        // Open the image file
        let img = image::open(&texture_path)?.to_rgb8();
        self.data = Some(img);
        Ok(())
    }
}
