/// Binary encoding/decoding for TLCPlate
/// A TLC Plate represents a Thin Layer Chromatography (TLC) plate containing lanes.

use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject,RawCdxProperty};
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::tlc_plate_tags::*;
use crate::error::CdxError;
use crate::cdx::tlc_plate::TLCPlate;

impl TaggedObject for TLCPlate {
    const TAG: u16 = CDXOBJ_TLC_PLATE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let z_order = raw.get_property(CDXPROP_Z_ORDER).and_then(|v| i16::decode(v).ok());
        let visible = raw.get_property(CDXPROP_VISIBLE).and_then(|v| bool::decode(v).ok());
        let bounding_box = raw.get_property(CDXPROP_BOUNDING_BOX).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let top_left = raw.get_property(CDXPROP_TOP_LEFT).and_then(|v| crate::cdx::values::Point2d::decode(v).ok());
        let top_right = raw.get_property(CDXPROP_TOP_RIGHT).and_then(|v| crate::cdx::values::Point2d::decode(v).ok());
        let bottom_right = raw.get_property(CDXPROP_BOTTOM_RIGHT).and_then(|v| crate::cdx::values::Point2d::decode(v).ok());
        let bottom_left = raw.get_property(CDXPROP_BOTTOM_LEFT).and_then(|v| crate::cdx::values::Point2d::decode(v).ok());
        let foreground_color = raw.get_property(CDXPROP_FOREGROUND_COLOR).and_then(|v| u16::decode(v).ok());
        let background_color = raw.get_property(CDXPROP_BACKGROUND_COLOR).and_then(|v| i16::decode(v).ok());
        let bold_width = raw.get_property(CDXPROP_BOLD_WIDTH).and_then(|v| f64::decode(v).ok());
        let line_width = raw.get_property(CDXPROP_LINE_WIDTH).and_then(|v| f64::decode(v).ok());
        let margin_width = raw.get_property(CDXPROP_MARGIN_WIDTH).and_then(|v| f64::decode(v).ok());
        let label_font = raw.get_property(CDXPROP_LABEL_FONT).and_then(|v| i16::decode(v).ok());
        let label_size = raw.get_property(CDXPROP_LABEL_SIZE).and_then(|v| i16::decode(v).ok());
        let label_face = raw.get_property(CDXPROP_LABEL_FACE).and_then(|v| i16::decode(v).ok());
        let tlc_origin_fraction = raw.get_property(CDXPROP_TLC_ORIGIN_FRACTION).and_then(|v| f64::decode(v).ok());
        let tlc_solvent_front_fraction = raw.get_property(CDXPROP_TLC_SOLVENT_FRONT_FRACTION).and_then(|v| f64::decode(v).ok());
        let tlc_show_origin = raw.get_property(CDXPROP_TLC_SHOW_ORIGIN).and_then(|v| bool::decode(v).ok());
        let tlc_show_solvent_front = raw.get_property(CDXPROP_TLC_SHOW_SOLVENT_FRONT).and_then(|v| bool::decode(v).ok());
        let tlc_show_borders = raw.get_property(CDXPROP_TLC_SHOW_BORDERS).and_then(|v| bool::decode(v).ok());

        Ok(TLCPlate {
            id: raw.id,
            z_order,
            visible,
            bounding_box,
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            foreground_color,
            background_color,
            bold_width,
            line_width,
            margin_width,
            label_font,
            label_size,
            label_face,
            tlc_origin_fraction,
            tlc_solvent_front_fraction,
            tlc_show_origin,
            tlc_show_solvent_front,
            tlc_show_borders,
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
        if let Some(ref val) = self.top_left {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TOP_LEFT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.top_right {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TOP_RIGHT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bottom_right {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOTTOM_RIGHT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bottom_left {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOTTOM_LEFT,
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
        if let Some(val) = self.label_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FONT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_SIZE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FACE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.tlc_origin_fraction {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TLC_ORIGIN_FRACTION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.tlc_solvent_front_fraction {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TLC_SOLVENT_FRONT_FRACTION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.tlc_show_origin {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TLC_SHOW_ORIGIN,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.tlc_show_solvent_front {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TLC_SHOW_SOLVENT_FRONT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.tlc_show_borders {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TLC_SHOW_BORDERS,
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
