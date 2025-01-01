use ray_tracing::models::point::Point; // Adjust path based on your structure
use serde_xml_rs::from_str;

#[test]
fn test_deserialize_point() {
    let xml_data = r#"
        <position x="1.0" y="2.0" z="3.0" />
    "#;

    // Deserialize the XML into a Point object
    let point: Point = from_str(xml_data).expect("Failed to parse Point from XML");

    // Check the values
    assert_eq!(point, Point { x: 1.0, y: 2.0, z: 3.0 });
}
