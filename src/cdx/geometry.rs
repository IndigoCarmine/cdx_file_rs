use crate::cdx::values::{Point2d, Rectangle};
use serde::{Deserialize, Serialize};

/// Geometry Object: A geometrical relationship between one or more objects
/// A Geometry object represents a geometrical relationship (e.g., a distance or angle)
/// between one or more chemical objects in a structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geometry {
    pub id: u32,

    // Visibility and ordering
    pub visible: Option<bool>,
    pub z_order: Option<i16>,

    // Geometry properties
    pub bounding_box: Option<Rectangle>,
    pub position: Option<Point2d>,
    pub rotation_angle: Option<i32>,

    // Color properties
    pub foreground_color: Option<u16>,
    pub background_color: Option<i16>,
}

impl Geometry {
    /// Create a new Geometry with just an ID
    pub fn new(id: u32) -> Self {
        Geometry {
            id,
            visible: None,
            z_order: None,
            bounding_box: None,
            position: None,
            rotation_angle: None,
            foreground_color: None,
            background_color: None,
        }
    }
}
