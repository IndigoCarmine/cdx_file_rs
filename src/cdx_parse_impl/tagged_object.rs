/// Binary encoding/decoding for NodeImpl
/// The Node object represents atoms or attachment points in chemical structures.
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;
pub trait TaggedObject: Sized {
    const TAG: u16;
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError>;
    fn to_raw(&self) -> Result<RawCdxObject, CdxError>;
}
