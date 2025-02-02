use crate::models::intersection::Intersection;
use crate::models::ray::Ray;
use crate::models::vector::Vector;
use crate::models::point::Point;
use crate::models::material::{Material, MaterialSolid};

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub normal: Vector,
}

impl Triangle {
    pub fn new(
        vertices: &[Point],
        normals: &[Vector],
        v_indices: [usize; 3],  // 0-based vertex indices
        n_index: usize          // 0-based normal index
    ) -> Self {
        Triangle {
            v0: vertices[v_indices[0]],
            v1: vertices[v_indices[1]],
            v2: vertices[v_indices[2]],
            normal: normals[n_index],
        }
    }

    /// Möller–Trumbore intersection algorithm implementation
    pub fn intersect(&self, ray: &Ray, material: &Material) -> Option<Intersection> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let h = ray.direction.cross(e2);  // Intermediate vector
        let a = e1.dot(h);

        // Early out if ray is parallel to triangle
        if a.abs() < 1e-8 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(h);

        // Check barycentric coordinate u
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);

        // Check barycentric coordinate v
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Compute t
        let t = f * e2.dot(q);

        // Check valid t range
        if t < ray.t_min || t > ray.t_max {
            return None;
        }

        // Compute intersection point and normal
        let point = ray.at(t);
        let normal = self.normal;

        Some(Intersection {
            t,
            point,
            normal,
            material: material.clone(),  // Use the material from the mesh
        })
    }
}