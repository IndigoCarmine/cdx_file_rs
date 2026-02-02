use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::object_tag::ObjectTag;
use crate::cdx::values::*;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011;

// Property tags
const CDXPROP_TAGNAME: u16 = 0x0A10;
const CDXPROP_ZORDER: u16 = 0x000A;
const CDXPROP_VISIBLE: u16 = 0x0011;
const CDXPROP_BOUNDINGBOX: u16 = 0x0204;
const CDXPROP_POSITION: u16 = 0x0203;
const CDXPROP_FOREGROUNDCOLOR: u16 = 0x0301;
const CDXPROP_TAGTYPE: u16 = 0x0A11;
const CDXPROP_TAGVALUE: u16 = 0x0A12;

impl TaggedObject for ObjectTag {
    const TAG: u16 = CDXOBJ_OBJECT_TAG;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut tag = ObjectTag::new(raw.id);

        tag.tag_name = raw
            .get_property(CDXPROP_TAGNAME)
            .and_then(|v| CDXString::decode(v).ok());

        tag.z_order = raw
            .get_property(CDXPROP_ZORDER)
            .and_then(|v| i16::decode(v).ok());

        tag.visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        tag.bounding_box = raw
            .get_property(CDXPROP_BOUNDINGBOX)
            .and_then(|v| Rectangle::decode(v).ok());

        tag.position = raw
            .get_property(CDXPROP_POSITION)
            .and_then(|v| Point2d::decode(v).ok());

        tag.foreground_color = raw
            .get_property(CDXPROP_FOREGROUNDCOLOR)
            .and_then(|v| u16::decode(v).ok());

        tag.tag_type = raw
            .get_property(CDXPROP_TAGTYPE)
            .and_then(|v| i16::decode(v).ok());

        tag.tag_value = raw
            .get_property(CDXPROP_TAGVALUE)
            .and_then(|v| String::from_utf8(v.to_vec()).ok());

        Ok(tag)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;

        let mut properties = Vec::new();

        if let Some(ref v) = self.tag_name {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TAGNAME,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ZORDER,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDINGBOX,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POSITION,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.foreground_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOREGROUNDCOLOR,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.tag_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TAGTYPE,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.tag_value {
            properties.push(RawCdxProperty {
                tag: CDXPROP_TAGVALUE,
                value: v.as_bytes().to_vec(),
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
