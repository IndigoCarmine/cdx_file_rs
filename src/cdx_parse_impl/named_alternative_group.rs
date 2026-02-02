use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::named_alternative_group::NamedAlternativeGroup;
use crate::cdx::values::{CDXString, Rectangle};
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::named_alternative_group_tags::*;
use crate::error::CdxError;

impl TaggedObject for NamedAlternativeGroup {
    const TAG: u16 = CDXOBJ_NAMED_ALTERNATIVE_GROUP;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut group = NamedAlternativeGroup::new(raw.id);

        // Parse common properties
        group.z_order = raw
            .get_property(CDXPROP_Z_ORDER)
            .and_then(|v| i16::decode(v).ok());

        group.ignore_warnings = raw
            .get_property(CDXPROP_IGNORE_WARNINGS)
            .and_then(|v| bool::decode(v).ok());

        group.chemical_warning = raw
            .get_property(CDXPROP_CHEMICAL_WARNING)
            .and_then(|v| CDXString::decode(v).ok());

        group.visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        // Parse geometry
        group.bounding_box = raw
            .get_property(CDXPROP_BOUNDING_BOX)
            .and_then(|v| Rectangle::decode(v).ok());

        // Parse colors
        group.foreground_color = raw
            .get_property(CDXPROP_FOREGROUND_COLOR)
            .and_then(|v| u16::decode(v).ok());

        group.background_color = raw
            .get_property(CDXPROP_BACKGROUND_COLOR)
            .and_then(|v| i16::decode(v).ok());

        // Parse Named Alternative Group-specific properties
        group.named_alternative_group_text_frame = raw
            .get_property(CDXPROP_NAMED_ALTERNATIVE_GROUP_TEXT_FRAME)
            .and_then(|v| Rectangle::decode(v).ok());

        group.named_alternative_group_group_frame = raw
            .get_property(CDXPROP_NAMED_ALTERNATIVE_GROUP_GROUP_FRAME)
            .and_then(|v| Rectangle::decode(v).ok());

        group.named_alternative_group_valence = raw
            .get_property(CDXPROP_NAMED_ALTERNATIVE_GROUP_VALENCE)
            .and_then(|v| i16::decode(v).ok());

        Ok(group)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Encode common properties
        if let Some(v) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_Z_ORDER,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.ignore_warnings {
            properties.push(RawCdxProperty {
                tag: CDXPROP_IGNORE_WARNINGS,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.chemical_warning {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_WARNING,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: v.encode()?,
            });
        }

        // Encode geometry
        if let Some(ref v) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: v.encode()?,
            });
        }

        // Encode colors
        if let Some(v) = self.foreground_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOREGROUND_COLOR,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.background_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BACKGROUND_COLOR,
                value: v.encode()?,
            });
        }

        // Encode Named Alternative Group-specific properties
        if let Some(ref v) = self.named_alternative_group_text_frame {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NAMED_ALTERNATIVE_GROUP_TEXT_FRAME,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.named_alternative_group_group_frame {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NAMED_ALTERNATIVE_GROUP_GROUP_FRAME,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.named_alternative_group_valence {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NAMED_ALTERNATIVE_GROUP_VALENCE,
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
