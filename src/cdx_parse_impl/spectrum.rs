use crate::cdx::spectrum::Spectrum;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_SPECTRUM: u16 = 0x8010;

impl TaggedObject for Spectrum {
    const TAG: u16 = CDXOBJ_SPECTRUM;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let spectrum = Spectrum::new(raw.id);
        // TODO: Parse spectrum data when needed
        Ok(spectrum)
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
