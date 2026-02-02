use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::table::Table;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::table_tags::*;
use crate::error::CdxError;

/// Binary encoding/decoding for Table
/// The Table object is a grid-like arrangement where each cell is a Page object.
impl TaggedObject for Table {
    const TAG: u16 = CDXOBJ_TABLE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let z_order = raw
            .get_property(CDXPROP_Z_ORDER)
            .and_then(|v| i16::decode(v).ok());
        let visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());
        let bounding_box = raw
            .get_property(CDXPROP_BOUNDING_BOX)
            .and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let foreground_color = raw
            .get_property(CDXPROP_FOREGROUND_COLOR)
            .and_then(|v| u16::decode(v).ok());
        let background_color = raw
            .get_property(CDXPROP_BACKGROUND_COLOR)
            .and_then(|v| i16::decode(v).ok());
        let bold_width = raw
            .get_property(CDXPROP_BOLD_WIDTH)
            .and_then(|v| f64::decode(v).ok());
        let line_width = raw
            .get_property(CDXPROP_LINE_WIDTH)
            .and_then(|v| f64::decode(v).ok());
        let margin_width = raw
            .get_property(CDXPROP_MARGIN_WIDTH)
            .and_then(|v| f64::decode(v).ok());
        let label_style_font = raw
            .get_property(CDXPROP_LABEL_STYLE_FONT)
            .and_then(|v| i16::decode(v).ok());
        let label_style_size = raw
            .get_property(CDXPROP_LABEL_STYLE_SIZE)
            .and_then(|v| i16::decode(v).ok());
        let label_style_face = raw
            .get_property(CDXPROP_LABEL_STYLE_FACE)
            .and_then(|v| i16::decode(v).ok());

        Ok(Table {
            id: raw.id,
            z_order,
            visible,
            bounding_box,
            foreground_color,
            background_color,
            bold_width,
            line_width,
            margin_width,
            label_style_font,
            label_style_size,
            label_style_face,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Optional properties - encode using BinaryCodec
        if let Some(val) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_Z_ORDER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.foreground_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOREGROUND_COLOR,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.background_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BACKGROUND_COLOR,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bold_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOLD_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.line_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.margin_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MARGIN_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_style_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_FONT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_style_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_SIZE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_style_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_FACE,
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
