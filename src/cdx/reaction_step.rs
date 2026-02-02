use crate::cdx::values::{CDXString, Rectangle};
use serde::{Deserialize, Serialize};

/// ReactionStep (反応ステップ) Object
/// Represents a single step in a chemical reaction
/// CDX ID: 0x800E
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionStep {
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
    /// The smallest rectangle that encloses the reaction step (Optional)
    pub bounding_box: Option<Rectangle>,

    // Color
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,

    // Reaction component references
    /// Object IDs of reactants (Optional)
    pub reaction_step_reactants: Option<Vec<u32>>,
    /// Object IDs of products (Optional)
    pub reaction_step_products: Option<Vec<u32>>,
    /// Object IDs of plus signs (Optional)
    pub reaction_step_plusses: Option<Vec<u32>>,
    /// Object IDs of arrows (Optional)
    pub reaction_step_arrows: Option<Vec<u32>>,
    /// Object IDs above the arrow (Optional)
    pub reaction_step_objectsabovearrow: Option<Vec<u32>>,
    /// Object IDs below the arrow (Optional)
    pub reaction_step_objectsbelowarrow: Option<Vec<u32>>,
}

impl ReactionStep {
    /// Create a new ReactionStep with just an ID
    pub fn new(id: u32) -> Self {
        ReactionStep {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
            reaction_step_reactants: None,
            reaction_step_products: None,
            reaction_step_plusses: None,
            reaction_step_arrows: None,
            reaction_step_objectsabovearrow: None,
            reaction_step_objectsbelowarrow: None,
        }
    }
}
