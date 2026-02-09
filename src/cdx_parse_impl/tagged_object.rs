/// Binary encoding/decoding for NodeImpl
/// The Node object represents atoms or attachment points in chemical structures.
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;

pub trait TaggedObject: Sized {
    const TAG: u16;
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError>;
    fn to_raw(&self) -> Result<RawCdxObject, CdxError>;
}

#[macro_export]
macro_rules! impl_tagged_object {
    ($type:ty, $tag:expr, { $($field:ident : $prop_tag:expr),* $(,)? }) => {
        impl $crate::cdx_parse_impl::tagged_object::TaggedObject for $type {
            const TAG: u16 = $tag;

            fn from_raw(raw: $crate::cdx_parse_impl::raw_nodes::RawCdxObject) -> Result<Self, $crate::error::CdxError> {
                let mut obj = <$type>::new(raw.id);
                $(
                    let val = raw.get_prop($prop_tag);
                    if let Ok(val) = val {
                        obj.$field = val;
                    }else if let Err(e) = val {
                        println!("Get error: {:?}", e);
                        println!("Warning: Missing property tag=0x{:x} in object tag=0x{:x}", $prop_tag, raw.tag);
                        println!("Raw object properties: {:?}", raw.get_property($prop_tag));
                    }
                )*
                Ok(obj)
            }

            fn to_raw(&self) -> Result<$crate::cdx_parse_impl::raw_nodes::RawCdxObject, $crate::error::CdxError> {
                let mut raw = $crate::cdx_parse_impl::raw_nodes::RawCdxObject::new(Self::TAG, self.id);
                $(
                    if let Some(val) = &self.$field {
                        raw.set_prop($prop_tag, val)?;
                    }
                )*
                Ok(raw)
            }
        }
    };
}
