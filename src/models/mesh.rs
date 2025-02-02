use std::io;
use std::path::Path;
use serde::Deserialize;
use crate::models::intersection::Intersection;
use crate::models::material::{Material, MaterialSolid, MaterialTextured};
use crate::models::ray::Ray;
use crate::models::surface::Surface;
use crate::models::triangle::Triangle;
use crate::services::obj_parser_service::read_obj_file;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    pub name: String,
    #[serde(default)]
    pub material_solid: Option<MaterialSolid>,
    #[serde(default)]
    pub material_textured: Option<MaterialTextured>,
    #[serde(skip)]
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Creates a new mesh with no triangles (used during XML deserialization)
    pub fn new(name: String, material_solid: Option<MaterialSolid>, material_textured: Option<MaterialTextured>) -> Self {
        Self {
            name,
            material_solid,
            material_textured,
            triangles: Vec::new(),
        }
    }

    /// Loads triangles from an OBJ file
    pub fn load_obj<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let obj_model = read_obj_file(path)?;
        self.triangles = obj_model.to_triangles();
        Ok(())
    }

    pub fn material(&self) -> Material {
        if let Some(m) = &self.material_solid {
            Material::Solid(m.clone())
        } else if let Some(m) = &self.material_textured {
            Material::Textured(m.clone())
        } else {
            unreachable!("There must always be either a solid or textured material.");
        }
    }

}

impl Surface for Mesh {
    /// Computes if there is an intersection between the mesh and a ray.
    /// If an intersection exists, returns an `Intersection` object with the intersection data.
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;

        // Iterate through all triangles in the mesh
        for triangle in &self.triangles {
            // Check for intersection with the triangle
            if let Some(intersection) = triangle.intersect(ray, &self.material()) {
                // Keep the closest intersection
                closest = match closest {
                    Some(current) if intersection.t < current.t => Some(intersection),
                    None => Some(intersection),
                    _ => closest,
                };
            }
        }

        closest
    }
}