use crate::models::scene::Scene;
use crate::models::ray::Ray;
use crate::models::intersection::Intersection;
use crate::models::surface::{Surface, SurfaceType};
use crate::models::color::Color;
use image::{RgbImage, Rgb};
use std::path::Path;
use crate::models::material::MaterialSolid;
use crate::models::point::Point;
use crate::models::vector::Vector;

/// Service to generate a ray traced image from a scene
pub struct RenderService;

impl RenderService {
    /// Generates and saves the ray traced image
    pub fn generate_image(scene: &Scene) {
        let camera = &scene.camera;
        let width = camera.resolution.horizontal;
        let height = camera.resolution.vertical;
        let max_bounces = scene.camera.max_bounces.n;

        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = camera.generate_ray(x, y);
                let color = Self::trace_ray(&ray, scene, max_bounces);
                img.put_pixel(x, y, Self::color_to_rgb(color));
            }
        }

        Self::save_image(img, scene);
    }

    /// Casts a ray into the scene and returns the color from the closest intersection,
    /// or the background color if there is none.
    fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return scene.background_color;
        }

        if let Some(intersection) = Self::find_closest_intersection(ray, scene) {
            Self::calculate_lighting(&intersection, ray, scene, depth)
        } else {
            scene.background_color
        }
    }

    /// Finds the closest intersection of a ray with any surface in the scene.
    fn find_closest_intersection(ray: &Ray, scene: &Scene) -> Option<Intersection> {
        let mut closest = None;

        for surface in &scene.surfaces.surfaces {
            match surface {
                SurfaceType::Sphere(sphere) => {
                    if let Some(intersection) = sphere.intersect(ray) {
                        closest = Self::keep_closest(closest, intersection);
                    }
                }
                SurfaceType::Mesh(mesh) => {
                    for triangle in &mesh.triangles {
                        if let Some(intersection) = triangle.intersect(ray, &mesh.material_solid) {
                            closest = Self::keep_closest(closest, intersection);
                        }
                    }
                }
            }
        }

        closest
    }

    /// Calculates lighting at an intersection. (Note that reflection/refraction
    /// can be added later by making this function recursive.)
    fn calculate_lighting(intersection: &Intersection, ray: &Ray, scene: &Scene, _depth: u32) -> Color {
        let material = &intersection.material;
        let normal = intersection.normal;
        let view_dir = -ray.direction.normalize();
        let point = intersection.point;

        let mut color = Color::new(0.0, 0.0, 0.0);

        // Ambient lighting (same for all surfaces)
        for ambient in &scene.lights.ambient_light {
            color += material.color * ambient.color * material.phong.ka;
        }

        // Parallel (directional) lights: no distance attenuation (max_distance = INFINITY)
        for parallel in &scene.lights.parallel_light {
            let light_dir = -parallel.direction.normalize();
            if !Self::is_in_shadow(&point, normal, light_dir, f64::INFINITY, scene) {
                color += Self::calc_diffuse(material, parallel.color, light_dir, normal, 1.0);
                color += Self::calc_specular(material, parallel.color, light_dir, normal, view_dir, 1.0);
            }
        }

        // Point lights: include attenuation and intensity boost
        let light_intensity = 1.7; // Adjust this as needed
        for point_light in &scene.lights.point_light {
            let to_light = point_light.position - point;
            let distance = to_light.length();
            let light_dir = to_light.normalize();

            if !Self::is_in_shadow(&point, normal, light_dir, distance, scene) {
                // Soft attenuation: adjust the constants to tune falloff behavior
                let attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
                let factor = attenuation * light_intensity;

                color += Self::calc_diffuse(material, point_light.color, light_dir, normal, factor);
                color += Self::calc_specular(material, point_light.color, light_dir, normal, view_dir, factor);
            }
        }

        color
    }

    /// Returns true if an object is between the point and the light.
    /// For parallel lights, use max_distance = f64::INFINITY.
    fn is_in_shadow(point: &Point, normal: Vector, light_dir: Vector, max_distance: f64, scene: &Scene) -> bool {
        let shadow_ray = Ray::new(
            *point + normal * 1e-4, // Offset to avoid self-intersection
            light_dir.normalize(),
            1e-4,
            max_distance - 1e-4,    // Stop just before the light (if finite)
        );

        scene.surfaces.surfaces.iter().any(|surface| match surface {
            SurfaceType::Sphere(sphere) => sphere.intersect(&shadow_ray).is_some(),
            SurfaceType::Mesh(mesh) => mesh.triangles.iter()
                .any(|triangle| triangle.intersect(&shadow_ray, &mesh.material_solid).is_some()),
        })
    }

    /// Calculates the diffuse contribution from a light.
    fn calc_diffuse(material: &MaterialSolid, light_color: Color, light_dir: Vector, normal: Vector, factor: f64) -> Color {
        let diffuse_intensity = normal.dot(light_dir).max(0.0);
        material.color * light_color * diffuse_intensity * material.phong.kd * factor
    }

    /// Calculates the specular contribution from a light.
    fn calc_specular(
        material: &MaterialSolid,
        light_color: Color,
        light_dir: Vector,
        normal: Vector,
        view_dir: Vector,
        factor: f64,
    ) -> Color {
        let reflection_dir = (normal * (2.0 * normal.dot(light_dir)) - light_dir).normalize();
        let specular_intensity = view_dir.dot(reflection_dir).max(0.0).powf(material.phong.exponent);
        light_color * specular_intensity * material.phong.ks * factor
    }

    /// Keeps the closest intersection (the one with the smallest t value).
    fn keep_closest(current: Option<Intersection>, new: Intersection) -> Option<Intersection> {
        match current {
            Some(closest) if new.t < closest.t => Some(new),
            None => Some(new),
            _ => current,
        }
    }

    /// Converts a Color (with components in [0,1]) to an 8-bit RGB pixel.
    fn color_to_rgb(color: Color) -> Rgb<u8> {
        Rgb([
            (color.r.clamp(0.0, 1.0) * 255.0) as u8,
            (color.g.clamp(0.0, 1.0) * 255.0) as u8,
            (color.b.clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }

    /// Saves the final image to the specified output file.
    fn save_image(img: RgbImage, scene: &Scene) {
        let output_path = Path::new("output").join(&scene.output_file);
        img.save(&output_path).expect("Failed to save image");
        println!("Image saved to: {}", output_path.display());
    }
}
