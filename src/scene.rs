use crate::camera::Camera;

// src/scene.rs
#[derive(Debug, Clone)]
pub struct Scene {
    pub background_color: (f64, f64, f64), // RGB values in [0, 1]
    pub camera: Camera,
}

impl Scene {
    /// Creates a new Scene with specified background color and camera
    pub fn new(background_color: (f64, f64, f64), camera: Camera) -> Self {
        Scene {
            background_color,
            camera,
        }
    }
}