// src/object.rs
use crate::ray::Ray;
use crate::point::Point;
use crate::vector::Vector;
use crate::material::Material;

#[derive(Debug, Clone)]
pub struct Object {
    pub material: Material,
    // Placeholder for transformation data; we'll keep it simple as a 4x4 matrix.
    pub transformation: [[f64; 4]; 4],
}

impl Object {
    /// Creates a new Object with material and transformation
    pub fn new(material: Material, transformation: [[f64; 4]; 4]) -> Self {
        Object {
            material,
            transformation,
        }
    }

    /// Checks for ray-object intersection
    /// Returns the distance `t` of the intersection along the ray, or None if no intersection
    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        // Example for a sphere at origin with radius 1
        let sphere_center = Point::new(0.0, 0.0, 0.0);
        let sphere_radius = 1.0;

        // Vector from ray origin to sphere center
        let oc = ray.origin - sphere_center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - sphere_radius * sphere_radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None // No intersection
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t1 > ray.t_min && t1 < ray.t_max {
                Some(t1) // First intersection within range
            } else if t2 > ray.t_min && t2 < ray.t_max {
                Some(t2) // Second intersection within range
            } else {
                None // No intersection within range
            }
        }
    }
}
