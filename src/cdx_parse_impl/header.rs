use binrw::binrw;
use serde::{Deserialize, Serialize};

/// CDX File Header (32 bytes)
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[binrw]
#[br(little)]
#[bw(little)]
pub struct CdxHeader {
    pub magic: [u8; 8],          // "VjCD0100"
    pub reserved_legacy: u32,    // 0x04030201
    pub reserved_zero: [u8; 10], // zeros
}

impl Default for CdxHeader {
    fn default() -> Self {
        Self {
            magic: *b"VjCD0100",
            reserved_legacy: 0x04030201,
            reserved_zero: [0; 10],
        }
    }
}
