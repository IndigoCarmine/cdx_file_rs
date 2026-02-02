use crate::cdx::chemical_property::ChemicalProperty;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_CHEMICAL_PROPERTY: u16 = 0x8026;

impl TaggedObject for ChemicalProperty {
    const TAG: u16 = CDXOBJ_CHEMICAL_PROPERTY;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let property = ChemicalProperty::new(raw.id);
        // TODO: Parse properties when needed
        Ok(property)
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
