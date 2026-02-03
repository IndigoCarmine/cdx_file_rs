use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::cross_reference::CrossReference;
use crate::cdx::values::CDXString;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::cross_reference_tags::*;
use crate::error::CdxError;

impl TaggedObject for CrossReference {
    const TAG: u16 = CDXOBJ_CROSS_REFERENCE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut cross_ref = CrossReference::new(raw.id);

        // Parse cross_reference_container (stored as u32 in struct, but spec says CDXString)
        // Using u32 decode as per struct definition
        cross_ref.cross_reference_container = raw
            .get_property(CDXPROP_CROSS_REFERENCE_CONTAINER)
            .and_then(|v| u32::decode(v).ok());

        // Parse cross_reference_document (CDXString)
        cross_ref.cross_reference_document = raw
            .get_property(CDXPROP_CROSS_REFERENCE_DOCUMENT)
            .and_then(|v| CDXString::decode(v).ok());

        // Parse cross_reference_identifier (CDXString) - Required
        cross_ref.cross_reference_identifier = raw
            .get_property(CDXPROP_CROSS_REFERENCE_IDENTIFIER)
            .and_then(|v| CDXString::decode(v).ok());

        // Parse cross_reference_sequence (CDXString) - Required
        cross_ref.cross_reference_sequence = raw
            .get_property(CDXPROP_CROSS_REFERENCE_SEQUENCE)
            .and_then(|v| CDXString::decode(v).ok());

        Ok(cross_ref)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        if let Some(v) = self.cross_reference_container {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSS_REFERENCE_CONTAINER,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.cross_reference_document {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSS_REFERENCE_DOCUMENT,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.cross_reference_identifier {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSS_REFERENCE_IDENTIFIER,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.cross_reference_sequence {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CROSS_REFERENCE_SEQUENCE,
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
