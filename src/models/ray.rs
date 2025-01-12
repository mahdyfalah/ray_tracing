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

    /// Calculates a point along the ray at distance t
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    /// Calculates the reflection ray
    /// source: raytracing in one weekend 10.4.
    /// r = v − 2 (v ⋅ n) n
    pub fn reflect(&self, intersection_point: Point, normal: Vector) -> Ray {
        let reflected_direction = self.direction - normal * 2.0 * self.direction.dot(normal);

        Ray::new(
            intersection_point + normal,
            reflected_direction.normalize(),
            0.0,
            f64::INFINITY,
        )
    }
}
