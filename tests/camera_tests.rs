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
