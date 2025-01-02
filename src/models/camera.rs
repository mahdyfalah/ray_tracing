use serde::Deserialize;
use crate::models::point::Point;
use crate::models::vector::Vector;
use crate::models::ray::Ray;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Camera {
    pub position: Point,
    #[serde(rename = "lookat")]
    pub look_at: Vector,
    pub up: Vector,
    pub horizontal_fov: Fov,
    pub resolution: Resolution,
    pub max_bounces: MaxBounces,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Fov {
    pub angle: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resolution {
    pub horizontal: u32,
    pub vertical: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MaxBounces {
    pub n: u32,
}

impl Camera {
    /// Generates a ray for a given pixel (u, v)
    pub fn generate_ray(&self, u: u32, v: u32) -> Ray {
        // 1. Normalize pixel coordinates
        let x_n = (u as f64 + 0.5) / self.resolution.horizontal as f64;
        let y_n = (v as f64 + 0.5) / self.resolution.vertical as f64;

        // 2. FOV calculations (corrected scaling for fov_y)
        let fov_x = self.horizontal_fov.angle.to_radians(); // Horizontal FOV
        let fov_y = fov_x * (self.resolution.vertical as f64 / self.resolution.horizontal as f64); // Aspect ratio scaling

        // 3. Map to image plane range [-1, 1]
        let x_i = (2.0 * x_n - 1.0) * fov_x.tan();
        let y_i = (1.0 - 2.0 * y_n) * fov_y.tan(); // Flipped vertically

        // 4. Calculate direction
        let direction = Vector::new(x_i, y_i, -1.0).normalize();

        // 5. Create and return the ray
        Ray::new(self.position, direction, 0.01, f64::INFINITY) // t_min = 0.01 to avoid self-intersections
    }
}
