use serde::{Deserialize, Serialize};

/// TLC Spot (TLCスポット) Object
/// Represents an individual spot on a TLC (Thin Layer Chromatography) lane
/// CDX ID: 0x8025
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TLCSpot {
    pub id: u32,

    // Common properties
    /// Visibility flag (Optional)
    pub visible: Option<bool>,

    // Geometry
    /// Spot width in unrotated reference frame (Optional)
    pub width: Option<f64>,
    /// Spot height in unrotated reference frame (Optional)
    pub height: Option<f64>,

    // Display properties
    /// Curve type (bit-encoded) (Optional)
    pub curve_type: Option<i16>,

    // TLC-specific properties
    /// Retention factor (Rf) value (Optional)
    pub tlc_rf: Option<f64>,
    /// Tail length (Optional)
    pub tlc_tail: Option<f64>,
    /// Flag to display Rf value (Optional)
    pub tlc_show_rf: Option<bool>,
}

impl TLCSpot {
    /// Create a new TLCSpot with just an ID
    pub fn new(id: u32) -> Self {
        TLCSpot {
            id,
            visible: None,
            width: None,
            height: None,
            curve_type: None,
            tlc_rf: None,
            tlc_tail: None,
            tlc_show_rf: None,
        }
    }
}
