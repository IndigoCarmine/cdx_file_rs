//! Binary encoding/decoding for TlcLane
//! A TLC Lane represents a lane on a TLC (Thin Layer Chromatography) plate.

use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::tlc_lane::TlcLane;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::tlc_lane_tags::*;
use crate::error::CdxError;

impl TaggedObject for TlcLane {
    const TAG: u16 = CDXOBJ_TLC_LANE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties
        let visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        Ok(TlcLane {
            id: raw.id,
            visible,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Optional properties
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
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
