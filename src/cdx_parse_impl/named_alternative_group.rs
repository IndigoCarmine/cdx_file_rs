use crate::cdx::named_alternative_group::NamedAlternativeGroup;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_NAMED_ALTERNATIVE_GROUP: u16 = 0x800A;

impl TaggedObject for NamedAlternativeGroup {
    const TAG: u16 = CDXOBJ_NAMED_ALTERNATIVE_GROUP;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let group = NamedAlternativeGroup::new(raw.id);
        // TODO: Parse properties when needed
        Ok(group)
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
