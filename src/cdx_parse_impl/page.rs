/// Binary encoding/decoding for Page
/// The Page object divides objects into separate drawing spaces and coordinate systems.
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::CDXString;
use crate::cdx_tags::page_tags::*;
use crate::error::CdxError;
use crate::cdx::page::Page;

impl TaggedObject for Page {
    const TAG: u16 = CDXOBJ_PAGE;
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional properties using BinaryCodec
        let bounding_box = raw.get_property(CDXPROP_BOUNDING_BOX).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        let z_order = raw.get_property(CDXPROP_Z_ORDER).and_then(|v| i16::decode(v).ok());
        let ignore_warnings = raw.get_property(CDXPROP_IGNORE_WARNINGS).and_then(|v| bool::decode(v).ok());
        let chemical_warning = raw.get_property(CDXPROP_CHEMICAL_WARNING).and_then(|v| CDXString::decode(v).ok());
        let visible = raw.get_property(CDXPROP_VISIBLE).and_then(|v| bool::decode(v).ok());
        let foreground_color = raw.get_property(CDXPROP_FOREGROUND_COLOR).and_then(|v| u16::decode(v).ok());
        let background_color = raw.get_property(CDXPROP_BACKGROUND_COLOR).and_then(|v| i16::decode(v).ok());
        let width_pages = raw.get_property(CDXPROP_WIDTH_PAGES).and_then(|v| i16::decode(v).ok());
        let height_pages = raw.get_property(CDXPROP_HEIGHT_PAGES).and_then(|v| i16::decode(v).ok());
        let drawing_space_type = raw.get_property(CDXPROP_DRAWING_SPACE_TYPE).and_then(|v| i8::decode(v).ok());
        let width = raw.get_property(CDXPROP_WIDTH).and_then(|v| f64::decode(v).ok());
        let height = raw.get_property(CDXPROP_HEIGHT).and_then(|v| f64::decode(v).ok());
        let page_overlap = raw.get_property(CDXPROP_PAGE_OVERLAP).and_then(|v| f64::decode(v).ok());
        let header = raw.get_property(CDXPROP_HEADER).and_then(|v| CDXString::decode(v).ok());
        let header_position = raw.get_property(CDXPROP_HEADER_POSITION).and_then(|v| f64::decode(v).ok());
        let footer = raw.get_property(CDXPROP_FOOTER).and_then(|v| CDXString::decode(v).ok());
        let footer_position = raw.get_property(CDXPROP_FOOTER_POSITION).and_then(|v| f64::decode(v).ok());
        let print_trim_marks = raw.get_property(CDXPROP_PRINT_TRIM_MARKS).and_then(|v| bool::decode(v).ok());
        let splitter_positions = raw.get_property(CDXPROP_SPLITTER_POSITIONS).and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());
        let page_definition = raw.get_property(CDXPROP_PAGE_DEFINITION).and_then(|v| i8::decode(v).ok());
        let bounds_in_parent = raw.get_property(CDXPROP_BOUNDS_IN_PARENT).and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());

        Ok(Page {
            id: raw.id,
            bounding_box,
            z_order,
            ignore_warnings,
            chemical_warning,
            visible,
            foreground_color,
            background_color,
            width_pages,
            height_pages,
            drawing_space_type,
            width,
            height,
            page_overlap,
            header,
            header_position,
            footer,
            footer_position,
            print_trim_marks,
            splitter_positions,
            page_definition,
            bounds_in_parent,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        
        let mut properties = Vec::new();
        
        // Optional properties - encode using BinaryCodec
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.background_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BACKGROUND_COLOR,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.width_pages {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WIDTH_PAGES,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.height_pages {
            properties.push(RawCdxProperty {
                tag: CDXPROP_HEIGHT_PAGES,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.drawing_space_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_DRAWING_SPACE_TYPE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.height {
            properties.push(RawCdxProperty {
                tag: CDXPROP_HEIGHT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.page_overlap {
            properties.push(RawCdxProperty {
                tag: CDXPROP_PAGE_OVERLAP,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.header {
            properties.push(RawCdxProperty {
                tag: CDXPROP_HEADER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.header_position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_HEADER_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.footer {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOOTER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.footer_position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOOTER_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.print_trim_marks {
            properties.push(RawCdxProperty {
                tag: CDXPROP_PRINT_TRIM_MARKS,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.splitter_positions {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPLITTER_POSITIONS,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
            });
        }
        if let Some(val) = self.page_definition {
            properties.push(RawCdxProperty {
                tag: CDXPROP_PAGE_DEFINITION,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bounds_in_parent {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDS_IN_PARENT,
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
