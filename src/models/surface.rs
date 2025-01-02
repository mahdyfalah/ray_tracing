use crate::models::ray::Ray;
use crate::models::intersection::Intersection;

/// Defines the behavior for surfaces.
pub trait Surface {
    /// Calculates the intersection with a ray and returns an optional result.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

