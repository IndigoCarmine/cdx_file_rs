use crate::cdx::cross_reference::CrossReference;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_CROSS_REFERENCE: u16 = 0x8014;

impl TaggedObject for CrossReference {
    const TAG: u16 = CDXOBJ_CROSS_REFERENCE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let cross_ref = CrossReference::new(raw.id);
        // TODO: Parse properties when needed
        Ok(cross_ref)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties: Vec::new(),
            children: Vec::new(),
        })
    }
}
