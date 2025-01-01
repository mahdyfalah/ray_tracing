use ray_tracing::models::scene::*;
use ray_tracing::models::color::Color;
use ray_tracing::models::point::Point;
use ray_tracing::models::vector::Vector;
use serde_xml_rs::from_str;

#[test]
fn test_parse_scene() {
    let xml_data = r#"
        <scene output_file="example1.png">
            <background_color r="0.0" g="0.0" b="0.0"/>
            <camera>
                <position x="0.0" y="0.0" z="1.0"/>
                <lookat x="0.0" y="0.0" z="-2.5"/>
                <up x="0.0" y="1.0" z="0.0"/>
                <horizontal_fov angle="45"/>
                <resolution horizontal="512" vertical="512"/>
                <max_bounces n="8"/>
            </camera>
            <lights>
                <ambient_light>
                    <color r="1.0" g="1.0" b="1.0"/>
                </ambient_light>
            </lights>
            <surfaces>
                <sphere radius="1.0">
                    <position x="-2.1" y="-2.0" z="-3.0"/>
                    <material_solid>
                        <color r="0.25" g="0.18" b="0.50"/>
                        <phong ka="0.3" kd="0.9" ks="1.0" exponent="200"/>
                        <reflectance r="0.0"/>
                        <transmittance t="0.0"/>
                        <refraction iof="2.3"/>
                    </material_solid>
                </sphere>
            </surfaces>
        </scene>
    "#;

    let scene: Scene = from_str(xml_data).expect("Failed to parse Scene");

    // Output file
    assert_eq!(scene.output_file, "example1.png");

    // Background color
    assert_eq!(scene.background_color, Color { r: 0.0, g: 0.0, b: 0.0 });

    // Camera
    assert_eq!(scene.camera.position, Point { x: 0.0, y: 0.0, z: 1.0 });
    assert_eq!(scene.camera.look_at, Vector { x: 0.0, y: 0.0, z: -2.5 });
    assert_eq!(scene.camera.up, Vector { x: 0.0, y: 1.0, z: 0.0 });
    assert_eq!(scene.camera.horizontal_fov.angle, 45.0);
    assert_eq!(scene.camera.resolution.horizontal, 512);
    assert_eq!(scene.camera.resolution.vertical, 512);
    assert_eq!(scene.camera.max_bounces.n, 8);

    // Lights
    let ambient_light = scene.lights.ambient_light.as_ref().expect("Missing ambient light");
    assert_eq!(ambient_light.color, Color { r: 1.0, g: 1.0, b: 1.0 });

    // Surfaces
    assert_eq!(scene.surfaces.spheres.len(), 1);
    assert_eq!(scene.surfaces.spheres[0].radius, 1.0);
    assert_eq!(
        scene.surfaces.spheres[0].position,
        Point { x: -2.1, y: -2.0, z: -3.0 }
    );
    assert_eq!(
        scene.surfaces.spheres[0].material_solid.color,
        Color { r: 0.25, g: 0.18, b: 0.50 }
    );
}
