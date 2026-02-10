
use serde::{Deserialize, Serialize};
use crate::cdx::values::{CDXString, Point2d, BooleanImplied};
/// ObjectTag object (0x8011)
/// A metadata tag attached to objects, optionally containing a Text object for display
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectTag {
    pub id: u32,
    pub name: Option<String>,
    pub object_type: Option<i16>,
    pub trackig: Option<BooleanImplied>,
    pub oersistent: Option<BooleanImplied>,
    pub value: Option<Vec<u8>>,
    pub positioning: Option<i8>,
    pub position_angle: Option<i32>,
    pub position_offset: Option<Point2d>,
}

impl ObjectTag {
    pub fn new(id: u32) -> Self {
        ObjectTag {
            id,
            name: None,
            object_type: None,
            trackig: None,
            oersistent: None,
            value: None,
            positioning: None,
            position_angle: None,
            position_offset: None,
        }
    }
}
