use crate::models::point::Point;
use crate::models::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub t_min: f64, // Minimum distance
    pub t_max: f64, // Maximum distance
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, t_min: f64, t_max: f64) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
            t_min,
            t_max,
        }
    }

    /// Computes a point along the ray at distance `t`
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
