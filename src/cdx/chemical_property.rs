use crate::cdx::values::Point2d;
use serde::{Deserialize, Serialize};

/// Chemical Property (化学プロパティ) Object
/// Represents physical/chemical property annotation attached to objects
/// CDX ID: 0x8026
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChemicalProperty {
    pub id: u32,

    // Core properties
    /// Object name (Optional)
    pub name: Option<String>,

    // Property references
    /// IDs of objects defining the property (Optional)
    pub basis_objects: Option<Vec<u32>>,
    /// Property type (name, formula, MW, etc.) (enumerated) (Optional)
    pub chemical_property_type: Option<i8>,
    /// ID of the display object (Optional)
    pub chemical_property_display_id: Option<u32>,
    /// Auto-update flag (Optional)
    pub chemical_property_is_active: Option<bool>,

    // Positioning
    /// Positioning type (enumerated) (Optional)
    pub positioning: Option<i8>,
    /// Angular positioning (degrees * 65536) (Optional)
    pub positioning_angle: Option<i32>,
    /// Offset positioning (Optional)
    pub positioning_offset: Option<Point2d>,
}

impl ChemicalProperty {
    /// Create a new ChemicalProperty with just an ID
    pub fn new(id: u32) -> Self {
        ChemicalProperty {
            id,
            name: None,
            basis_objects: None,
            chemical_property_type: None,
            chemical_property_display_id: None,
            chemical_property_is_active: None,
            positioning: None,
            positioning_angle: None,
            positioning_offset: None,
        }
    }
}
