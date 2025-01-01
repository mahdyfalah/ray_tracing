use ray_tracing::models::lights::*;
use ray_tracing::models::color::Color;
use ray_tracing::models::point::Point;
use ray_tracing::models::vector::Vector;
use serde_xml_rs::from_str;

#[test]
fn test_parse_lights() {
    let xml_data = r#"
        <lights>
            <ambient_light>
                <color r="0.1" g="0.2" b="0.3"/>
            </ambient_light>
            <point_light>
                <color r="0.1" g="0.2" b="0.3"/>
                <position x="1.0" y="2.0" z="3.0"/>
            </point_light>
            <parallel_light>
                <color r="0.1" g="0.2" b="0.3"/>
                <direction x="1.0" y="2.0" z="3.0"/>
            </parallel_light>
            <spot_light>
                <color r="0.1" g="0.2" b="0.3"/>
                <position x="1.0" y="2.0" z="3.0"/>
                <direction x="1.0" y="2.0" z="3.0"/>
                <falloff alpha1="1.0" alpha2="3.0"/>
            </spot_light>
        </lights>
    "#;

    let lights: Lights = from_str(xml_data).expect("Failed to parse Lights");

    // Ambient Light
    assert_eq!(lights.ambient_light.color, Color { r: 0.1, g: 0.2, b: 0.3 });

    // Point Light
    assert_eq!(lights.point_light.color, Color { r: 0.1, g: 0.2, b: 0.3 });
    assert_eq!(lights.point_light.position, Point { x: 1.0, y: 2.0, z: 3.0 });

    // Parallel Light
    assert_eq!(lights.parallel_light.color, Color { r: 0.1, g: 0.2, b: 0.3 });
    assert_eq!(
        lights.parallel_light.direction,
        Vector { x: 1.0, y: 2.0, z: 3.0 }
    );

    // Spotlight
    assert_eq!(lights.spot_light.color, Color { r: 0.1, g: 0.2, b: 0.3 });
    assert_eq!(lights.spot_light.position, Point { x: 1.0, y: 2.0, z: 3.0 });
    assert_eq!(lights.spot_light.direction, Vector { x: 1.0, y: 2.0, z: 3.0 });
    assert_eq!(lights.spot_light.falloff.alpha1, 1.0);
    assert_eq!(lights.spot_light.falloff.alpha2, 3.0);
}
