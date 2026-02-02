use crate::cdx::values::{CDXString, Rectangle};
use serde::{Deserialize, Serialize};

/// Constraint (制約) Object
/// Represents a distance or angle constraint between one or more objects
/// CDX ID: 0x8022
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraint {
    pub id: u32,

    // Common properties
    /// Object name (Optional)
    pub name: Option<CDXString>,

    // Visual properties
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,

    // Styling
    /// Default bond length (Optional)
    pub bond_length: Option<f64>,
    /// Line width (Optional)
    pub line_width: Option<f64>,
    /// Hash spacing (Optional)
    pub hash_spacing: Option<f64>,

    // Font properties
    /// Label font family (Optional)
    pub label_style_font: Option<i16>,
    /// Label font size (Optional)
    pub label_style_size: Option<i16>,
    /// Label font face (Optional)
    pub label_style_face: Option<i16>,
    /// Label color (Optional)
    pub label_style_color: Option<i16>,

    // Constraint-specific properties
    /// REQUIRED: Ordered list of objects defining the constraint
    pub basis_objects: Option<Vec<u32>>,
    /// Constraint type (distance/angle/exclusion) (enumerated) (Optional)
    pub constraint_type: Option<i8>,
    /// Minimum constraint value (Optional)
    pub constraint_min: Option<f64>,
    /// Maximum constraint value (Optional)
    pub constraint_max: Option<f64>,
    /// Ignore unconnected atoms in exclusion sphere (Optional)
    pub ignore_unconnected_atoms: Option<bool>,
    /// Dihedral signed/unsigned flag (Optional)
    pub dihedral_is_chiral: Option<bool>,

    // Legacy properties
    pub visible: Option<bool>,
    pub z_order: Option<i16>,
    pub bounding_box: Option<Rectangle>,
}

impl Constraint {
    /// Create a new Constraint with just an ID
    pub fn new(id: u32) -> Self {
        Constraint {
            id,
            name: None,
            foreground_color: None,
            bond_length: None,
            line_width: None,
            hash_spacing: None,
            label_style_font: None,
            label_style_size: None,
            label_style_face: None,
            label_style_color: None,
            basis_objects: None,
            constraint_type: None,
            constraint_min: None,
            constraint_max: None,
            ignore_unconnected_atoms: None,
            dihedral_is_chiral: None,
            visible: None,
            z_order: None,
            bounding_box: None,
        }
    }
}
