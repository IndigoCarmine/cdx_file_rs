use crate::cdx::values::CDXString;
use serde::{Deserialize, Serialize};

/// Sequence (シーケンス) Object
/// Represents an ordered series member with bookmarking capability
/// CDX ID: 0x8013
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub id: u32,

    // Required property
    /// REQUIRED: Unique sequence identifier
    pub sequence_identifier: Option<CDXString>,
}

impl Sequence {
    /// Create a new Sequence with just an ID
    pub fn new(id: u32) -> Self {
        Sequence {
            id,
            sequence_identifier: None,
        }
    }
}
