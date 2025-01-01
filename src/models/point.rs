use serde::Deserialize;
use std::ops::{Add, Sub};
use crate::models::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename = "position")]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    /// Creates a new Point
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    /// Computes the dot product of two points (treated as vectors from origin)
    pub fn dot(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of two points (treated as vectors from origin)
    pub fn cross(self, other: Point) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, vector: Vector) -> Self::Output {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, vector: Vector) -> Self::Output {
        Point {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}
