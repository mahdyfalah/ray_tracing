use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub t_min: f64,
    pub t_max: f64,
}

impl Ray {
    /// Creates a new Ray with the given origin, direction, and bounds for t
    pub fn new(origin: Point, direction: Vector, t_min: f64, t_max: f64) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
            t_min,
            t_max,
        }
    }

    /// Computes a point along the ray at parameter t
    pub fn point_at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
