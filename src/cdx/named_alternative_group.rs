use crate::cdx::values::{CDXString, Rectangle};
use serde::{Deserialize, Serialize};

/// Named Alternative Group (名前付き代替グループ) Object
/// Container for alternative substituents (R-Group/G-Group)
/// CDX ID: 0x800A
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NamedAlternativeGroup {
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
    /// Bounding rectangle (Optional)
    pub bounding_box: Option<Rectangle>,

    // Color
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,

    // Named Alternative Group-specific properties
    /// Upper portion bounding box (group name) (Optional)
    pub named_alternative_group_text_frame: Option<Rectangle>,
    /// Lower portion bounding box (group definition) (Optional)
    pub named_alternative_group_group_frame: Option<Rectangle>,
    /// Number of attachment points per alternative (Optional)
    pub named_alternative_group_valence: Option<i16>,
}

impl NamedAlternativeGroup {
    /// Create a new NamedAlternativeGroup with just an ID
    pub fn new(id: u32) -> Self {
        NamedAlternativeGroup {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
            named_alternative_group_text_frame: None,
            named_alternative_group_group_frame: None,
            named_alternative_group_valence: None,
        }
    }
}
