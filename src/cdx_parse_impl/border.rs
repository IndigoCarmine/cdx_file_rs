use crate::cdx::border::Border;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::border_tags::*;
use crate::error::CdxError;

impl TaggedObject for Border {
    const TAG: u16 = CDXOBJ_BORDER;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut border = Border::new(raw.id);

        // Parse foreground color
        if let Some(data) = raw.get_property(CDXPROP_FOREGROUND_COLOR) {
            border.foreground_color = Some(u16::decode(data)?);
        }

        // Parse line width (CDXCoordinate = f64)
        if let Some(data) = raw.get_property(CDXPROP_LINE_WIDTH) {
            border.line_width = Some(f64::decode(data)?);
        }

        // Parse side (required)
        if let Some(data) = raw.get_property(CDXPROP_SIDE) {
            border.side = Some(u16::decode(data)?);
        }

        // Parse line type
        if let Some(data) = raw.get_property(CDXPROP_LINE_TYPE) {
            border.line_type = Some(i16::decode(data)?);
        }

        Ok(border)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties: Vec::new(),
            children: Vec::new(),
        };

        // Serialize properties
        if let Some(color) = self.foreground_color {
            raw.set_property(CDXPROP_FOREGROUND_COLOR, color.encode()?);
        }

        if let Some(width) = self.line_width {
            raw.set_property(CDXPROP_LINE_WIDTH, width.encode()?);
        }

        if let Some(side) = self.side {
            raw.set_property(CDXPROP_SIDE, side.encode()?);
        }

        if let Some(line_type) = self.line_type {
            raw.set_property(CDXPROP_LINE_TYPE, line_type.encode()?);
        }

        Ok(raw)
    }
}
