use crate::cdx::values::*;

/// Represents a Stoichiometry Grid object
#[derive(Debug, Clone)]
pub struct StoichiometryGrid {
    pub id: u32,
    pub position_2d: Option<Point2d>,
    pub raw_data: Option<Vec<u8>>, // For unknown/raw property
}

impl StoichiometryGrid {
    pub fn new(id: u32) -> Self {
        StoichiometryGrid {
            id,
            position_2d: None,
            raw_data: None,
        }
    }
}
