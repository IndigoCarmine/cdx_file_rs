use crate::cdx::values::*;
use serde::{Deserialize, Serialize};

/// ObjectTag object (0x8011)
/// A metadata tag attached to objects, optionally containing a Text object for display
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectTag {
    pub id: u32,

    // Required and core properties
    pub tag_name: Option<CDXString>, // Tag name (e.g., "/CS/CD/assign")

    // Optional properties
    pub visible: Option<bool>,
    pub z_order: Option<i16>,

    // Tag value (can be string or numeric)
    pub tag_value: Option<String>, // Tag value as string
    pub tag_type: Option<i16>,     // Tag type (String, Number, etc.)

    // Position
    pub bounding_box: Option<Rectangle>,
    pub position: Option<Point2d>,

    // Color
    pub foreground_color: Option<u16>,
}

impl ObjectTag {
    pub fn new(id: u32) -> Self {
        ObjectTag {
            id,
            tag_name: None,
            visible: None,
            z_order: None,
            tag_value: None,
            tag_type: None,
            bounding_box: None,
            position: None,
            foreground_color: None,
        }
    }
}
