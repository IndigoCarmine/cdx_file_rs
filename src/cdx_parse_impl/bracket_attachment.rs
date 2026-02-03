use crate::cdx::bracket_attachment::BracketAttachment;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::bracket_attachment_tags::*;
use crate::error::CdxError;

impl TaggedObject for BracketAttachment {
    const TAG: u16 = CDXOBJ_BRACKET_ATTACHMENT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut attachment = BracketAttachment::new(raw.id);

        // Parse bracket graphic ID
        if let Some(graphic_id_data) = raw.get_property(CDXPROP_BRACKET_GRAPHIC_ID) {
            if graphic_id_data.len() >= 4 {
                attachment.bracket_graphic_id = Some(u32::from_le_bytes(
                    graphic_id_data[0..4]
                        .try_into()
                        .map_err(|_| CdxError::Parse("Invalid bracket_graphic_id data".to_string()))?,
                ));
            }
        }

        Ok(attachment)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Write bracket graphic ID
        if let Some(graphic_id) = self.bracket_graphic_id {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_GRAPHIC_ID,
                value: graphic_id.to_le_bytes().to_vec(),
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
