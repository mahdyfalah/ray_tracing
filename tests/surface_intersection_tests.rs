use ray_tracing::models::sphere::Sphere;
use ray_tracing::models::material_solid::{MaterialSolid, Phong, Reflectance, Transmittance, Refraction};
use ray_tracing::models::ray::Ray;
use ray_tracing::models::point::Point;
use ray_tracing::models::vector::Vector;
use ray_tracing::models::color::Color;
use ray_tracing::models::surface::Surface;

fn create_test_material() -> MaterialSolid {
    MaterialSolid {
        color: Color { r: 1.0, g: 0.0, b: 0.0 },
        phong: Phong {
            ka: 0.3,
            kd: 0.7,
            ks: 1.0,
            exponent: 32.0,
        },
        reflectance: Reflectance { r: 0.5 },
        transmittance: Transmittance { t: 0.0 },
        refraction: Refraction { iof: 1.0 },
    }
}

#[test]
fn test_sphere_intersection_hit() {
    let sphere = Sphere {
        radius: 1.0,
        position: Point::new(0.0, 0.0, -5.0),
        material_solid: create_test_material(),
    };

    let ray = Ray::new(
        Point::new(0.0, 0.0, 0.0), // Origin
        Vector::new(0.0, 0.0, -1.0), // Direction
        0.01,
        f64::INFINITY,
    );

    let intersection = sphere.intersect(&ray);

    assert!(intersection.is_some(), "Ray should intersect the sphere");
    let result = intersection.unwrap();

    // Check intersection distance
    assert!((result.t - 4.0).abs() < 1e-6, "Incorrect intersection distance");

    // Check intersection point
    assert_eq!(result.point, Point::new(0.0, 0.0, -4.0), "Incorrect intersection point");

    // Check normal
    assert_eq!(
        result.normal,
        Vector::new(0.0, 0.0, 1.0),
        "Incorrect normal at intersection"
    );

    // Check material
    assert_eq!(
        result.material.color,
        Color { r: 1.0, g: 0.0, b: 0.0 },
        "Incorrect material color"
    );
}

#[test]
fn test_sphere_intersection_miss() {
    let sphere = Sphere {
        radius: 1.0,
        position: Point::new(0.0, 0.0, -5.0),
        material_solid: create_test_material(),
    };

    let ray = Ray::new(
        Point::new(2.0, 2.0, 0.0), // Offset origin
        Vector::new(0.0, 0.0, -1.0), // Direction
        0.01,
        f64::INFINITY,
    );

    let intersection = sphere.intersect(&ray);
    assert!(intersection.is_none(), "Ray should miss the sphere");
}

#[test]
fn test_sphere_intersection_inside() {
    let sphere = Sphere {
        radius: 1.0,
        position: Point::new(0.0, 0.0, -1.0),
        material_solid: create_test_material(),
    };

    let ray = Ray::new(
        Point::new(0.0, 0.0, -1.0), // Inside the sphere
        Vector::new(0.0, 0.0, -1.0), // Direction
        0.01,
        f64::INFINITY,
    );

    let intersection = sphere.intersect(&ray);

    assert!(intersection.is_some(), "Ray should intersect the sphere from inside");
    let result = intersection.unwrap();

    // Check intersection distance (exiting the sphere)
    assert!((result.t - 1.0).abs() < 1e-6, "Incorrect intersection distance");
}

#[test]
fn test_sphere_intersection_behind_ray() {
    let sphere = Sphere {
        radius: 1.0,
        position: Point::new(0.0, 0.0, 5.0),
        material_solid: create_test_material(),
    };

    let ray = Ray::new(
        Point::new(0.0, 0.0, 0.0), // Origin
        Vector::new(0.0, 0.0, -1.0), // Direction away from sphere
        0.01,
        f64::INFINITY,
    );

    let intersection = sphere.intersect(&ray);
    assert!(intersection.is_none(), "Ray should miss the sphere (behind)");
}
