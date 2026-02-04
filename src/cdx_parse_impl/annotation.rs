use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_tags::anotation_tags::*;
use crate::error::CdxError;
use crate::cdx::values::CDXString;
use crate::cdx::annotation::Annotation;

impl TaggedObject for Annotation{
    const TAG: u16 = CDXOBJ_ANNOTATION;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut obj = Annotation::new(raw.id);

        for prop in &raw.properties {
            match prop.tag {
                CDXPROP_ANNOTATION_KEYWORD => {
                    obj.keyword = Some(CDXString::decode(&prop.value)?);
                }
                CDXPROP_ANNOTATION_CONTENT => {
                    obj.content = Some(CDXString::decode(&prop.value)?);
                }
                _ => {
                    // Ignore unknown properties for now
                }
            }
        }

        Ok(obj)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;

        let mut properties: Vec<RawCdxProperty> = Vec::new();

        if let Some(keyword) = &self.keyword {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ANNOTATION_KEYWORD,
                value: keyword.encode()?,
            });
        }

        if let Some(content) = &self.content {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ANNOTATION_CONTENT,
                value: content.encode()?,
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