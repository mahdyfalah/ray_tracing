use serde::Deserialize;
use crate::models::point::Point;
use crate::models::material::MaterialSolid;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub position: Point,
    pub material_solid: MaterialSolid,
}
