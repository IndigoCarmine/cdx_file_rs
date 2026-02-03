use crate::cdx::splitter::Splitter;
use crate::cdx::values::Point2d;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::splitter_tags::*;
use crate::error::CdxError;


impl TaggedObject for Splitter {
    const TAG: u16 = CDXOBJ_SPLITTER;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut splitter = Splitter::new(raw.id);

        // Parse position_2d
        if let Some(pos_data) = raw.get_property(CDXPROP_2D_POSITION) {
            if pos_data.len() >= 16 {
                let y = f64::from_le_bytes(
                    pos_data[0..8]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid position_2d data".to_string()))?,
                );
                let x = f64::from_le_bytes(
                    pos_data[8..16]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid position_2d data".to_string()))?,
                );
                splitter.position_2d = Some(Point2d { x, y });
            }
        }

        // Parse page_definition
        if let Some(def_data) = raw.get_property(CDXPROP_PAGE_DEFINITION) {
            if !def_data.is_empty() {
                splitter.page_definition = Some(i8::from_le_bytes([def_data[0]]));
            }
        }

        Ok(splitter)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject::new(Self::TAG, self.id);

        // Write position_2d
        if let Some(ref pos) = self.position_2d {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&pos.y.to_le_bytes());
            bytes.extend_from_slice(&pos.x.to_le_bytes());
            raw.add_property(CDXPROP_2D_POSITION, bytes);
        }

        // Write page_definition
        if let Some(def) = self.page_definition {
            raw.add_property(CDXPROP_PAGE_DEFINITION, vec![def as u8]);
        }

        Ok(raw)
    }
}
