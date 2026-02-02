use crate::cdx::binary_codec::{decode_f64_array, encode_f64_array, BinaryCodec};
use crate::cdx::spectrum::Spectrum;
use crate::cdx::values::{CDXString, Rectangle};
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::spectrum_tags::*;
use crate::error::CdxError;

impl TaggedObject for Spectrum {
    const TAG: u16 = CDXOBJ_SPECTRUM;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut spectrum = Spectrum::new(raw.id);

        // Parse common properties
        spectrum.z_order = raw
            .get_property(CDXPROP_Z_ORDER)
            .and_then(|v| i16::decode(v).ok());

        spectrum.ignore_warnings = raw
            .get_property(CDXPROP_IGNORE_WARNINGS)
            .and_then(|v| bool::decode(v).ok());

        spectrum.chemical_warning = raw
            .get_property(CDXPROP_CHEMICAL_WARNING)
            .and_then(|v| CDXString::decode(v).ok());

        spectrum.visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        // Parse bounding box (Required)
        spectrum.bounding_box = raw
            .get_property(CDXPROP_BOUNDING_BOX)
            .and_then(|v| Rectangle::decode(v).ok());

        // Parse colors
        spectrum.foreground_color = raw
            .get_property(CDXPROP_FOREGROUND_COLOR)
            .and_then(|v| u16::decode(v).ok());

        spectrum.background_color = raw
            .get_property(CDXPROP_BACKGROUND_COLOR)
            .and_then(|v| i16::decode(v).ok());

        // Parse styling properties
        spectrum.bold_width = raw
            .get_property(CDXPROP_BOLD_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        spectrum.line_width = raw
            .get_property(CDXPROP_LINE_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        spectrum.label_style_font = raw
            .get_property(CDXPROP_LABEL_STYLE_FONT)
            .and_then(|v| i16::decode(v).ok());

        spectrum.label_style_size = raw
            .get_property(CDXPROP_LABEL_STYLE_SIZE)
            .and_then(|v| i16::decode(v).ok());

        spectrum.label_style_face = raw
            .get_property(CDXPROP_LABEL_STYLE_FACE)
            .and_then(|v| i16::decode(v).ok());

        // Parse spectrum-specific properties
        spectrum.spectrum_x_spacing = raw
            .get_property(CDXPROP_SPECTRUM_X_SPACING)
            .and_then(|v| f64::decode(v).ok());

        spectrum.spectrum_x_low = raw
            .get_property(CDXPROP_SPECTRUM_X_LOW)
            .and_then(|v| f64::decode(v).ok());

        spectrum.spectrum_x_type = raw
            .get_property(CDXPROP_SPECTRUM_X_TYPE)
            .and_then(|v| i8::decode(v).ok());

        spectrum.spectrum_y_type = raw
            .get_property(CDXPROP_SPECTRUM_Y_TYPE)
            .and_then(|v| i8::decode(v).ok());

        spectrum.spectrum_x_axis_label = raw
            .get_property(CDXPROP_SPECTRUM_X_AXIS_LABEL)
            .and_then(|v| CDXString::decode(v).ok());

        spectrum.spectrum_y_axis_label = raw
            .get_property(CDXPROP_SPECTRUM_Y_AXIS_LABEL)
            .and_then(|v| CDXString::decode(v).ok());

        // Parse data points (array of f64)
        spectrum.spectrum_data_point = raw
            .get_property(CDXPROP_SPECTRUM_DATA_POINT)
            .and_then(|v| decode_f64_array(v).ok());

        spectrum.spectrum_class = raw
            .get_property(CDXPROP_SPECTRUM_CLASS)
            .and_then(|v| i8::decode(v).ok());

        spectrum.spectrum_y_low = raw
            .get_property(CDXPROP_SPECTRUM_Y_LOW)
            .and_then(|v| f64::decode(v).ok());

        spectrum.spectrum_y_scale = raw
            .get_property(CDXPROP_SPECTRUM_Y_SCALE)
            .and_then(|v| f64::decode(v).ok());

        Ok(spectrum)
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

        // Encode bounding box
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

        // Encode styling
        if let Some(v) = self.bold_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOLD_WIDTH,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.line_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_WIDTH,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.label_style_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_FONT,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.label_style_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_SIZE,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.label_style_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_STYLE_FACE,
                value: v.encode()?,
            });
        }

        // Encode spectrum-specific properties
        if let Some(v) = self.spectrum_x_spacing {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_X_SPACING,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.spectrum_x_low {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_X_LOW,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.spectrum_x_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_X_TYPE,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.spectrum_y_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_Y_TYPE,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.spectrum_x_axis_label {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_X_AXIS_LABEL,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.spectrum_y_axis_label {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_Y_AXIS_LABEL,
                value: v.encode()?,
            });
        }

        if let Some(ref v) = self.spectrum_data_point {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_DATA_POINT,
                value: encode_f64_array(v)?,
            });
        }

        if let Some(v) = self.spectrum_class {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_CLASS,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.spectrum_y_low {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_Y_LOW,
                value: v.encode()?,
            });
        }

        if let Some(v) = self.spectrum_y_scale {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_Y_SCALE,
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
