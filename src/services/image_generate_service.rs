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
    // /// Generates and saves the raytraced image
    // pub fn generate_image(scene: &Scene) {
    //     let camera = &scene.camera;
    //     let width = camera.resolution.horizontal;
    //     let height = camera.resolution.vertical;
    //
    //     // Create an empty image
    //     let mut img = RgbImage::new(width, height);
    //
    //     for y in 0..height {
    //         for x in 0..width {
    //             // Generate ray for the current pixel
    //             let ray = camera.generate_ray(x, y);
    //
    //             // Trace the ray
    //             let color = Self::trace_ray(&ray, &scene);
    //
    //             // Convert color to RGB format and write it to the image
    //             let pixel = Rgb([
    //                 (color.r * 255.0) as u8,
    //                 (color.g * 255.0) as u8,
    //                 (color.b * 255.0) as u8,
    //             ]);
    //             img.put_pixel(x, y, pixel);
    //         }
    //     }
    //
    //     // Save the image to the specified output file
    //     let output_path = Path::new(&scene.output_file);
    //     img.save(output_path).expect("Failed to save the image.");
    //     println!("Image saved to: {}", scene.output_file);
    // }
    //
    // /// Traces a ray and determines the color at the intersection
    // fn trace_ray(ray: &Ray, scene: &Scene) -> Color {
    //     let mut closest_intersection: Option<Intersection> = None;
    //
    //     // Check intersections with all surfaces
    //     for surface in &scene.surfaces.spheres {
    //         if let Some(intersection) = surface.intersect(ray) {
    //             if let Some(closest) = &closest_intersection {
    //                 if intersection.t < closest.t {
    //                     closest_intersection = Some(intersection);
    //                 }
    //             } else {
    //                 closest_intersection = Some(intersection);
    //             }
    //         }
    //     }
    //
    //     // Return the color of the closest intersection or the background color
    //     if let Some(intersection) = closest_intersection {
    //         return intersection.material.color; // Return material color (basic shading)
    //     }
    //
    //     // No intersection: return background color
    //     scene.background_color
    // }

    /// Generates and saves the raytraced image
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
                let color = Self::trace_ray(&ray, &scene, max_bounces); // Pass max_bounces
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

    /// No Phong
    // fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    //     // Base case: if maximum bounces exceeded, return background color
    //     if depth == 0 {
    //         return scene.background_color;
    //     }
    //
    //     let mut closest_intersection: Option<Intersection> = None;
    //
    //     // Find the closest intersection
    //     for surface in &scene.surfaces.spheres {
    //         if let Some(intersection) = surface.intersect(ray) {
    //             if let Some(closest) = &closest_intersection {
    //                 if intersection.t < closest.t {
    //                     closest_intersection = Some(intersection);
    //                 }
    //             } else {
    //                 closest_intersection = Some(intersection);
    //             }
    //         }
    //     }
    //
    //     // If no intersection, return background color
    //     if closest_intersection.is_none() {
    //         return scene.background_color;
    //     }
    //
    //     let intersection = closest_intersection.unwrap();
    //     // let mut final_color = Color::default(); // Initialize color
    //     let mut final_color = Color::new(0.0, 0.0, 0.0); // Black as initial color
    //
    //
    //     // Illuminate the point using all light sources
    //     if let Some(lights) = &scene.lights.ambient_light {
    //         let light_color = lights.color;
    //         final_color = final_color + intersection.material.color * light_color;
    //     }
    //
    //     // Reflective and recursive handling
    //     let reflectance = intersection.material.reflectance.r;
    //     if reflectance > 0.0 {
    //         let reflected_ray = ray.reflect(intersection.point, intersection.normal);
    //         let reflected_color = Self::trace_ray(&reflected_ray, scene, depth - 1);
    //         final_color = final_color * (1.0 - reflectance) + reflected_color * reflectance;
    //     }
    //
    //     // Return the final computed color
    //     final_color
    // }

    /// phong
    // fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    //     // Stop recursion at max depth
    //     if depth == 0 {
    //         return scene.background_color;
    //     }
    //
    //     let mut closest_intersection: Option<Intersection> = None;
    //
    //     // Find the closest intersection
    //     for surface in &scene.surfaces.spheres {
    //         if let Some(intersection) = surface.intersect(ray) {
    //             if let Some(closest) = &closest_intersection {
    //                 if intersection.t < closest.t {
    //                     closest_intersection = Some(intersection);
    //                 }
    //             } else {
    //                 closest_intersection = Some(intersection);
    //             }
    //         }
    //     }
    //
    //     // No intersection: return background color
    //     if let Some(intersection) = closest_intersection {
    //         let mut final_color = Color::new(0.0, 0.0, 0.0); // Black as initial color
    //
    //         let material = &intersection.material;
    //         let normal = intersection.normal;
    //         let view_dir = -ray.direction.normalize(); // Viewing direction
    //
    //         // -------------------------
    //         // Ambient Light Contribution
    //         // -------------------------
    //         if let Some(ambient) = &scene.lights.ambient_light {
    //             let ambient_color = material.color * ambient.color;
    //             final_color += ambient_color * material.phong.ka; // Apply ambient intensity
    //         }
    //
    //         // -------------------------
    //         // Parallel Light Contribution
    //         // -------------------------
    //         if let Some(parallel) = &scene.lights.parallel_light {
    //             let light_dir = -parallel.direction.normalize(); // Light direction
    //
    //             // Diffuse Component
    //             let diffuse_intensity = normal.dot(light_dir).max(0.0); // Clamp to [0, 1]
    //             let diffuse_color = material.color * parallel.color * diffuse_intensity;
    //
    //             // Specular Component
    //             let reflection_dir = (normal * (2.0 * normal.dot(light_dir)) - light_dir).normalize();
    //             let specular_intensity = view_dir.dot(reflection_dir).max(0.0).powf(material.phong.exponent);
    //             let specular_color = parallel.color * specular_intensity * material.phong.ks;
    //
    //             // Add both diffuse and specular to final color
    //             final_color += diffuse_color * material.phong.kd;
    //             final_color += specular_color;
    //         }
    //
    //         // -------------------------
    //         // Recursive Reflection
    //         // -------------------------
    //         let reflectance = material.reflectance.r;
    //         if reflectance > 0.0 {
    //             let reflected_ray = ray.reflect(intersection.point, intersection.normal);
    //             let reflected_color = Self::trace_ray(&reflected_ray, scene, depth - 1);
    //             final_color = final_color * (1.0 - reflectance) + reflected_color * reflectance;
    //         }
    //
    //         return final_color; // Return computed color
    //     }
    //
    //     scene.background_color // No intersection
    // }

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

            // -------------------------
            // Ambient Light Contribution
            // -------------------------
            if let Some(ambient) = &scene.lights.ambient_light {
                let ambient_color = material.color * ambient.color;
                final_color += ambient_color * material.phong.ka;
            }

            // -------------------------
            // Parallel Light Contribution
            // -------------------------
            if let Some(parallel) = &scene.lights.parallel_light {
                let light_dir = -parallel.direction.normalize(); // Light direction

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

                    // Add both diffuse and specular to final color
                    final_color += diffuse_color * material.phong.kd;
                    final_color += specular_color;
                }
            }

            // -------------------------
            // Recursive Reflection
            // -------------------------
            let reflectance = material.reflectance.r;
            if reflectance > 0.0 {
                let reflected_ray = ray.reflect(intersection.point, intersection.normal);
                let reflected_color = Self::trace_ray(&reflected_ray, scene, depth - 1);
                final_color = final_color * (1.0 - reflectance) + reflected_color * reflectance;
            }

            return final_color; // Return computed color
        }

        scene.background_color // No intersection
    }


}
