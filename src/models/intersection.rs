use crate::models::point::Point;
use crate::models::vector::Vector;
use crate::models::material::MaterialSolid;

/// Stores the result of a ray-surface intersection
#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,                   // Distance along the ray
    pub point: Point,             // Intersection point
    pub normal: Vector,           // Surface normal at intersection
    pub material: MaterialSolid,  // Material at the point
}
