use serde::{Deserialize, Serialize};
use crate::cdx::values::{Rectangle, Point3d};

/// Arrow Object: A line or arc with optional arrowheads
/// Represents an arrow graphic element in the CDX document.
/// An Arrow can be a simple line, an arc, or can have arrowheads on one or both ends.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Arrow {
    pub id: u32,
    
    // Geometry
    /// The smallest rectangle that encloses the graphical representation of the object (Optional)
    pub bounding_box: Option<Rectangle>,
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Fill type of the arrow (Optional)
    pub fill_type: Option<i16>,
    
    // Arrowhead properties
    /// Arrowhead style at the head end (Optional)
    pub arrowhead_head: Option<i16>,
    /// Arrowhead type (Optional)
    pub arrowhead_type: Option<i16>,
    /// Size of the arrowhead at the head (Optional)
    pub head_size: Option<i16>,
    
    // 3D coordinates
    /// 3D location of the head point (Optional)
    pub head_3d: Option<Point3d>,
    /// 3D location of the tail point (Optional)
    pub tail_3d: Option<Point3d>,
    /// 3D location of the center point (Optional)
    pub center_3d: Option<Point3d>,
    /// 3D location of the major axis endpoint (Optional)
    pub major_axis_end_3d: Option<Point3d>,
    /// 3D location of the minor axis endpoint (Optional)
    pub minor_axis_end_3d: Option<Point3d>,
    
    // Styling
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,
    /// Line width (Optional)
    pub line_width: Option<f64>,
}

impl Arrow {
    /// Create a new Arrow with the given ID
    pub fn new(id: u32) -> Self {
        Arrow {
            id,
            bounding_box: None,
            z_order: None,
            fill_type: None,
            arrowhead_head: None,
            arrowhead_type: None,
            head_size: None,
            head_3d: None,
            tail_3d: None,
            center_3d: None,
            major_axis_end_3d: None,
            minor_axis_end_3d: None,
            foreground_color: None,
            background_color: None,
            line_width: None,
        }
    }
}
