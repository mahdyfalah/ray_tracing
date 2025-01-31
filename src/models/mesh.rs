use serde::Deserialize;
use crate::models::intersection::Intersection;
use crate::models::material::MaterialSolid;
use crate::models::ray::Ray;
use crate::models::surface::Surface;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Mesh {
    pub name: String,
    pub material_solid: MaterialSolid
}

impl Surface for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}