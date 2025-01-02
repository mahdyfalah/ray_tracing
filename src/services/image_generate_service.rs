use crate::models::scene::Scene;
use crate::models::ray::Ray;
use crate::models::intersection::Intersection;
use crate::models::surface::Surface;
use crate::models::color::Color;
use image::{RgbImage, Rgb};
use std::path::Path;

/// Service to generate a raytraced image from a scene
pub struct ImageGenerateService;

impl ImageGenerateService {
    /// Generates and saves the raytraced image
    pub fn generate_image(scene: &Scene) {
        let camera = &scene.camera;
        let width = camera.resolution.horizontal;
        let height = camera.resolution.vertical;

        // Create an empty image
        let mut img = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                // Generate ray for the current pixel
                let ray = camera.generate_ray(x, y);

                // Trace the ray
                let color = Self::trace_ray(&ray, &scene);

                // Convert color to RGB format and write it to the image
                let pixel = Rgb([
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                ]);
                img.put_pixel(x, y, pixel);
            }
        }

        // Save the image to the specified output file
        let output_path = Path::new(&scene.output_file);
        img.save(output_path).expect("Failed to save the image.");
        println!("Image saved to: {}", scene.output_file);
    }

    /// Traces a ray and determines the color at the intersection
    fn trace_ray(ray: &Ray, scene: &Scene) -> Color {
        let mut closest_intersection: Option<Intersection> = None;

        // Check intersections with all surfaces
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

        // Return the color of the closest intersection or the background color
        if let Some(intersection) = closest_intersection {
            return intersection.material.color; // Return material color (basic shading)
        }

        // No intersection: return background color
        scene.background_color
    }
}
