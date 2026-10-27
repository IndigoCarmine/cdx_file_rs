//! CDXML (XML representation of CDX) reading and writing for [`CdxFile`]; the codec itself is
//! [`crate::cdxml`].

use crate::cdx::file::{CdxFile, NodePayload};
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;

impl CdxFile {
    /// Read a CDXML document. What the codec leaves out is listed by
    /// [`crate::cdxml::parse_cdxml_with_warnings`].
    pub fn from_cdxml_str(xml: &str) -> Result<Self, CdxError> {
        Self::from_raw(crate::cdxml::parse_cdxml(xml)?)
    }

    /// Read binary CDX or CDXML, whichever the bytes are.
    pub fn from_cdx_or_cdxml(bytes: &[u8]) -> Result<Self, CdxError> {
        if is_xml(bytes) {
            let xml = std::str::from_utf8(bytes).map_err(|e| CdxError::DecodeError(format!("CDXML: {e}")))?;
            Self::from_cdxml_str(xml)
        } else {
            Self::from_bytes(bytes)
        }
    }

    /// The document as a raw object tree, as binary CDX and CDXML are written from.
    pub fn to_raw_tree(&self) -> Result<RawCdxObject, CdxError> {
        fn raw(node: &dendron::Node<NodePayload>) -> Result<RawCdxObject, CdxError> {
            let mut obj = node.borrow_data().to_raw()?;
            for child in node.children() {
                obj.children.push(raw(&child)?);
            }
            Ok(obj)
        }
        raw(&self.tree.root())
    }

    /// The document as CDXML.
    pub fn to_cdxml(&self) -> Result<String, CdxError> {
        Ok(crate::cdxml::write_cdxml(&self.to_raw_tree()?))
    }
}

/// CDXML starts with `<` (after a byte-order mark and whitespace); binary CDX with `VjCD`.
fn is_xml(bytes: &[u8]) -> bool {
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(bytes);
    bytes.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'<')
}
