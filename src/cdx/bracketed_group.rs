use serde::{Deserialize, Serialize};

/// Bracketed Group (括弧で囲まれたグループ) Object
/// Represents a collection of objects surrounded by brackets (polymers, etc.)
/// CDX ID: 0x8017
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BracketedGroup {
    pub id: u32,

    // Bracket properties
    /// Chemical meaning of the bracket (SRU, mer, mon, xlink, etc.)
    pub bracket_usage: Option<i8>,
    /// Head-to-tail connectivity pattern (enumerated)
    pub polymer_repeat_pattern: Option<i8>,
    /// Flip state of contained objects (enumerated)
    pub polymer_flip_type: Option<i8>,

    // Object references
    /// IDs of objects contained in the bracketed group
    pub bracketed_objects: Option<Vec<u32>>,
    /// Repeat count for the bracketed group
    pub bracket_repeat_count: Option<i16>,
    /// Component order for the bracketed group
    pub bracket_component_order: Option<Vec<u32>>,

    // Labels
    /// SRU (Structural Repeat Unit) label text
    pub bracket_sru_label: Option<String>,
}

impl BracketedGroup {
    /// Create a new BracketedGroup with just an ID
    pub fn new(id: u32) -> Self {
        BracketedGroup {
            id,
            bracket_usage: None,
            polymer_repeat_pattern: None,
            polymer_flip_type: None,
            bracketed_objects: None,
            bracket_repeat_count: None,
            bracket_component_order: None,
            bracket_sru_label: None,
        }
    }
}
