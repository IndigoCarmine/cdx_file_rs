use crate::cdx::unknown_802b::UnknownObject802B;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;

pub const CDXOBJ_UNKNOWN_802B: u16 = 0x802B;

impl TaggedObject for UnknownObject802B {
    const TAG: u16 = CDXOBJ_UNKNOWN_802B;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut obj = UnknownObject802B::new(raw.id);
        
        // Store all properties as-is for potential roundtrip
        for prop in &raw.properties {
            obj.raw_properties.push((prop.tag, prop.value.clone()));
        }
        
        Ok(obj)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;
        
        let properties: Vec<RawCdxProperty> = self.raw_properties
            .iter()
            .map(|(tag, value)| RawCdxProperty {
                tag: *tag,
                value: value.clone(),
            })
            .collect();

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
