use serde::{Deserialize, Serialize};

/// Crossing Bond Object
/// Represents a bond that crosses the boundary of a bracketed group
/// CDX ID: 0x8019
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossingBond {
    pub id: u32,

    // Properties
    /// ID of the bond that crosses the bracket boundary
    pub crossing_bond_id: Option<u32>,
    /// Flag indicating if the begin node is inside the bracket
    pub crossing_bond_begin_inside: Option<bool>,
}

impl CrossingBond {
    /// Create a new CrossingBond with just an ID
    pub fn new(id: u32) -> Self {
        CrossingBond {
            id,
            crossing_bond_id: None,
            crossing_bond_begin_inside: None,
        }
    }
}
