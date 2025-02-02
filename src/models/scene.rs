use std::io;
use std::path::Path;
use serde::Deserialize;
use crate::models::camera::Camera;
use crate::models::color::Color;
use crate::models::lights::Lights;
use crate::models::surface::{Surfaces, SurfaceType}; // Import from surface.rs

#[derive(Debug, Deserialize, PartialEq)]
pub struct Scene {
    pub output_file: String,
    pub background_color: Color,
    pub camera: Camera,
    pub lights: Lights,
    pub surfaces: Surfaces,
}

impl Scene {
    /// Loads OBJ models for all meshes in the scene
    pub fn load_meshes(&mut self) -> io::Result<()> {
        let obj_base_path = "assets/obj_models/";
        // Base path used for loading textures; adjust as needed.
        let base_path = Path::new(".");

        for surface in &mut self.surfaces.surfaces {
            if let SurfaceType::Mesh(mesh) = surface {
                // Load the geometry from the OBJ file.
                let obj_path = format!("{}{}", obj_base_path, mesh.name);
                mesh.load_obj(&obj_path)?;

                // Since the mesh has either a solid or textured material,
                // only load a texture if it has a textured material.
                if mesh.material_textured.is_some() {
                    // Convert the ImageError into an io::Error if needed.
                    mesh.load_texture(base_path)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                }
            }
        }

        Ok(())
    }
}