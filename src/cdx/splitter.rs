use crate::cdx::values::Point2d;
use serde::{Deserialize, Serialize};

/// Splitter (スプリッター) Object
/// Represents a horizontal page divider
/// CDX ID: 0x8015
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Splitter {
    pub id: u32,

    // Properties
    /// 2D position (Y, X) (Optional)
    pub position_2d: Option<Point2d>,
    /// Page formatting type (enumerated) (Optional)
    pub page_definition: Option<i8>,
}

impl Splitter {
    /// Create a new Splitter with just an ID
    pub fn new(id: u32) -> Self {
        Splitter {
            id,
            position_2d: None,
            page_definition: None,
        }
    }
}
