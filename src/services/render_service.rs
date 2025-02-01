// use crate::models::scene::Scene;
// use crate::models::ray::Ray;
// use crate::models::intersection::Intersection;
// use crate::models::surface::Surface;
// use crate::models::color::Color;
// use image::{RgbImage, Rgb};
// use std::path::Path;
//
// /// Service to generate a ray traced image from a scene
// pub struct RenderService;
//
// impl RenderService {
//     /// Generates and saves the ray traced image
//     pub fn generate_image(scene: &Scene) {
//         let camera = &scene.camera;
//         let width = camera.resolution.horizontal;
//         let height = camera.resolution.vertical;
//         let max_bounces = scene.camera.max_bounces.n; // Retrieve max bounces
//
//         // Create an empty image
//         let mut img = RgbImage::new(width, height);
//
//         for y in 0..height {
//             for x in 0..width {
//                 let ray = camera.generate_ray(x, y);
//                 let color = Self::trace_ray(&ray, &scene, max_bounces);
//                 let pixel = Rgb([
//                     (color.r * 255.0) as u8,
//                     (color.g * 255.0) as u8,
//                     (color.b * 255.0) as u8,
//                 ]);
//                 img.put_pixel(x, y, pixel);
//             }
//         }
//
//         // Save the image to the specified output file
//         let output_path = Path::new("output").join(&scene.output_file);
//         img.save(output_path).expect("Failed to save the image.");
//         println!("Image saved to: output/{}", scene.output_file);
//     }
//
//     /// source: https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection
//     fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
//         if depth == 0 {
//             return scene.background_color;
//         }
//
//         let mut closest_intersection: Option<Intersection> = None;
//
//         // Find the closest intersection
//         for surface in &scene.surfaces.spheres {
//             if let Some(intersection) = surface.intersect(ray) {
//                 if let Some(closest) = &closest_intersection {
//                     if intersection.t < closest.t {
//                         closest_intersection = Some(intersection);
//                     }
//                 } else {
//                     closest_intersection = Some(intersection);
//                 }
//             }
//         }
//
//         if let Some(intersection) = closest_intersection {
//             let mut final_color = Color::new(0.0, 0.0, 0.0); // Black as initial color
//
//             let material = &intersection.material;
//             let normal = intersection.normal;
//             let view_dir = -ray.direction.normalize(); // View direction
//
//             // Ambient Light
//             if let Some(ambient) = &scene.lights.ambient_light {
//                 let ambient_color = material.color * ambient.color;
//                 final_color += ambient_color * material.phong.ka;
//             }
//
//             // Parallel Light
//             if let Some(parallel) = &scene.lights.parallel_light {
//                 let light_dir = -parallel.direction.normalize();
//
//                 // Cast a shadow ray
//                 let shadow_ray = Ray::new(
//                     intersection.point + normal * 1e-4, // Offset to avoid self-intersections
//                     light_dir,
//                     1e-4,
//                     f64::INFINITY,
//                 );
//
//                 // Check for occlusion
//                 let mut in_shadow = false;
//                 for surface in &scene.surfaces.spheres {
//                     if let Some(_) = surface.intersect(&shadow_ray) {
//                         in_shadow = true; // Object blocks light
//                         break;
//                     }
//                 }
//
//                 if !in_shadow {
//                     // Only calculate lighting if not in shadow
//
//                     // Diffuse Component
//                     let diffuse_intensity = normal.dot(light_dir).max(0.0);
//                     let diffuse_color = material.color * parallel.color * diffuse_intensity;
//
//                     // Specular Component
//                     let reflection_dir =
//                         (normal * (2.0 * normal.dot(light_dir)) - light_dir).normalize();
//                     let specular_intensity = view_dir
//                         .dot(reflection_dir)
//                         .max(0.0)
//                         .powf(material.phong.exponent);
//                     let specular_color = parallel.color * specular_intensity * material.phong.ks;
//
//                     final_color += diffuse_color * material.phong.kd;
//                     final_color += specular_color;
//                 }
//             }
//
//             return final_color;
//         }
//
//         scene.background_color // No intersection
//     }
// }


