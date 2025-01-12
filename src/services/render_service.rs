use crate::models::scene::Scene;
use crate::models::ray::Ray;
use crate::models::intersection::Intersection;
use crate::models::surface::Surface;
use crate::models::color::Color;
use image::{RgbImage, Rgb};
use std::path::Path;

/// Service to generate a ray traced image from a scene
pub struct RenderService;

impl RenderService {
    /// Generates and saves the ray traced image
    pub fn generate_image(scene: &Scene) {
        let camera = &scene.camera;
        let width = camera.resolution.horizontal;
        let height = camera.resolution.vertical;
        let max_bounces = scene.camera.max_bounces.n; // Retrieve max bounces

        // Create an empty image
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let ray = camera.generate_ray(x, y);
                let color = Self::trace_ray(&ray, &scene, max_bounces);
                let pixel = Rgb([
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ]);
                img.put_pixel(x, y, pixel);
            }
        }

        // Save the image to the specified output file
        let output_path = Path::new("output").join(&scene.output_file);
        img.save(output_path).expect("Failed to save the image.");
        println!("Image saved to: output/{}", scene.output_file);
    }

    /// source: https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection
    fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return scene.background_color;
        }

        let mut closest_intersection: Option<Intersection> = None;

        // Find the closest intersection
        for surface in &scene.surfaces.spheres {
            if let Some(intersection) = surface.intersect(ray) {
                if let Some(closest) = &closest_intersection {
                    if intersection.t < closest.t {
                        closest_intersection = Some(intersection);
                    }
                } else {
                    closest_intersection = Some(intersection);
                }
            }
        }

        if let Some(intersection) = closest_intersection {
            let mut final_color = Color::new(0.0, 0.0, 0.0); // Black as initial color

            let material = &intersection.material;
            let normal = intersection.normal;
            let view_dir = -ray.direction.normalize(); // View direction

            // Ambient Light
            if let Some(ambient) = &scene.lights.ambient_light {
                let ambient_color = material.color * ambient.color;
                final_color += ambient_color * material.phong.ka;
            }

            // Parallel Light
            if let Some(parallel) = &scene.lights.parallel_light {
                let light_dir = -parallel.direction.normalize();

                // Cast a shadow ray
                let shadow_ray = Ray::new(
                    intersection.point + normal * 1e-4, // Offset to avoid self-intersections
                    light_dir,
                    1e-4,
                    f64::INFINITY,
                );

                // Check for occlusion
                let mut in_shadow = false;
                for surface in &scene.surfaces.spheres {
                    if let Some(_) = surface.intersect(&shadow_ray) {
                        in_shadow = true; // Object blocks light
                        break;
                    }
                }

                if !in_shadow {
                    // Only calculate lighting if not in shadow

                    // Diffuse Component
                    let diffuse_intensity = normal.dot(light_dir).max(0.0);
                    let diffuse_color = material.color * parallel.color * diffuse_intensity;

                    // Specular Component
                    let reflection_dir =
                        (normal * (2.0 * normal.dot(light_dir)) - light_dir).normalize();
                    let specular_intensity = view_dir
                        .dot(reflection_dir)
                        .max(0.0)
                        .powf(material.phong.exponent);
                    let specular_color = parallel.color * specular_intensity * material.phong.ks;

                    final_color += diffuse_color * material.phong.kd;
                    final_color += specular_color;
                }
            }

            return final_color;
        }

        scene.background_color // No intersection
    }
}
