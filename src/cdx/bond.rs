use crate::cdx::values::CDXString;
use serde::{Deserialize, Serialize};

/// Bond Object: Defines a connection between Node objects
/// A Bond object defines a chemical bond connection between two atoms (Node objects).
/// All Bonds must be contained in Fragment objects.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bond {
    pub id: u32,
    /// ID of the CDX node object at the first end of the bond (CDX_PROP_BOND_BEGIN, 0x0600)
    pub begin: Option<u32>,
    /// ID of the CDX node object at the second end of the bond (CDX_PROP_BOND_END, 0x0601)
    pub end: Option<u32>,
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Whether chemical warnings should be suppressed (Optional)
    pub ignore_warnings: Option<bool>,
    /// Chemical warning text (Optional)
    pub chemical_warning: Option<CDXString>,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,
    /// Bond order (bit-encoded) (Optional)
    pub bond_order: Option<i16>,
    /// Primary bond display type (Optional)
    pub display: Option<i16>,
    /// Secondary display for double bonds (Optional)
    pub display2: Option<i16>,
    /// Double bond line position (Optional)
    pub double_position: Option<i16>,
    /// Query topology restriction (Optional)
    pub topology: Option<i8>,
    /// Reaction participation restriction (Optional)
    pub rxn_participation: Option<i8>,
    /// Attachment point on begin node (Optional)
    pub begin_attach: Option<u8>,
    /// Attachment point on end node (Optional)
    pub end_attach: Option<u8>,
    /// Cahn–Ingold–Prelog stereochemistry (Optional)
    pub cip_stereochemistry: Option<i8>,
    /// Ordered list of attached bond IDs (Optional)
    pub bond_circular_ordering: Option<Vec<u32>>,
    /// Show query indicator (Optional)
    pub show_query: Option<bool>,
    /// Show stereochemistry indicator (Optional)
    pub show_stereo: Option<bool>,
    /// Bonds crossing this bond (Optional)
    pub crossing_bonds: Option<Vec<u32>>,
    /// Show reaction-change indicator (Optional)
    pub show_rxn: Option<bool>,
    /// Relative spacing of multiple bonds (Optional)
    pub bond_spacing: Option<i16>,
    /// Default bond length (Optional)
    pub bond_length: Option<f64>,
    /// Bold bond width (Optional)
    pub bold_width: Option<f64>,
    /// Line width (Optional)
    pub line_width: Option<f64>,
    /// Margin around atom labels (Optional)
    pub margin_width: Option<f64>,
    /// Hashed bond spacing (Optional)
    pub hash_spacing: Option<f64>,
    /// Atom label font family (Optional)
    pub label_font: Option<i16>,
    /// Atom label font size (Optional)
    pub label_size: Option<i16>,
    /// Atom label font face (Optional)
    pub label_face: Option<i16>,
    /// Absolute spacing of multiple bonds (Optional)
    pub bond_spacing_abs: Option<f64>,
}

impl Bond {
    /// Create a new Bond with required properties
    pub fn new(id: u32) -> Self {
        Bond {
            id,
            begin: None,
            end: None,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            foreground_color: None,
            background_color: None,
            bond_order: None,
            display: None,
            display2: None,
            double_position: None,
            topology: None,
            rxn_participation: None,
            begin_attach: None,
            end_attach: None,
            cip_stereochemistry: None,
            bond_circular_ordering: None,
            show_query: None,
            show_stereo: None,
            crossing_bonds: None,
            show_rxn: None,
            bond_spacing: None,
            bond_length: None,
            bold_width: None,
            line_width: None,
            margin_width: None,
            hash_spacing: None,
            label_font: None,
            label_size: None,
            label_face: None,
            bond_spacing_abs: None,
        }
    }
}
