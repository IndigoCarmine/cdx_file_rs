/// Binary encoding/decoding for Group
/// A Group is a logical collection of objects.
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::group_tag::*;
use crate::error::CdxError;
use crate::cdx::group::Group;

impl TaggedObject for Group {
    const TAG: u16 = CDXOBJ_GROUP;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let bounding_box = raw.get_property(CDXPROP_BOUNDING_BOX).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let integral = raw.get_property(CDXPROP_GROUP_INTEGRAL).and_then(|v| bool::decode(v).ok());

        Ok(Group {
            id: raw.id,
            bounding_box,
            integral,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        
        let mut properties = Vec::new();
        
        // Optional properties
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.integral {
            properties.push(RawCdxProperty {
                tag: CDXPROP_GROUP_INTEGRAL,
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
