use crate::cdx::values::CDXString;
use serde::{Deserialize, Serialize};

/// Cross-Reference (クロスリファレンス) Object
/// Represents a link to a Sequence object
/// CDX ID: 0x8014
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrossReference {
    pub id: u32,

    // Cross-reference properties
    /// External container object (Optional)
    pub cross_reference_container: Option<u32>,
    /// External document path (Optional)
    pub cross_reference_document: Option<CDXString>,
    /// REQUIRED: Unique cross-reference ID
    pub cross_reference_identifier: Option<CDXString>,
    /// REQUIRED: Target sequence identifier
    pub cross_reference_sequence: Option<CDXString>,
}

impl CrossReference {
    /// Create a new CrossReference with just an ID
    pub fn new(id: u32) -> Self {
        CrossReference {
            id,
            cross_reference_container: None,
            cross_reference_document: None,
            cross_reference_identifier: None,
            cross_reference_sequence: None,
        }
    }
}
