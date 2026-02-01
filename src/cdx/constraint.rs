use crate::cdx::values::*;
use serde::{Deserialize, Serialize};

/// Constraint object (0x8022)
/// A distance or angle constraint between one or more objects
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub id: u32,

    // Visibility and ordering
    pub visible: Option<bool>,
    pub z_order: Option<i16>,

    // Constraint properties
    pub bounding_box: Option<Rectangle>,
    pub foreground_color: Option<u16>,
}

impl Constraint {
    pub fn new(id: u32) -> Self {
        Constraint {
            id,
            visible: None,
            z_order: None,
            bounding_box: None,
            foreground_color: None,
        }
    }
}
