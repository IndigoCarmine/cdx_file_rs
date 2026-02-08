use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::stoichiometrygrid::StoichiometryGrid;
use crate::cdx::values::*;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::stoichiometrygrid_tags::*;
use crate::error::CdxError;

impl TaggedObject for StoichiometryGrid {
    const TAG: u16 = CDXOBJ_STOICHIOMETRYGRID;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let position_2d = raw
            .get_property(CDXPROP_STOICHIOMETRYGRID_POSITION)
            .and_then(|v| Point2d::decode(v).ok());
        let raw_data = raw.get_property(CDXPROP_STOICHIOMETRYGRID_RAW).cloned();
        Ok(StoichiometryGrid {
            id: raw.id,
            position_2d,
            raw_data,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        if let Some(pos) = &self.position_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_STOICHIOMETRYGRID_POSITION,
                value: pos.encode()?,
            });
        }
        if let Some(ref raw_val) = self.raw_data {
            properties.push(RawCdxProperty {
                tag: CDXPROP_STOICHIOMETRYGRID_RAW,
                value: raw_val.clone(),
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
