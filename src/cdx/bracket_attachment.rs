use serde::{Deserialize, Serialize};

/// Bracket Attachment Object
/// Represents a linkage between a bracketed group and an external object
/// CDX ID: 0x8018
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BracketAttachment {
    pub id: u32,

    // Core property
    /// ID of the associated graphic (bracket, brace, or parenthesis)
    pub bracket_graphic_id: Option<u32>,
}

impl BracketAttachment {
    /// Create a new BracketAttachment with just an ID
    pub fn new(id: u32) -> Self {
        BracketAttachment {
            id,
            bracket_graphic_id: None,
        }
    }
}
