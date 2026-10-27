//! CDXML (XML representation of CDX) reading.

use crate::cdx::file::CdxFile;
use crate::error::CdxError;

impl CdxFile {
    /// Parse a CDXML document.
    ///
    /// Not implemented yet; always returns an error.
    pub fn from_cdxml_str(_xml: &str) -> Result<Self, CdxError> {
        Err(CdxError::Parse(
            "CDXML reading is not implemented yet".to_string(),
        ))
    }
}
