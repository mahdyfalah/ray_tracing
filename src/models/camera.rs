use serde::Deserialize;
use crate::models::point::Point;
use crate::models::vector::Vector;

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
