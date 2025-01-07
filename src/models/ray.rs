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

    /// Computes the reflection ray
    pub fn reflect(&self, intersection_point: Point, normal: Vector) -> Ray {
        // Reflect the direction vector based on the normal
        let reflected_direction = self.direction - normal * 2.0 * self.direction.dot(normal);

        // Return a new ray starting slightly off the intersection point to avoid self-intersection
        Ray::new(
            intersection_point + normal * 1e-4, // Offset to prevent floating-point errors
            reflected_direction.normalize(),
            1e-4,                               // Minimum t to avoid hitting the same point
            f64::INFINITY,                      // Maximum t
        )
    }
}
