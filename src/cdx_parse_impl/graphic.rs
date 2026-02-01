use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::graphic::Graphic;
use crate::cdx::values::*;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_GRAPHIC: u16 = 0x8007;

// Property tags based on specification
const CDXPROP_ZORDER: u16 = 0x000A;
const CDXPROP_IGNOREWARNINGS: u16 = 0x000F;
const CDXPROP_CHEMICALWARNING: u16 = 0x0010;
const CDXPROP_VISIBLE: u16 = 0x0011;
const CDXPROP_SUPERSEDEDBY: u16 = 0x0012;
const CDXPROP_BOUNDINGBOX: u16 = 0x0204;
const CDXPROP_3DHEAD: u16 = 0x0207;
const CDXPROP_3DTAIL: u16 = 0x0208;
const CDXPROP_FOREGROUNDCOLOR: u16 = 0x0301;
const CDXPROP_BACKGROUNDCOLOR: u16 = 0x0302;
const CDXPROP_BOLDWIDTH: u16 = 0x0806;
const CDXPROP_LINEWIDTH: u16 = 0x0807;
const CDXPROP_CAPTIONSTYLE: u16 = 0x080B;
const CDXPROP_CAPTIONSTYLEFONT: u16 = 0x081B;
const CDXPROP_CAPTIONSTYLESIZE: u16 = 0x081D;
const CDXPROP_CAPTIONSTYLEFACE: u16 = 0x081F;
const CDXPROP_GRAPHIC_TYPE: u16 = 0x0A00;
const CDXPROP_LINE_TYPE: u16 = 0x0A01;
const CDXPROP_ARROW_TYPE: u16 = 0x0A02;
const CDXPROP_RECTANGLE_TYPE: u16 = 0x0A03;
const CDXPROP_OVAL_TYPE: u16 = 0x0A04;
const CDXPROP_ORBITAL_TYPE: u16 = 0x0A05;
const CDXPROP_BRACKET_TYPE: u16 = 0x0A06;
const CDXPROP_SYMBOL_TYPE: u16 = 0x0A07;
const CDXPROP_ARROWHEAD_SIZE: u16 = 0x0A20;
const CDXPROP_ARC_ANGULARSIZE: u16 = 0x0A21;
const CDXPROP_BRACKET_LIPSIZE: u16 = 0x0A22;
const CDXPROP_BRACKET_USAGE: u16 = 0x0A24;
const CDXPROP_POLYMER_REPEATPATTERN: u16 = 0x0A25;
const CDXPROP_POLYMER_FLIPTYPE: u16 = 0x0A26;
const CDXPROP_CORNERRADIUS: u16 = 0x0A3C;
const CDXPROP_FRAME_TYPE: u16 = 0x0A3D;

impl TaggedObject for Graphic {
    const TAG: u16 = CDXOBJ_GRAPHIC;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut graphic = Graphic::new(raw.id);

        // Parse all properties using BinaryCodec
        graphic.z_order = raw
            .get_property(CDXPROP_ZORDER)
            .and_then(|v| i16::decode(v).ok());

        graphic.ignore_warnings = raw
            .get_property(CDXPROP_IGNOREWARNINGS)
            .and_then(|v| bool::decode(v).ok());

        graphic.chemical_warning = raw
            .get_property(CDXPROP_CHEMICALWARNING)
            .and_then(|v| String::from_utf8(v.to_vec()).ok());

        graphic.visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        graphic.superseded_by = raw
            .get_property(CDXPROP_SUPERSEDEDBY)
            .and_then(|v| u32::decode(v).ok());

        graphic.bounding_box = raw
            .get_property(CDXPROP_BOUNDINGBOX)
            .and_then(|v| Rectangle::decode(v).ok());

        graphic.head_3d = raw
            .get_property(CDXPROP_3DHEAD)
            .and_then(|v| Point3d::decode(v).ok());

        graphic.tail_3d = raw
            .get_property(CDXPROP_3DTAIL)
            .and_then(|v| Point3d::decode(v).ok());

        graphic.foreground_color = raw
            .get_property(CDXPROP_FOREGROUNDCOLOR)
            .and_then(|v| u16::decode(v).ok());

        graphic.background_color = raw
            .get_property(CDXPROP_BACKGROUNDCOLOR)
            .and_then(|v| i16::decode(v).ok());

        graphic.bold_width = raw
            .get_property(CDXPROP_BOLDWIDTH)
            .and_then(|v| f64::decode(v).ok());

        graphic.line_width = raw
            .get_property(CDXPROP_LINEWIDTH)
            .and_then(|v| f64::decode(v).ok());

