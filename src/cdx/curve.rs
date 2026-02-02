use crate::cdx::values::{CDXString, Point3d, Rectangle};
use serde::{Deserialize, Serialize};

/// Curve (曲線) Object
/// Represents a Bézier curve
/// CDX ID: 0x8008
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Curve {
    pub id: u32,

    // Common properties
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Suppress chemical warnings (Optional)
    pub ignore_warnings: Option<bool>,
    /// Chemical warning text (Optional)
    pub chemical_warning: Option<CDXString>,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,

    // Geometry
    /// The smallest rectangle that encloses the curve (Optional)
    pub bounding_box: Option<Rectangle>,

    // Color
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,

    // Curve-specific properties
    /// Curve type (bit-encoded) (Optional)
    pub curve_type: Option<i16>,
    /// Arrowhead size (Optional)
    pub arrowhead_size: Option<i16>,
    /// REQUIRED: Bézier control points
    pub curve_points: Option<Vec<Point3d>>,
    /// 3D Bézier control points (Optional)
    pub curve_points_3d: Option<Vec<Point3d>>,
    /// Arrowhead type (enumerated) (Optional)
    pub arrowhead_type: Option<i8>,
    /// Arrowhead center size (Optional)
    pub arrowhead_center_size: Option<i16>,
    /// Arrowhead half-width (Optional)
    pub arrowhead_width: Option<i16>,
    /// Head arrowhead type (enumerated) (Optional)
    pub arrow_arrowhead_head: Option<i8>,
    /// Tail arrowhead type (enumerated) (Optional)
    pub arrow_arrowhead_tail: Option<i8>,
    /// Fill type (enumerated) (Optional)
    pub fill_type: Option<i8>,
    /// Closed curve flag (Optional)
    pub closed: Option<bool>,
    /// Spacing for doubled curves (Optional)
    pub curve_spacing: Option<i16>,
}

impl Curve {
    /// Create a new Curve with just an ID
    pub fn new(id: u32) -> Self {
        Curve {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
            curve_type: None,
            arrowhead_size: None,
            curve_points: None,
            curve_points_3d: None,
            arrowhead_type: None,
            arrowhead_center_size: None,
            arrowhead_width: None,
            arrow_arrowhead_head: None,
            arrow_arrowhead_tail: None,
            fill_type: None,
            closed: None,
            curve_spacing: None,
        }
    }
}
