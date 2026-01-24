pub mod cdx;
use binrw::binrw;
pub use cdx::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CdxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Binary read error: {0}")]
    BinRead(#[from] binrw::Error),
    #[error("Invalid CDX header")]
    InvalidHeader,
    #[error("Unknown Tag: {0:#06x}")]
    UnknownTag(u16),
    #[error("Decoding error: {0}")]
    DecodeError(String),
}

pub type Result<T> = std::result::Result<T, CdxError>;

/// CDX File Header (32 bytes)
#[binrw]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