        graphic.caption_style = raw
            .get_property(CDXPROP_CAPTIONSTYLE)
            .and_then(|v| u16::decode(v).ok());

        graphic.caption_style_font = raw
            .get_property(CDXPROP_CAPTIONSTYLEFONT)
            .and_then(|v| i16::decode(v).ok());

        graphic.caption_style_size = raw
            .get_property(CDXPROP_CAPTIONSTYLESIZE)
            .and_then(|v| i16::decode(v).ok());

        graphic.caption_style_face = raw
            .get_property(CDXPROP_CAPTIONSTYLEFACE)
            .and_then(|v| i16::decode(v).ok());

        graphic.graphic_type = raw
            .get_property(CDXPROP_GRAPHIC_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.line_type = raw
            .get_property(CDXPROP_LINE_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.arrow_type = raw
            .get_property(CDXPROP_ARROW_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.rectangle_type = raw
            .get_property(CDXPROP_RECTANGLE_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.oval_type = raw
            .get_property(CDXPROP_OVAL_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.orbital_type = raw
            .get_property(CDXPROP_ORBITAL_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.bracket_type = raw
            .get_property(CDXPROP_BRACKET_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.symbol_type = raw
            .get_property(CDXPROP_SYMBOL_TYPE)
            .and_then(|v| i16::decode(v).ok());

        graphic.arrowhead_size = raw
            .get_property(CDXPROP_ARROWHEAD_SIZE)
            .and_then(|v| i16::decode(v).ok());

        graphic.arc_angular_size = raw
            .get_property(CDXPROP_ARC_ANGULARSIZE)
            .and_then(|v| i16::decode(v).ok());

        graphic.bracket_lip_size = raw
            .get_property(CDXPROP_BRACKET_LIPSIZE)
            .and_then(|v| i16::decode(v).ok());

        graphic.bracket_usage = raw
            .get_property(CDXPROP_BRACKET_USAGE)
            .and_then(|v| i8::decode(v).ok());

        graphic.polymer_repeat_pattern = raw
            .get_property(CDXPROP_POLYMER_REPEATPATTERN)
            .and_then(|v| i8::decode(v).ok());

        graphic.polymer_flip_type = raw
            .get_property(CDXPROP_POLYMER_FLIPTYPE)
            .and_then(|v| i8::decode(v).ok());

        graphic.corner_radius = raw
            .get_property(CDXPROP_CORNERRADIUS)
            .and_then(|v| i16::decode(v).ok());

        graphic.frame_type = raw
            .get_property(CDXPROP_FRAME_TYPE)
            .and_then(|v| i16::decode(v).ok());

        Ok(graphic)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;

        let mut properties = Vec::new();

        if let Some(v) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ZORDER,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.ignore_warnings {
            properties.push(RawCdxProperty {
                tag: CDXPROP_IGNOREWARNINGS,
                value: v.encode()?,
            });
        }
        if let Some(ref v) = self.chemical_warning {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICALWARNING,
                value: v.as_bytes().to_vec(),
            });
        }
        if let Some(v) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.superseded_by {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SUPERSEDEDBY,
                value: v.encode()?,
            });
        }
        if let Some(ref v) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDINGBOX,
                value: v.encode()?,
            });
        }
        if let Some(ref v) = self.head_3d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_3DHEAD,
                value: v.encode()?,
            });
        }
        if let Some(ref v) = self.tail_3d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_3DTAIL,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.foreground_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOREGROUNDCOLOR,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.background_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BACKGROUNDCOLOR,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.bold_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOLDWIDTH,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.line_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINEWIDTH,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.caption_style {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTIONSTYLE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.caption_style_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTIONSTYLEFONT,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.caption_style_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTIONSTYLESIZE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.caption_style_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTIONSTYLEFACE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.graphic_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_GRAPHIC_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.line_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.arrow_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ARROW_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.rectangle_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_RECTANGLE_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.oval_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_OVAL_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.orbital_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ORBITAL_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.bracket_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.symbol_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SYMBOL_TYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.arrowhead_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ARROWHEAD_SIZE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.arc_angular_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ARC_ANGULARSIZE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.bracket_lip_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_LIPSIZE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.bracket_usage {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BRACKET_USAGE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.polymer_repeat_pattern {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POLYMER_REPEATPATTERN,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.polymer_flip_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POLYMER_FLIPTYPE,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.corner_radius {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CORNERRADIUS,
                value: v.encode()?,
            });
        }
        if let Some(v) = self.frame_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FRAME_TYPE,
                value: v.encode()?,
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