use crate::models::scene::Scene;
use crate::models::ray::Ray;
use crate::models::intersection::Intersection;
use crate::models::surface::{Surface, SurfaceType};
use crate::models::color::Color;
use image::{RgbImage, Rgb};
use std::path::Path;
use crate::models::lights::{AmbientLight, ParallelLight};
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
                let pixel = Self::color_to_rgb(color);
                img.put_pixel(x, y, pixel);
            }
        }

        Self::save_image(img, scene);
    }

    fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return scene.background_color;
        }

        if let Some(intersection) = Self::find_closest_intersection(ray, scene) {
            Self::calculate_lighting(intersection, ray, scene, depth)
        } else {
            scene.background_color
        }
    }

    /// Finds the closest intersection with any surface in the scene
    fn find_closest_intersection(ray: &Ray, scene: &Scene) -> Option<Intersection> {
        let mut closest = None;

        for surface in &scene.surfaces.surfaces {
            match surface {
                SurfaceType::Sphere(sphere) => {
                    if let Some(intersection) = sphere.intersect(ray) {
                        closest = Self::keep_closest(closest, intersection);
                    }
                },
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

    /// Calculates lighting contributions for a surface intersection
    fn calculate_lighting(intersection: Intersection, ray: &Ray, scene: &Scene, depth: u32) -> Color {
        let material = &intersection.material;
        let normal = intersection.normal;
        let view_dir = -ray.direction.normalize();

        let mut color = Color::new(0.0, 0.0, 0.0);

        // Ambient lighting
        for ambient in &scene.lights.ambient_light {
            color += Self::calculate_ambient(material, ambient);
        }

        // Directional lighting
        for parallel in &scene.lights.parallel_light {
            if !Self::is_in_shadow(&intersection.point, normal, parallel.direction, scene) {
                color += Self::calculate_diffuse(material, parallel, normal);
                color += Self::calculate_specular(material, parallel, normal, view_dir);
            }
        }

        color
    }

    // Helper functions --------------------------------------------------------

    fn keep_closest(current: Option<Intersection>, new: Intersection) -> Option<Intersection> {
        match current {
            Some(closest) if new.t < closest.t => Some(new),
            None => Some(new),
            _ => current,
        }
    }

    fn calculate_ambient(material: &MaterialSolid, ambient: &AmbientLight) -> Color {
        material.color * ambient.color * material.phong.ka
    }

    fn calculate_diffuse(material: &MaterialSolid, light: &ParallelLight, normal: Vector) -> Color {
        let light_dir = -light.direction.normalize();
        let diffuse_intensity = normal.dot(light_dir).max(0.0);
        material.color * light.color * diffuse_intensity * material.phong.kd
    }

    fn calculate_specular(
        material: &MaterialSolid,
        light: &ParallelLight,
        normal: Vector,
        view_dir: Vector,
    ) -> Color {
        let light_dir = -light.direction.normalize();
        let reflection_dir = (normal * (2.0 * normal.dot(light_dir)) - light_dir).normalize();
        let specular_intensity = view_dir.dot(reflection_dir).max(0.0).powf(material.phong.exponent);
        light.color * specular_intensity * material.phong.ks
    }

    fn is_in_shadow(point: &Point, normal: Vector, light_dir: Vector, scene: &Scene) -> bool {
        let shadow_ray = Ray::new(
            *point + normal * 1e-4,
            -light_dir.normalize(),
            1e-4,
            f64::INFINITY,
        );

        scene.surfaces.surfaces.iter().any(|surface| {
            match surface {
                SurfaceType::Sphere(sphere) => sphere.intersect(&shadow_ray).is_some(),
                SurfaceType::Mesh(mesh) => mesh.triangles.iter()
                    .any(|triangle| triangle.intersect(&shadow_ray, &mesh.material_solid).is_some())
            }
        })
    }

    fn color_to_rgb(color: Color) -> Rgb<u8> {
        Rgb([
            (color.r.clamp(0.0, 1.0) * 255.0) as u8,
            (color.g.clamp(0.0, 1.0) * 255.0) as u8,
            (color.b.clamp(0.0, 1.0) * 255.0) as u8,
        ])
    }

    fn save_image(img: RgbImage, scene: &Scene) {
        let output_path = Path::new("output").join(&scene.output_file);
        img.save(output_path).expect("Failed to save image");
        println!("Image saved to: output/{}", scene.output_file);
    }
}