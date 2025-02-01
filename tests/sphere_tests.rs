use ray_tracing::models::sphere::*;
use ray_tracing::models::point::Point;
use ray_tracing::models::material_solid::*;
use ray_tracing::models::color::Color;
use serde_xml_rs::from_str;

#[test]
fn test_parse_sphere() {
    let xml_data = r#"
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
    "#;

    let sphere: Sphere = from_str(xml_data).expect("Failed to parse Sphere");

    // Radius
    assert_eq!(sphere.radius, 1.0);

    // Position
    assert_eq!(sphere.position, Point { x: 0.0, y: 0.0, z: -3.0 });

    // Material
    assert_eq!(sphere.material_solid.color, Color { r: 0.95, g: 0.63, b: 0.01 });
    assert_eq!(
        sphere.material_solid.phong,
        Phong {
            ka: 0.3,
            kd: 0.9,
            ks: 1.0,
            exponent: 200.0
        }
    );
    assert_eq!(sphere.material_solid.reflectance.r, 0.0);
    assert_eq!(sphere.material_solid.transmittance.t, 0.0);
    assert_eq!(sphere.material_solid.refraction.iof, 2.3);
}
