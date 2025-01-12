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
    /// source: tutorial page 12
    pub fn generate_ray(&self, u: u32, v: u32) -> Ray {
        let x_n = (u as f64 + 0.5) / self.resolution.horizontal as f64;
        let y_n = (v as f64 + 0.5) / self.resolution.vertical as f64;

        let fov_x = self.horizontal_fov.angle.to_radians();
        let fov_y = fov_x * (self.resolution.vertical as f64 / self.resolution.horizontal as f64);

        let x_i = (2.0 * x_n - 1.0) * fov_x.tan();
        let y_i = (1.0 - 2.0 * y_n) * fov_y.tan();

        let direction = Vector::new(x_i, y_i, -1.0).normalize();

        Ray::new(self.position, direction, 0.01, f64::INFINITY)
    }
}
