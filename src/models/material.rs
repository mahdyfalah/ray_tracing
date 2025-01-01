use serde::Deserialize;
use crate::models::color::Color;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MaterialSolid {
    pub color: Color,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Phong {
    pub ka: f64,         // Ambient coefficient
    pub kd: f64,         // Diffuse coefficient
    pub ks: f64,         // Specular coefficient
    pub exponent: f64,   // Shininess exponent
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Reflectance {
    pub r: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Transmittance {
    pub t: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Refraction {
    pub iof: f64, // Index of refraction
}
