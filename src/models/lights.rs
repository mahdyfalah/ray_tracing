use serde::Deserialize;
use crate::models::color::Color;
use crate::models::point::Point;
use crate::models::vector::Vector;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Lights {
    pub ambient_light: AmbientLight,
    pub point_light: PointLight,
    pub parallel_light: ParallelLight,
    pub spot_light: SpotLight,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ambient_light")]
pub struct AmbientLight {
    pub color: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "point_light")]
pub struct PointLight {
    pub color: Color,
    pub position: Point,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "parallel_light")]
pub struct ParallelLight {
    pub color: Color,
    pub direction: Vector,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "spot_light")]
pub struct SpotLight {
    pub color: Color,
    pub position: Point,
    pub direction: Vector,
    pub falloff: Falloff,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Falloff {
    pub alpha1: f64,
    pub alpha2: f64,
}
