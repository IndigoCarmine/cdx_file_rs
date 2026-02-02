use crate::cdx::tlc_spot::TLCSpot;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::tlc_spot_tags::*;
use crate::error::CdxError;

impl TaggedObject for TLCSpot {
    const TAG: u16 = CDXOBJ_TLC_SPOT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        Ok(TLCSpot {
            id: raw.id,
            visible: raw.get_property(CDXPROP_VISIBLE).and_then(|v| bool::decode(v).ok()),
            width: raw.get_property(CDXPROP_WIDTH).and_then(|v| f64::decode(v).ok()),
            height: raw.get_property(CDXPROP_HEIGHT).and_then(|v| f64::decode(v).ok()),
            curve_type: raw.get_property(CDXPROP_CURVE_TYPE).and_then(|v| i16::decode(v).ok()),
            tlc_rf: raw.get_property(CDXPROP_TLC_RF).and_then(|v| f64::decode(v).ok()),
            tlc_tail: raw.get_property(CDXPROP_TLC_TAIL).and_then(|v| f64::decode(v).ok()),
            tlc_show_rf: raw.get_property(CDXPROP_TLC_SHOW_RF).and_then(|v| bool::decode(v).ok()),
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty { tag: CDXPROP_VISIBLE, value: val.encode()? });
        }
        if let Some(val) = self.width {
            properties.push(RawCdxProperty { tag: CDXPROP_WIDTH, value: val.encode()? });
        }
        if let Some(val) = self.height {
            properties.push(RawCdxProperty { tag: CDXPROP_HEIGHT, value: val.encode()? });
        }
        if let Some(val) = self.curve_type {
            properties.push(RawCdxProperty { tag: CDXPROP_CURVE_TYPE, value: val.encode()? });
        }
        if let Some(val) = self.tlc_rf {
            properties.push(RawCdxProperty { tag: CDXPROP_TLC_RF, value: val.encode()? });
        }
        if let Some(val) = self.tlc_tail {
            properties.push(RawCdxProperty { tag: CDXPROP_TLC_TAIL, value: val.encode()? });
        }
        if let Some(val) = self.tlc_show_rf {
            properties.push(RawCdxProperty { tag: CDXPROP_TLC_SHOW_RF, value: val.encode()? });
        }

        Ok(RawCdxObject { tag: Self::TAG, id: self.id, properties, children: Vec::new() })
    }
}
