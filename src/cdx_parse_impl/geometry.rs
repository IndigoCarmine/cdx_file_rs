/// Binary encoding/decoding for Geometry
/// The Geometry object represents geometrical relationships between objects.
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::geometry_tags::*;
use crate::error::CdxError;
use crate::cdx::geometry::Geometry;

impl TaggedObject for Geometry {
    const TAG: u16 = CDXOBJ_GEOMETRY;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let visible = raw.get_property(CDXPROP_VISIBLE).and_then(|v| bool::decode(v).ok());
        let z_order = raw.get_property(CDXPROP_Z_ORDER).and_then(|v| i16::decode(v).ok());
        let bounding_box = raw.get_property(CDXPROP_BOUNDING_BOX).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let position = raw.get_property(CDXPROP_2D_POSITION).and_then(|v| crate::cdx::values::Point2d::decode(v).ok());
        let rotation_angle = raw.get_property(CDXPROP_ROTATION_ANGLE).and_then(|v| i32::decode(v).ok());
        let foreground_color = raw.get_property(CDXPROP_FOREGROUND_COLOR).and_then(|v| u16::decode(v).ok());
        let background_color = raw.get_property(CDXPROP_BACKGROUND_COLOR).and_then(|v| i16::decode(v).ok());

        Ok(Geometry {
            id: raw.id,
            visible,
            z_order,
            bounding_box,
            position,
            rotation_angle,
            foreground_color,
            background_color,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        
        let mut properties = Vec::new();
        
        // Optional properties - encode using BinaryCodec
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_Z_ORDER,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.rotation_angle {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ROTATION_ANGLE,
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
        
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
