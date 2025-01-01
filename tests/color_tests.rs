use ray_tracing::models::color::Color; // Adjust based on your project structure
use serde_xml_rs::from_str;

#[test]
fn test_deserialize_color() {
    // Test for <color>
    let xml_data = r#"
        <color r="0.1" g="0.2" b="0.3" />
    "#;

    let color: Color = from_str(xml_data).expect("Failed to parse Color from XML");
    assert_eq!(color, Color { r: 0.1, g: 0.2, b: 0.3 });

    let xml_data = r#"
        <background_color r="1.0" g="0.0" b="0.0" />
    "#;

    let background_color: Color = from_str(xml_data).expect("Failed to parse Background Color");
    assert_eq!(background_color, Color { r: 1.0, g: 0.0, b: 0.0 });
}
