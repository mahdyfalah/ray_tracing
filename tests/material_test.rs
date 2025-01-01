use ray_tracing::models::material::*;
use ray_tracing::models::color::Color;
use serde_xml_rs::from_str;

#[test]
fn test_parse_material_solid() {
    let xml_data = r#"
        <material_solid>
            <color r="0.25" g="0.18" b="0.50"/>
            <phong ka="0.3" kd="0.9" ks="1.0" exponent="200"/>
            <reflectance r="0.0"/>
            <transmittance t="0.0"/>
            <refraction iof="2.3"/>
        </material_solid>
    "#;

    let material: MaterialSolid = from_str(xml_data).expect("Failed to parse MaterialSolid");

    // Color
    assert_eq!(material.color, Color { r: 0.25, g: 0.18, b: 0.50 });

    // Phong properties
    assert_eq!(
        material.phong,
        Phong {
            ka: 0.3,
            kd: 0.9,
            ks: 1.0,
            exponent: 200.0
        }
    );

    // Reflectance
    assert_eq!(material.reflectance.r, 0.0);

    // Transmittance
    assert_eq!(material.transmittance.t, 0.0);

    // Refraction
    assert_eq!(material.refraction.iof, 2.3);
}
