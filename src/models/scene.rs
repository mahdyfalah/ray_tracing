use serde::Deserialize;
use crate::models::camera::Camera;
use crate::models::lights::Lights;
use crate::models::surface::{Surfaces, SurfaceType}; // Import from surface.rs

#[derive(Debug, Deserialize, PartialEq)]
pub struct Scene {
    pub output_file: String,
    pub background_color: crate::models::color::Color,
    pub camera: Camera,
    pub lights: Lights,
    pub surfaces: Surfaces,
}