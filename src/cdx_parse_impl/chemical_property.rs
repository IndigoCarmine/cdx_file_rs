use crate::cdx::binary_codec::{decode_u32_array, encode_object_id_array, BinaryCodec};
use crate::cdx::chemical_property::ChemicalProperty;
use crate::cdx::values::Point2d;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::chemical_property_tags::*;
use crate::error::CdxError;

impl TaggedObject for ChemicalProperty {
    const TAG: u16 = CDXOBJ_CHEMICAL_PROPERTY;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut property = ChemicalProperty::new(raw.id);

        // Parse name (CDXString stored as plain text for now)
        property.name = raw
            .get_property(CDXPROP_NAME)
            .and_then(|v| String::decode(v).ok());

        // Parse basis objects (CDXObjectIDArray)
        property.basis_objects = raw
            .get_property(CDXPROP_BASIS_OBJECTS)
            .and_then(|v| decode_u32_array(v).ok());

        // Parse chemical property type (UINT32 -> i8 per struct definition)
        property.chemical_property_type = raw
            .get_property(CDXPROP_CHEMICAL_PROPERTY_TYPE)
            .and_then(|v| i8::decode(v).ok());

        // Parse display ID (CDXObjectID -> u32)
        property.chemical_property_display_id = raw
            .get_property(CDXPROP_CHEMICAL_PROPERTY_DISPLAY_ID)
            .and_then(|v| u32::decode(v).ok());

        // Parse is_active flag (CDXBoolean)
        property.chemical_property_is_active = raw
            .get_property(CDXPROP_CHEMICAL_PROPERTY_IS_ACTIVE)
            .and_then(|v| bool::decode(v).ok());

        // Parse positioning type (INT8)
        property.positioning = raw
            .get_property(CDXPROP_POSITIONING)
            .and_then(|v| i8::decode(v).ok());

        // Parse positioning angle (INT32)
        property.positioning_angle = raw
            .get_property(CDXPROP_POSITIONING_ANGLE)
            .and_then(|v| i32::decode(v).ok());

        // Parse positioning offset (CDXPoint2D)
        property.positioning_offset = raw
            .get_property(CDXPROP_POSITIONING_OFFSET)
            .and_then(|v| Point2d::decode(v).ok());

        Ok(property)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        if let Some(ref v) = self.name {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NAME,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.basis_objects {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BASIS_OBJECTS,
                value: encode_object_id_array(v)?,
            });
        }

        if let Some(v) = self.chemical_property_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_PROPERTY_TYPE,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.chemical_property_display_id {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_PROPERTY_DISPLAY_ID,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.chemical_property_is_active {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_PROPERTY_IS_ACTIVE,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.positioning {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POSITIONING,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.positioning_angle {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POSITIONING_ANGLE,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.positioning_offset {
            properties.push(RawCdxProperty {
                tag: CDXPROP_POSITIONING_OFFSET,
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
