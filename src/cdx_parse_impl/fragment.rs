/// Binary encoding/decoding for Fragment
/// A Fragment is a collection of nodes and bonds representing chemically meaningful structures.


use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::fragment_tags::*;
use crate::error::CdxError;
use crate::cdx::fragment::Fragment;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};


impl TaggedObject for Fragment {
    const TAG: u16 = CDXOBJ_FRAGMENT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let bounding_box = raw.get_property(CDXPROP_BOUNDING_BOX).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let mole_racemic = raw.get_property(CDXPROP_MOLE_RACEMIC).and_then(|v| bool::decode(v).ok());
        let mole_absolute = raw.get_property(CDXPROP_MOLE_ABSOLUTE).and_then(|v| bool::decode(v).ok());
        let mole_relative = raw.get_property(CDXPROP_MOLE_RELATIVE).and_then(|v| bool::decode(v).ok());
        let mole_weight = raw.get_property(CDXPROP_MOLE_WEIGHT).and_then(|v| f64::decode(v).ok());
        let frag_connection_order = raw.get_property(CDXPROP_FRAG_CONNECTION_ORDER).and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());

        Ok(Fragment {
            id: raw.id,
            bounding_box,
            mole_racemic,
            mole_absolute,
            mole_relative,
            mole_weight,
            frag_connection_order,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        
        let mut properties = Vec::new();
        
        // Optional properties - encode using BinaryCodec
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.mole_racemic {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MOLE_RACEMIC,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.mole_absolute {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MOLE_ABSOLUTE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.mole_relative {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MOLE_RELATIVE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.mole_weight {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MOLE_WEIGHT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.frag_connection_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FRAG_CONNECTION_ORDER,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
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
