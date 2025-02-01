use serde::Deserialize;
use crate::models::intersection::Intersection;
use crate::models::point::Point;
use crate::models::material_solid::MaterialSolid;
use crate::models::ray::Ray;
use crate::models::surface::Surface;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sphere {
    pub radius: f64,
    pub position: Point,
    pub material_solid: MaterialSolid,
}

impl Surface for Sphere {
    /// Computes if there is intersection between sphere and ray
    /// if so an Intersection object containing the intersection
    /// data is returned
    /// source tutorial page 16, 17
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let t = if t1 > ray.t_min && t1 < ray.t_max {
            t1
        } else if t2 > ray.t_min && t2 < ray.t_max {
            t2
        } else {
            return None;
        };

        let point = ray.at(t);
        let normal = (point - self.position).normalize();

        Some(Intersection {
            t,
            point,
            normal,
            material: self.material_solid.clone(),
        })
    }
}

