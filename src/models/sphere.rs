use serde::Deserialize;
use crate::models::point::Point;
use crate::models::material::MaterialSolid;
use crate::models::surface::Surface;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub position: Point,
    pub material_solid: MaterialSolid,
}

impl Surface for Sphere {
    fn intersect(&self) {
        // TODO: Implement intersection algorithm
        println!("Sphere intersection algorithm not implemented yet.");
    }
}
