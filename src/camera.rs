use crate::point::Point;
use crate::vector::Vector;

// src/camera.rs
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Point,
    pub look_at: Point,
    pub up: Vector,
    pub horizontal_fov: f64, // In degrees
    pub resolution: (u32, u32), // (horizontal, vertical)
    pub max_bounces: u32,
}

impl Camera {
    /// Creates a new Camera with specified parameters
    pub fn new(
        position: Point,
        look_at: Point,
        up: Vector,
        horizontal_fov: f64,
        resolution: (u32, u32),
        max_bounces: u32,
    ) -> Self {
        Camera {
            position,
            look_at,
            up,
            horizontal_fov,
            resolution,
            max_bounces,
        }
    }
}
