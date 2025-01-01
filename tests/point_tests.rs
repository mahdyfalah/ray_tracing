use ray_tracing::models::point::Point;
use serde_xml_rs::from_str;

#[test]
fn test_deserialize_point() {
    let xml_data = r#"
        <position x="1.0" y="2.0" z="3.0" />
    "#;

    let position: Point = from_str(xml_data).expect("Failed to parse Point from XML");

    assert_eq!(position, Point { x: 1.0, y: 2.0, z: 3.0 });
}
