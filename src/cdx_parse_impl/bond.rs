use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::bond::Bond;
use crate::cdx::values::CDXString;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::bond_tags::*;
use crate::error::CdxError;

impl TaggedObject for Bond {
    const TAG: u16 = CDXOBJ_BOND;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract begin and end (treating as optional to support CDX files that may not have them)
        let begin = raw
            .get_property(CDXPROP_BOND_BEGIN)
            .and_then(|v| u32::decode(v).ok())
            .unwrap_or(0);

        let end = raw
            .get_property(CDXPROP_BOND_END)
            .and_then(|v| u32::decode(v).ok())
            .unwrap_or(0);

        // Extract optional properties using BinaryCodec
        let z_order = raw
            .get_property(CDXPROP_Z_ORDER)
            .and_then(|v| i16::decode(v).ok());

        let ignore_warnings = raw
            .get_property(CDXPROP_IGNORE_WARNINGS)
            .and_then(|v| bool::decode(v).ok());

        let chemical_warning = raw
            .get_property(CDXPROP_CHEMICAL_WARNING)
            .and_then(|v| CDXString::decode(v).ok());

        let visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        let foreground_color = raw
            .get_property(CDXPROP_FOREGROUND_COLOR)
            .and_then(|v| u16::decode(v).ok());

        let background_color = raw
            .get_property(CDXPROP_BACKGROUND_COLOR)
            .and_then(|v| i16::decode(v).ok());

        let bond_order = raw
            .get_property(CDXPROP_BOND_ORDER)
            .and_then(|v| i16::decode(v).ok());

        let display = raw
            .get_property(CDXPROP_BOND_DISPLAY)
            .and_then(|v| i16::decode(v).ok());

        let display2 = raw
            .get_property(CDXPROP_BOND_DISPLAY2)
            .and_then(|v| i16::decode(v).ok());

        let double_position = raw
            .get_property(CDXPROP_BOND_DOUBLE_POSITION)
            .and_then(|v| i16::decode(v).ok());

        let topology = raw
            .get_property(CDXPROP_BOND_RESTRICT_TOPOLOGY)
            .and_then(|v| i8::decode(v).ok());

        let rxn_participation = raw
            .get_property(CDXPROP_BOND_RESTRICT_RXN_PARTICIPATION)
            .and_then(|v| i8::decode(v).ok());

        let begin_attach = raw
            .get_property(CDXPROP_BOND_BEGIN_ATTACH)
            .and_then(|v| u8::decode(v).ok());

        let end_attach = raw
            .get_property(CDXPROP_BOND_END_ATTACH)
            .and_then(|v| u8::decode(v).ok());

        let cip_stereochemistry = raw
            .get_property(CDXPROP_BOND_CIP_STEREOCHEMISTRY)
            .and_then(|v| i8::decode(v).ok());

        let bond_circular_ordering = raw
            .get_property(CDXPROP_BOND_CIRCULAR_ORDERING)
            .and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());

        let show_query = raw
            .get_property(CDXPROP_BOND_SHOW_QUERY)
            .and_then(|v| bool::decode(v).ok());

        let show_stereo = raw
            .get_property(CDXPROP_BOND_SHOW_STEREO)
            .and_then(|v| bool::decode(v).ok());

        let crossing_bonds = raw
            .get_property(CDXPROP_BOND_CROSSING_BONDS)
            .and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());

        let show_rxn = raw
            .get_property(CDXPROP_BOND_SHOW_RXN)
            .and_then(|v| bool::decode(v).ok());

        let bond_spacing = raw
            .get_property(CDXPROP_BOND_SPACING)
            .and_then(|v| i16::decode(v).ok());

        let bond_length = raw
            .get_property(CDXPROP_BOND_LENGTH)
            .and_then(|v| f64::decode(v).ok());

        let bold_width = raw
            .get_property(CDXPROP_BOLD_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        let line_width = raw
            .get_property(CDXPROP_LINE_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        let margin_width = raw
            .get_property(CDXPROP_MARGIN_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        let hash_spacing = raw
            .get_property(CDXPROP_HASH_SPACING)
            .and_then(|v| f64::decode(v).ok());

        let label_font = raw
            .get_property(CDXPROP_LABEL_FONT)
            .and_then(|v| i16::decode(v).ok());

        let label_size = raw
            .get_property(CDXPROP_LABEL_SIZE)
            .and_then(|v| i16::decode(v).ok());

        let label_face = raw
            .get_property(CDXPROP_LABEL_FACE)
            .and_then(|v| i16::decode(v).ok());

        let bond_spacing_abs = raw
            .get_property(CDXPROP_BOND_SPACING_ABS)
            .and_then(|v| f64::decode(v).ok());

        Ok(Bond {
            id: raw.id,
            begin,
            end,
            z_order,
            ignore_warnings,
            chemical_warning,
            visible,
            foreground_color,
            background_color,
            bond_order,
            display,
            display2,
            double_position,
            topology,
            rxn_participation,
            begin_attach,
            end_attach,
            cip_stereochemistry,
            bond_circular_ordering,
            show_query,
            show_stereo,
            crossing_bonds,
            show_rxn,
            bond_spacing,
            bond_length,
            bold_width,
            line_width,
            margin_width,
            hash_spacing,
            label_font,
            label_size,
            label_face,
            bond_spacing_abs,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Required properties
        properties.push(RawCdxProperty {
            tag: CDXPROP_BOND_BEGIN,
            value: self.begin.encode()?,
        });
        properties.push(RawCdxProperty {
            tag: CDXPROP_BOND_END,
            value: self.end.encode()?,
        });

        // Optional properties - encode using BinaryCodec
        if let Some(val) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_Z_ORDER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.ignore_warnings {
            properties.push(RawCdxProperty {
                tag: CDXPROP_IGNORE_WARNINGS,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.chemical_warning {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_WARNING,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
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
        if let Some(val) = self.bond_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_ORDER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.display {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_DISPLAY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.display2 {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_DISPLAY2,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.double_position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_DOUBLE_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.topology {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_RESTRICT_TOPOLOGY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.rxn_participation {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_RESTRICT_RXN_PARTICIPATION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.begin_attach {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_BEGIN_ATTACH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.end_attach {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_END_ATTACH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.cip_stereochemistry {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_CIP_STEREOCHEMISTRY,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bond_circular_ordering {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_CIRCULAR_ORDERING,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
            });
        }
        if let Some(val) = self.show_query {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_QUERY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.show_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.crossing_bonds {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_CROSSING_BONDS,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
            });
        }
        if let Some(val) = self.show_rxn {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_RXN,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_spacing {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SPACING,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_length {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_LENGTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bold_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOLD_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.line_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.margin_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MARGIN_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.hash_spacing {
            properties.push(RawCdxProperty {
                tag: CDXPROP_HASH_SPACING,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FONT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_SIZE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FACE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_spacing_abs {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SPACING_ABS,
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
