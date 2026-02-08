use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::segcomponent::SegComponent;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::segcomponent_tags::*;
use crate::error::CdxError;

impl TaggedObject for SegComponent {
    const TAG: u16 = CDXOBJ_SEGCOMPONENT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let width = raw
            .get_property(CDXPROP_SEGCOMPONENT_WIDTH)
            .and_then(|v| i32::decode(v).ok());
        let component_is_reactant = raw
            .get_property(CDXPROP_SEGCOMPONENT_IS_REACTANT)
            .and_then(|v| bool::decode(v).ok());
        let component_is_header = raw
            .get_property(CDXPROP_SEGCOMPONENT_IS_HEADER)
            .and_then(|v| bool::decode(v).ok());
        Ok(SegComponent {
            id: raw.id,
            width,
            component_is_reactant,
            component_is_header,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        if let Some(val) = self.width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SEGCOMPONENT_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.component_is_reactant {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SEGCOMPONENT_IS_REACTANT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.component_is_header {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SEGCOMPONENT_IS_HEADER,
                value: val.encode()?,
            });
        }
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
