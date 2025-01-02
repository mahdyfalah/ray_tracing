use ray_tracing::models::camera::Camera;
use ray_tracing::models::point::Point;
use ray_tracing::models::vector::Vector;
use ray_tracing::models::camera::{Fov, Resolution, MaxBounces};
use serde_xml_rs::from_str;

#[test]
fn test_parse_camera() {
    let xml_data = r#"
        <camera>
            <position x="1.0" y="-2.0E-10" z="-3"/>
            <lookat x="1.0" y="2.0" z="3.0"/>
            <up x="0.0" y="1.0" z="0.0"/>
            <horizontal_fov angle="90"/>
            <resolution horizontal="1920" vertical="1080"/>
            <max_bounces n="100"/>
        </camera>
    "#;

    let camera: Camera = from_str(xml_data).expect("Failed to parse Camera");

    // Assertions
    assert_eq!(camera.position, Point { x: 1.0, y: -2.0E-10, z: -3.0 });
    assert_eq!(camera.look_at, Vector { x: 1.0, y: 2.0, z: 3.0 });
    assert_eq!(camera.up, Vector { x: 0.0, y: 1.0, z: 0.0 });
    assert_eq!(camera.horizontal_fov, Fov { angle: 90.0 });
    assert_eq!(camera.resolution, Resolution { horizontal: 1920, vertical: 1080 });
    assert_eq!(camera.max_bounces, MaxBounces { n: 100 });
}

// use ray_tracing::models::ray::Ray;
//
// #[test]
// fn test_generate_ray_center_pixel() {
//     let camera = Camera {
//         position: Point::new(0.0, 0.0, 0.0),
//         look_at: Vector::new(0.0, 0.0, -1.0),
//         up: Vector::new(0.0, 1.0, 0.0),
//         horizontal_fov: Fov { angle: 45.0 },
//         resolution: Resolution { horizontal: 512, vertical: 512 },
//         max_bounces: MaxBounces { n: 8 },
//     };
//
//     // Generate ray for center pixel (256, 256)
//     let ray: Ray = camera.generate_ray(256, 256);
//
//     // Check origin
//     assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
//
//     // Check direction approximately (should point straight forward)
//     let expected_dir = Vector::new(0.0, 0.0, -1.0).normalize();
//     let diff = ray.direction - expected_dir;
//     assert!(diff.length() < 1e-6, "Direction not normalized properly!");
// }
//
// #[test]
// fn test_generate_ray_top_left_pixel() {
//     let camera = Camera {
//         position: Point::new(0.0, 0.0, 0.0),
//         look_at: Vector::new(0.0, 0.0, -1.0),
//         up: Vector::new(0.0, 1.0, 0.0),
//         horizontal_fov: Fov { angle: 45.0 },
//         resolution: Resolution { horizontal: 512, vertical: 512 },
//         max_bounces: MaxBounces { n: 8 },
//     };
//
//     // Generate ray for top-left pixel (0, 0)
//     let ray: Ray = camera.generate_ray(0, 0);
//
//     // Check origin
//     assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
//
//     // Check direction approximately
//     let expected_dir = Vector::new(-0.4142, 0.4142, -1.0).normalize(); // Based on FOV calculations
//     let diff = ray.direction - expected_dir;
//     assert!(diff.length() < 1e-6, "Direction for top-left pixel is incorrect!");
// }
//
// #[test]
// fn test_generate_ray_bottom_right_pixel() {
//     let camera = Camera {
//         position: Point::new(0.0, 0.0, 0.0),
//         look_at: Vector::new(0.0, 0.0, -1.0),
//         up: Vector::new(0.0, 1.0, 0.0),
//         horizontal_fov: Fov { angle: 45.0 },
//         resolution: Resolution { horizontal: 512, vertical: 512 },
//         max_bounces: MaxBounces { n: 8 },
//     };
//
//     // Generate ray for bottom-right pixel (511, 511)
//     let ray: Ray = camera.generate_ray(511, 511);
//
//     // Check origin
//     assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
//
//     // Check direction approximately
//     let expected_dir = Vector::new(0.4142, -0.4142, -1.0).normalize(); // Based on FOV calculations
//     let diff = ray.direction - expected_dir;
//     assert!(diff.length() < 1e-6, "Direction for bottom-right pixel is incorrect!");
// }
