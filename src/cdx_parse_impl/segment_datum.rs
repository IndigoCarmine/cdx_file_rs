use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::seg_datum::SegDatum;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::segment_datum_tags::*;
use crate::error::CdxError;

impl TaggedObject for SegDatum {
    const TAG: u16 = CDXOBJ_SEGMENTDATUM;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let data_type = raw
            .get_property(CDXPROP_SEGMENTDATUM_TYPE)
            .ok_or_else(|| CdxError::Parse("Missing SGDataType".to_string()))
            .and_then(|v| {
                u8::decode(v).map_err(|_| CdxError::Parse("Invalid SGDataType".to_string()))
            })?;
        let sg_property_type = raw
            .get_property(CDXPROP_SEGMENTDATUM_PROPERTY_TYPE)
            .ok_or_else(|| CdxError::Parse("Missing SGPropertyType".to_string()))
            .and_then(|v| {
                u8::decode(v).map_err(|_| CdxError::Parse("Invalid SGPropertyType".to_string()))
            })?;
        let sg_data_value = raw
            .get_property(CDXPROP_SEGMENTDATUM_VALUE)
            .ok_or_else(|| CdxError::Parse("Missing SGDataValue".to_string()))
            .and_then(|v| {
                String::decode(v).map_err(|_| CdxError::Parse("Invalid SGDataValue".to_string()))
            })?;
        let is_read_only = raw
            .get_property(CDXPROP_SEGMENTDATUM_IS_READONLY)
            .and_then(|v| bool::decode(v).ok());
        Ok(SegDatum {
            id: raw.id,
            sg_data_type: data_type,
            sg_data_value,
            sg_property_type,
            is_read_only,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        properties.push(RawCdxProperty {
            tag: CDXPROP_SEGMENTDATUM_TYPE,
            value: self.sg_data_type.encode()?,
        });
        properties.push(RawCdxProperty {
            tag: CDXPROP_SEGMENTDATUM_PROPERTY_TYPE,
            value: self.sg_property_type.encode()?,
        });
        properties.push(RawCdxProperty {
            tag: CDXPROP_SEGMENTDATUM_VALUE,
            value: self.sg_data_value.encode()?,
        });
        if let Some(is_read_only) = self.is_read_only {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SEGMENTDATUM_IS_READONLY,
                value: is_read_only.encode()?,
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
