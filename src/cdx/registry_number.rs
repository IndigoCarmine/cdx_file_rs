use crate::cdx::values::CDXString;
use serde::{Deserialize, Serialize};

/// Registry Number (レジストリ番号) Object
/// Represents a registry or catalog number (CAS, Beilstein, etc.)
/// CDX ID: 0x800C
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryNumber {
    pub id: u32,

    // Required properties
    /// REQUIRED: Registry/catalog number
    pub registry_number: Option<CDXString>,
    /// REQUIRED: Issuing authority (CAS, Beilstein, etc.)
    pub registry_authority: Option<CDXString>,
}

impl RegistryNumber {
    /// Create a new RegistryNumber with just an ID
    pub fn new(id: u32) -> Self {
        RegistryNumber {
            id,
            registry_number: None,
            registry_authority: None,
        }
    }
}
