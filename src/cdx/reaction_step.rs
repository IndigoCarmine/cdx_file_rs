use crate::cdx::values::*;
use serde::{Deserialize, Serialize};

/// ReactionStep object (0x800E)
/// A component of a reaction scheme that represents a single chemical transformation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionStep {
    pub id: u32,

    // Core properties
    pub z_order: Option<i16>,
    pub visible: Option<bool>,

    // Positioning
    pub bounding_box: Option<Rectangle>,

    // Reaction properties
    pub reaction_step_reactants: Option<Vec<u32>>, // Object IDs of reactants
    pub reaction_step_products: Option<Vec<u32>>,  // Object IDs of products
    pub reaction_step_plusses: Option<Vec<u32>>,   // Object IDs of plus signs
    pub reaction_step_arrows: Option<Vec<u32>>,    // Object IDs of arrows
    pub reaction_step_objectsabovearrow: Option<Vec<u32>>, // Object IDs above arrow
    pub reaction_step_objectsbelowarrow: Option<Vec<u32>>, // Object IDs below arrow
}

impl ReactionStep {
    pub fn new(id: u32) -> Self {
        ReactionStep {
            id,
            z_order: None,
            visible: None,
            bounding_box: None,
            reaction_step_reactants: None,
            reaction_step_products: None,
            reaction_step_plusses: None,
            reaction_step_arrows: None,
            reaction_step_objectsabovearrow: None,
            reaction_step_objectsbelowarrow: None,
        }
    }
}
