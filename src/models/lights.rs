use serde::Deserialize;
use crate::models::color::Color;
use crate::models::point::Point;
use crate::models::vector::Vector;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Lights {
    #[serde(default)]
    pub ambient_light: Vec<AmbientLight>,
    #[serde(default)]
    pub point_light: Vec<PointLight>,
    #[serde(default)]
    pub parallel_light: Vec<ParallelLight>,
    #[serde(default)]
    pub spot_light: Vec<SpotLight>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AmbientLight {
    pub color: Color,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PointLight {
    pub color: Color,
    pub position: Point,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ParallelLight {
    pub color: Color,
    pub direction: Vector,
}

#[derive(Debug, Deserialize, PartialEq)]
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