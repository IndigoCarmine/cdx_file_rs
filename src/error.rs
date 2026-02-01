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
    #[error("Parse Error:{0}")]
    Parse(String),
    #[error("Encode Error:{0}")]
    EncodeError(String),
}

pub type Result<T> = std::result::Result<T, CdxError>;
