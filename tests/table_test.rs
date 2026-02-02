use cdx_file_rs::cdx::table::Table;
use cdx_file_rs::cdx::values::Rectangle;
use cdx_file_rs::cdx_parse_impl::tagged_object::TaggedObject;
use cdx_file_rs::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};

#[test]
fn test_table_creation() {
    let table = Table::new(1);
    assert_eq!(table.id, 1);
    assert!(table.bounding_box.is_none());
    assert!(table.z_order.is_none());
    assert!(table.visible.is_none());
}

#[test]
fn test_table_parsing() {
    // Create a raw Table object with some properties
    let mut raw = RawCdxObject {
        tag: 0x8016, // CDXOBJ_TABLE
        id: 42,
        properties: vec![],
        children: vec![],
    };

    // Add a bounding box property
    let bbox = Rectangle {
        top: 100.0,
        left: 50.0,
        bottom: 200.0,
        right: 150.0,
    };
    
    // Encode Rectangle properly using BinaryCodec
    use cdx_file_rs::cdx::binary_codec::BinaryCodec;
    let bbox_bytes = bbox.encode().expect("Failed to encode rectangle");
    
    raw.properties.push(RawCdxProperty {
        tag: 0x0204, // CDXPROP_BOUNDING_BOX
        value: bbox_bytes,
    });

    // Parse the raw object
    let table = Table::from_raw(raw.clone()).expect("Failed to parse Table");
    
    assert_eq!(table.id, 42);
    assert!(table.bounding_box.is_some());
    
    let bbox = table.bounding_box.unwrap();
    assert_eq!(bbox.top, 100.0);
    assert_eq!(bbox.left, 50.0);
    assert_eq!(bbox.bottom, 200.0);
    assert_eq!(bbox.right, 150.0);
}

#[test]
fn test_table_roundtrip() {
    // Create a Table with properties
    let mut table = Table::new(99);
    table.z_order = Some(5);
    table.visible = Some(true);
    table.bounding_box = Some(Rectangle {
        top: 0.0,
        left: 0.0,
        bottom: 100.0,
        right: 200.0,
    });
    table.line_width = Some(2.0);

    // Convert to raw
    let raw = table.to_raw().expect("Failed to convert to raw");
    assert_eq!(raw.tag, 0x8016); // CDXOBJ_TABLE
    assert_eq!(raw.id, 99);
    assert!(raw.properties.len() > 0);

    // Parse back
    let table2 = Table::from_raw(raw).expect("Failed to parse roundtrip");
    assert_eq!(table2.id, 99);
    assert_eq!(table2.z_order, Some(5));
    assert_eq!(table2.visible, Some(true));
    assert_eq!(table2.line_width, Some(2.0));
    
    let bbox = table2.bounding_box.unwrap();
    assert_eq!(bbox.top, 0.0);
    assert_eq!(bbox.left, 0.0);
    assert_eq!(bbox.bottom, 100.0);
    assert_eq!(bbox.right, 200.0);
}

#[test]
fn test_table_tag_constant() {
    // Verify that the Table tag is correct
    assert_eq!(Table::TAG, 0x8016);
}
