use serde::{Deserialize, Serialize};

/// A Group is a logical collection of objects
/// ChemDraw creates Group objects via the Group command in the Object menu.
/// Groups are especially useful to associate otherwise-disjoint fragments,
/// for example, an organic anion and the counterion that accompanies it.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: u32,
    
    // Optional properties
    pub bounding_box: Option<crate::cdx::values::Rectangle>,
    pub integral: Option<bool>,
}

impl Group {
    /// Create a new Group
    pub fn new(id: u32) -> Self {
        Group {
            id,
            bounding_box: None,
            integral: None,
        }
    }
}
