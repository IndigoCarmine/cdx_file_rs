use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::constraint::Constraint;
use crate::cdx::values::*;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;
use crate::cdx_tags::constraint_tags::*;

impl TaggedObject for Constraint {
    const TAG: u16 = CDXOBJ_CONSTRAINT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut constraint = Constraint::new(raw.id);

    //     constraint.z_order = raw
    //         .get_property(CDXPROP_ZORDER)
    //         .and_then(|v| i16::decode(v).ok());

    //     constraint.visible = raw
    //         .get_property(CDXPROP_VISIBLE)
    //         .and_then(|v| bool::decode(v).ok());

    //     constraint.bounding_box = raw
    //         .get_property(CDXPROP_BOUNDINGBOX)
    //         .and_then(|v| Rectangle::decode(v).ok());

    //     constraint.foreground_color = raw
    //         .get_property(CDXPROP_FOREGROUNDCOLOR)
    //         .and_then(|v| u16::decode(v).ok());

        Ok(constraint)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;

        let mut properties = Vec::new();

    //     if let Some(v) = self.z_order {
    //         properties.push(RawCdxProperty {
    //             tag: CDXPROP_ZORDER,
    //             value: v.encode()?,
    //         });
    //     }

    //     if let Some(v) = self.visible {
    //         properties.push(RawCdxProperty {
    //             tag: CDXPROP_VISIBLE,
    //             value: v.encode()?,
    //         });
    //     }

    //     if let Some(ref v) = self.bounding_box {
    //         properties.push(RawCdxProperty {
    //             tag: CDXPROP_BOUNDINGBOX,
    //             value: v.encode()?,
    //         });
    //     }

    //     if let Some(v) = self.foreground_color {
    //         properties.push(RawCdxProperty {
    //             tag: CDXPROP_FOREGROUNDCOLOR,
    //             value: v.encode()?,
    //         });
    //     }

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
