use serde::{Deserialize, Serialize};

/// TLC Lane Object: Lane within a TLC plate
/// A TLC Lane object represents a single lane on a TLC (Thin Layer Chromatography) plate.
/// TLC Lane objects technically have no required properties, but should contain at least one Spot to be useful.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TlcLane {
    pub id: u32,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,
}

impl TlcLane {
    /// Create a new TlcLane with default values
    pub fn new(id: u32) -> Self {
        TlcLane { id, visible: None }
    }
}
