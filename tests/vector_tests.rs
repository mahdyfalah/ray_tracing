use ray_tracing::models::vector::Vector;
use serde_xml_rs::from_str;

#[test]
fn test_deserialize_vector() {
    let xml_data = r#"
        <direction x="1.0" y="2.0" z="3.0" />
    "#;

    let dir: Vector = from_str(xml_data).expect("Failed to parse Vector from XML");

    assert_eq!(dir, Vector { x: 1.0, y: 2.0, z: 3.0 });
}