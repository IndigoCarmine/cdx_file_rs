use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

use crate::cdx::unknown::*;

macro_rules!   generate_tagged_object_impl {
    ( $name:ident, $tag:expr ) => {
        impl TaggedObject for $name {
            const TAG: u16 = $tag;

            fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
                let mut obj = $name::new(raw.id);

                // Store all properties as-is for potential roundtrip
                for prop in &raw.properties {
                    obj.raw_properties.push((prop.tag, prop.value.clone()));
                }

                Ok(obj)
            }

            fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
                use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;

                let properties: Vec<RawCdxProperty> = self
                    .raw_properties
                    .iter()
                    .map(|(tag, value)| RawCdxProperty {
                        tag: *tag,
                        value: value.clone(),
                    })
                    .collect();

                Ok(RawCdxObject {
                    tag: Self::TAG,
                    id: self.id,
                    properties,
                    children: Vec::new(),
                })
            }
        }
    }
}




//off
generate_tagged_object_impl!(UnknownObject801D, 0x8024);
generate_tagged_object_impl!(UnknownObject802B, 0x8023);
generate_tagged_object_impl!(UnknownObject801E, 0x8022);
generate_tagged_object_impl!(UnknownObject801F, 0xFFFF);