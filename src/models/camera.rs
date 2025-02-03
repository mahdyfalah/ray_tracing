use serde::Deserialize;
use crate::models::point::Point;
use crate::models::vector::Vector;
use crate::models::ray::Ray;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Camera {
    pub position: Point,
    #[serde(rename = "lookat")]
    pub look_at: Point,
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
    /// Generates a ray for a given pixel (u, v) with camera transformations applied
    pub fn generate_ray(&self, u: u32, v: u32) -> Ray {
        // Calculate normalized pixel coordinates
        let x_n = (u as f64 + 0.5) / self.resolution.horizontal as f64;
        let y_n = (v as f64 + 0.5) / self.resolution.vertical as f64;

        // Calculate FOV components
        let fov_x = self.horizontal_fov.angle.to_radians();
        let fov_y = fov_x * (self.resolution.vertical as f64 / self.resolution.horizontal as f64);

        // Compute coordinates in camera space
        let x_i = (2.0 * x_n - 1.0) * fov_x.tan();
        let y_i = (1.0 - 2.0 * y_n) * fov_y.tan();

        // Compute camera basis vectors (coordinate system)
        // source: tutorial 2 - page 25
        let z = (self.position - self.look_at).normalize();
        let x = self.up.cross(z).normalize();
        let y = z.cross(x).normalize();

        // Transform camera space direction to world space
        let direction_cam = Vector::new(x_i, y_i, -1.0);
        let direction_world = x * direction_cam.x
            + y * direction_cam.y
            + z * direction_cam.z;

        Ray::new(
            self.position,
            direction_world.normalize(),
            0.01,
            f64::INFINITY,
        )
    }
}