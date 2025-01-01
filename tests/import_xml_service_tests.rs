use ray_tracing::models::scene::Scene;
use serde_xml_rs::from_str;

#[test]
fn test_parse_scene_from_file() {
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
                    <position x="0.0" y="0.0" z="-3.0"/>
                    <material_solid>
                        <color r="0.95" g="0.63" b="0.01"/>
                        <phong ka="0.3" kd="0.9" ks="1.0" exponent="200"/>
                        <reflectance r="0.0"/>
                        <transmittance t="0.0"/>
                        <refraction iof="2.3"/>
                    </material_solid>
                </sphere>
            </surfaces>
        </scene>
    "#;

    // Parse the XML string directly
    let scene: Scene = from_str(xml_data).expect("Failed to parse scene from string");

    // Assertions
    assert_eq!(scene.output_file, "example1.png");
    assert_eq!(scene.camera.resolution.horizontal, 512);
    assert_eq!(scene.camera.resolution.vertical, 512);
    assert_eq!(scene.surfaces.spheres.len(), 1);
}
