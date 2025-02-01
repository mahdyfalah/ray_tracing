use std::io;
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

impl Scene {
    /// Loads OBJ models for all meshes in the scene
    pub fn load_meshes(&mut self) -> io::Result<()> {
        let obj_base_path = "assets/obj_models/";

        for surface in &mut self.surfaces.surfaces {
            if let SurfaceType::Mesh(mesh) = surface {
                let obj_path = format!("{}{}", obj_base_path, mesh.name);
                mesh.load_obj(obj_path)?;
            }
        }

        Ok(())
    }
}