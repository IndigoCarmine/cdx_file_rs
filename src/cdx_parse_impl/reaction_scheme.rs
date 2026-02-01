use crate::cdx::reaction_scheme::ReactionScheme;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::reaction_scheme_tags::*;
use crate::error::CdxError;

impl TaggedObject for ReactionScheme {
    const TAG: u16 = CDXOBJ_REACTION_SCHEME;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        Ok(ReactionScheme::new(raw.id))
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
