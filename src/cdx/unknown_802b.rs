use serde::{Deserialize, Serialize};
use crate::cdx::values::*;

/// Unknown object type 0x802B (32811)
/// This object type is not documented in the CDX specification
/// Creating stub implementation to prevent parse errors
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnknownObject802B {
    pub id: u32,
    
    // Store raw properties for roundtrip compatibility
    pub raw_properties: Vec<(u16, Vec<u8>)>,
}

impl UnknownObject802B {
    pub fn new(id: u32) -> Self {
        UnknownObject802B {
            id,
            raw_properties: Vec::new(),
        }
    }
}
