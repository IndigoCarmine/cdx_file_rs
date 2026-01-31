use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx::values::*;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::CDXString;
use crate::cdx_tags::text_tags::*;
use crate::error::CdxError;
use crate::cdx::text::TextObject;


impl TaggedObject for TextObject {
    const TAG: u16 = CDXOBJ_TEXT;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Required: text content
        let text = raw
            .get_property(CDXPROP_TEXT)
            .ok_or_else(|| CdxError::DecodeError("Text object missing required text".to_string()))
            .and_then(|v| CDXString::decode(v))?;

        // Optional: common properties
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

        // Position and geometry
        let position_2d = raw
            .get_property(CDXPROP_2D_POSITION)
            .and_then(|v| Point2d::decode(v).ok());

        let bounding_box = raw
            .get_property(CDXPROP_BOUNDING_BOX)
            .and_then(|v| {
                if v.len() == 32 {
                    use byteorder::{LittleEndian, ReadBytesExt};
                    use std::io::Cursor;
                    let mut cursor = Cursor::new(v);
                    let top = cursor.read_f64::<LittleEndian>().ok()?;
                    let left = cursor.read_f64::<LittleEndian>().ok()?;
                    let bottom = cursor.read_f64::<LittleEndian>().ok()?;
                    let right = cursor.read_f64::<LittleEndian>().ok()?;
                    Some(Rectangle { top, left, bottom, right })
                } else {
                    None
                }
            });

        let rotation_angle = raw
            .get_property(CDXPROP_ROTATION_ANGLE)
            .and_then(|v| i32::decode(v).ok());

        // Text formatting
        let justification = raw
            .get_property(CDXPROP_JUSTIFICATION)
            .and_then(|v| i8::decode(v).ok());
        
        let line_height = raw
            .get_property(CDXPROP_LINE_HEIGHT)
            .and_then(|v| u16::decode(v).ok());
        
        let word_wrap_width = raw
            .get_property(CDXPROP_WORD_WRAP_WIDTH)
            .and_then(|v| i16::decode(v).ok());

        let label_alignment = raw
            .get_property(CDXPROP_LABEL_ALIGNMENT)
            .and_then(|v| i8::decode(v).ok());
        
        let label_line_height = raw
            .get_property(CDXPROP_LABEL_LINE_HEIGHT)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_line_height = raw
            .get_property(CDXPROP_CAPTION_LINE_HEIGHT)
            .and_then(|v| i16::decode(v).ok());
        
        let interpret_chemically = raw
            .get_property(CDXPROP_INTERPRET_CHEMICALLY)
            .and_then(|v| bool::decode(v).ok());

        // Font/style properties
        let label_font = raw
            .get_property(CDXPROP_LABEL_FONT)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_font = raw
            .get_property(CDXPROP_CAPTION_FONT)
            .and_then(|v| i16::decode(v).ok());
        
        let label_size = raw
            .get_property(CDXPROP_LABEL_SIZE)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_size = raw
            .get_property(CDXPROP_CAPTION_SIZE)
            .and_then(|v| i16::decode(v).ok());
        
        let label_face = raw
            .get_property(CDXPROP_LABEL_FACE)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_face = raw
            .get_property(CDXPROP_CAPTION_FACE)
            .and_then(|v| i16::decode(v).ok());
        
        let label_color = raw
            .get_property(CDXPROP_LABEL_COLOR)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_color = raw
            .get_property(CDXPROP_CAPTION_COLOR)
            .and_then(|v| i16::decode(v).ok());
        
        let caption_justification = raw
            .get_property(CDXPROP_CAPTION_JUSTIFICATION)
            .and_then(|v| i8::decode(v).ok());
        
        let label_justification = raw
            .get_property(CDXPROP_LABEL_JUSTIFICATION)
            .and_then(|v| i8::decode(v).ok());

        // Parse line_starts if present (INT16 list with counts)
        let line_starts = raw
            .get_property(CDXPROP_LINE_STARTS)
            .and_then(|v| {
                if v.len() >= 2 {
                    use byteorder::{LittleEndian, ReadBytesExt};
                    use std::io::Cursor;
                    let mut cursor = Cursor::new(v);
                    let count = cursor.read_i16::<LittleEndian>().ok()?;
                    let mut starts = Vec::new();
                    for _ in 0..count {
                        starts.push(cursor.read_i16::<LittleEndian>().ok()?);
                    }
                    Some(starts)
                } else {
                    None
                }
            });

        Ok(TextObject {
            id: raw.id,
            text,
            z_order,
            ignore_warnings,
            chemical_warning,
            visible,
            position_2d,
            bounding_box,
            rotation_angle,
            justification,
            line_height,
            word_wrap_width,
            line_starts,
            label_alignment,
            label_line_height,
            caption_line_height,
            interpret_chemically,
            label_font,
            caption_font,
            label_size,
            caption_size,
            label_face,
            caption_face,
            label_color,
            caption_color,
            caption_justification,
            label_justification,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {

        let mut properties = Vec::new();

        // Required: text
        properties.push(RawCdxProperty {
            tag: CDXPROP_TEXT,
            value: self.text.encode()?,
        });

        // Optional: common properties
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

        // Position and geometry
        if let Some(val) = &self.position_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_POSITION,
                value: val.encode()?,
            });
        }

        if let Some(rect) = &self.bounding_box {
            use byteorder::{LittleEndian, WriteBytesExt};
            let mut buf = Vec::with_capacity(32);
            buf.write_f64::<LittleEndian>(rect.top)
                .map_err(|e| CdxError::EncodeError(format!("Failed to write top: {}", e)))?;
            buf.write_f64::<LittleEndian>(rect.left)
                .map_err(|e| CdxError::EncodeError(format!("Failed to write left: {}", e)))?;
            buf.write_f64::<LittleEndian>(rect.bottom)
                .map_err(|e| CdxError::EncodeError(format!("Failed to write bottom: {}", e)))?;
            buf.write_f64::<LittleEndian>(rect.right)
                .map_err(|e| CdxError::EncodeError(format!("Failed to write right: {}", e)))?;
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
        value: buf,
    });
        }

        if let Some(val) = self.rotation_angle {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ROTATION_ANGLE,
                value: val.encode()?,
            });
        }

        // Text formatting
        if let Some(val) = self.justification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_JUSTIFICATION,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.line_height {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_HEIGHT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.word_wrap_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WORD_WRAP_WIDTH,
                value: val.encode()?,
            });
        }

        if let Some(ref val) = self.line_starts {
            use byteorder::{LittleEndian, WriteBytesExt};
            let mut buf = Vec::new();
            buf.write_i16::<LittleEndian>(val.len() as i16)?;
            for &start in val {
                buf.write_i16::<LittleEndian>(start)?;
            }
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_STARTS,
                value: buf,
            });
        }

        if let Some(val) = self.label_alignment {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_ALIGNMENT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.label_line_height {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_LINE_HEIGHT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_line_height {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_LINE_HEIGHT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.interpret_chemically {
            properties.push(RawCdxProperty {
                tag: CDXPROP_INTERPRET_CHEMICALLY,
                value: val.encode()?,
            });
        }

        // Font/style properties
        if let Some(val) = self.label_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FONT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_FONT,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.label_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_SIZE,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_SIZE,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.label_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FACE,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_FACE,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.label_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_COLOR,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_COLOR,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.caption_justification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_JUSTIFICATION,
                value: val.encode()?,
            });
        }

        if let Some(val) = self.label_justification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_JUSTIFICATION,
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
