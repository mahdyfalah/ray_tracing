use serde::Deserialize;
use crate::models::ray::Ray;
use crate::models::intersection::Intersection;
use crate::models::sphere::Sphere;
use crate::models::mesh::Mesh;

/// Defines the behavior for surfaces
pub trait Surface {
    /// Calculates the intersection with a ray and returns an optional result
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

/// A collection of surfaces that can include spheres or meshes
#[derive(Debug, Deserialize, PartialEq)]
pub struct Surfaces {
    #[serde(rename = "$value")]
    pub surfaces: Vec<SurfaceType>, // Now holds any surface type
}

/// Enum to represent different types of surfaces
#[derive(Debug, Deserialize, PartialEq)]
pub enum SurfaceType {
    #[serde(rename = "sphere")]
    Sphere(Sphere),
    #[serde(rename = "mesh")]
    Mesh(Mesh),
}

impl Surface for SurfaceType {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            SurfaceType::Sphere(sphere) => sphere.intersect(ray),
            SurfaceType::Mesh(mesh) => mesh.intersect(ray),
        }
    }
}