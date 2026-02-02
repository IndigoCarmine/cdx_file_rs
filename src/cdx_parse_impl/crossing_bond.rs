use crate::cdx::crossing_bond::CrossingBond;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::crossing_bond_tags::*;
use crate::error::CdxError;

impl TaggedObject for CrossingBond {
    const TAG: u16 = CDXOBJ_CROSSING_BOND;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut crossing = CrossingBond::new(raw.id);

        // Parse crossing bond ID
        if let Some(bond_id_data) = raw.get_property(CDXPROP_CROSSING_BOND_ID) {
            if bond_id_data.len() >= 4 {
                crossing.crossing_bond_id = Some(u32::from_le_bytes(
                    bond_id_data[0..4]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid crossing_bond_id data".to_string()))?,
                ));
            }
        }

        // Parse crossing bond begin inside flag
        if let Some(inside_data) = raw.get_property(CDXPROP_CROSSING_BOND_BEGIN_INSIDE) {
            if !inside_data.is_empty() {
                crossing.crossing_bond_begin_inside = Some(inside_data[0] != 0);
            }
        }

        Ok(crossing)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Write crossing bond ID
        if let Some(bond_id) = self.crossing_bond_id {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSSING_BOND_ID,
                value: bond_id.to_le_bytes().to_vec(),
            });
        }

        // Write crossing bond begin inside flag
        if let Some(begin_inside) = self.crossing_bond_begin_inside {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSSING_BOND_BEGIN_INSIDE,
                value: vec![begin_inside as u8],
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
