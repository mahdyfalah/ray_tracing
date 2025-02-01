use crate::models::point::Point;
use crate::models::vector::Vector;
use crate::models::material_solid::MaterialSolid;

/// Stores the result of a ray-surface intersection
#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,                   // Distance along the ray
    pub point: Point,             // Intersection point
    pub normal: Vector,           // Surface normal at intersection
    pub material: MaterialSolid,  // Material at the point
}


impl Intersection {
    /// Creates a new intersection
    pub fn new(
        t: f64,
        point: Point,
        normal: Vector,
        material: MaterialSolid,
    ) -> Self {
        Self {
            t,
            point,
            normal,
            material,
        }
    }
}