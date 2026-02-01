//! Binary encoding/decoding for Document
//! The Document object is the top-level CDX object containing all document properties and content.

use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::document_tags::*;
use crate::error::CdxError;
use crate::cdx::document::Document;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};

impl TaggedObject for Document {
    const TAG: u16 = 0x8000; // kCDXObj_Document

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract optional creation/modification metadata using BinaryCodec
        let creation_user_name = raw
            .get_property(CDXPROP_CREATION_USER_NAME)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        let creation_date = raw
            .get_property(CDXPROP_CREATION_DATE)
            .and_then(|v| u32::decode(v).ok());

        let creation_program = raw
            .get_property(CDXPROP_CREATION_PROGRAM)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        let modification_user_name = raw
            .get_property(CDXPROP_MODIFICATION_USER_NAME)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        let modification_date = raw
            .get_property(CDXPROP_MODIFICATION_DATE)
            .and_then(|v| u32::decode(v).ok());

        let modification_program = raw
            .get_property(CDXPROP_MODIFICATION_PROGRAM)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        // Extract optional document metadata
        let name = raw
            .get_property(CDXPROP_NAME)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        let comment = raw
            .get_property(CDXPROP_COMMENT)
            .and_then(|v| crate::cdx::values::CDXString::decode(v).ok());

        // Extract optional geometry/appearance properties
        let bounding_box = raw
            .get_property(CDXPROP_BOUNDING_BOX)
            .and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());
        
        let color_table = raw
            .get_property(CDXPROP_COLOR_TABLE)
            .and_then(|v| crate::cdx::color_table::ColorTable::decode(v).ok());

        let atom_show_query = raw
            .get_property(CDXPROP_ATOM_SHOW_QUERY)
            .and_then(|v| bool::decode(v).ok());

        let atom_show_stereo = raw
            .get_property(CDXPROP_ATOM_SHOW_STEREO)
            .and_then(|v| bool::decode(v).ok());

        let atom_show_atom_number = raw
            .get_property(CDXPROP_ATOM_SHOW_ATOM_NUMBER)
            .and_then(|v| bool::decode(v).ok());

        let bond_show_query = raw
            .get_property(CDXPROP_BOND_SHOW_QUERY)
            .and_then(|v| bool::decode(v).ok());

        let bond_show_stereo = raw
            .get_property(CDXPROP_BOND_SHOW_STEREO)
            .and_then(|v| bool::decode(v).ok());

        let bond_show_rxn = raw
            .get_property(CDXPROP_BOND_SHOW_RXN)
            .and_then(|v| bool::decode(v).ok());

        // Extract text/line height settings
        let label_line_height = raw
            .get_property(CDXPROP_LABEL_LINE_HEIGHT)
            .and_then(|v| i16::decode(v).ok());

        let caption_line_height = raw
            .get_property(CDXPROP_CAPTION_LINE_HEIGHT)
            .and_then(|v| i16::decode(v).ok());

        let interpret_chemically = raw
            .get_property(CDXPROP_INTERPRET_CHEMICALLY)
            .and_then(|v| bool::decode(v).ok());

        // Extract printing/layout properties
        let mac_print_info = raw
            .get_property(CDXPROP_MAC_PRINT_INFO)
            .and_then(|v| Vec::<u8>::decode(v).ok());

        let win_print_info = raw
            .get_property(CDXPROP_WIN_PRINT_INFO)
            .and_then(|v| Vec::<u8>::decode(v).ok());

        let print_margins = raw
            .get_property(CDXPROP_PRINT_MARGINS)
            .and_then(|v| crate::cdx::values::Rectangle::decode(v).ok());

        // Extract bond/chain defaults
        let chain_angle = raw
            .get_property(CDXPROP_CHAIN_ANGLE)
            .and_then(|v| i32::decode(v).ok());

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

        // Extract justification/width settings
        let caption_justification = raw
            .get_property(CDXPROP_CAPTION_JUSTIFICATION)
            .and_then(|v| i8::decode(v).ok());

        let fractional_widths = raw
            .get_property(CDXPROP_FRACTIONAL_WIDTHS)
            .and_then(|v| bool::decode(v).ok());

        let magnification = raw
            .get_property(CDXPROP_MAGNIFICATION)
            .and_then(|v| i16::decode(v).ok());

        // Extract font defaults
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

        let label_justification = raw
            .get_property(CDXPROP_LABEL_JUSTIFICATION)
            .and_then(|v| i8::decode(v).ok());

        // Extract OLE / external data
        let fix_inplace_extent = raw
            .get_property(CDXPROP_FIX_INPLACE_EXTENT)
            .and_then(|v| crate::cdx::values::Point2d::decode(v).ok());

        let fix_inplace_gap = raw
            .get_property(CDXPROP_FIX_INPLACE_GAP)
            .and_then(|v| crate::cdx::values::Point2d::decode(v).ok());

        let cartridge_data = raw
            .get_property(CDXPROP_CARTRIDGE_DATA)
            .and_then(|v| Vec::<u8>::decode(v).ok());

        // Extract window state
        let window_is_zoomed = raw
            .get_property(CDXPROP_WINDOW_IS_ZOOMED)
            .and_then(|v| bool::decode(v).ok());

        let window_position = raw
            .get_property(CDXPROP_WINDOW_POSITION)
            .and_then(|v| crate::cdx::values::Point2d::decode(v).ok());

        let window_size = raw
            .get_property(CDXPROP_WINDOW_SIZE)
            .and_then(|v| crate::cdx::values::Point2d::decode(v).ok());

        Ok(Document {
            id: raw.id,
            creation_user_name,
            creation_date,
            creation_program,
            modification_user_name,
            modification_date,
            modification_program,
            name,
            comment,
            bounding_box,
            color_table,
            atom_show_query,
            atom_show_stereo,
            atom_show_atom_number,
            bond_show_query,
            bond_show_stereo,
            bond_show_rxn,
            label_line_height,
            caption_line_height,
            interpret_chemically,
            mac_print_info,
            win_print_info,
            print_margins,
            chain_angle,
            bond_spacing,
            bond_length,
            bold_width,
            line_width,
            margin_width,
            hash_spacing,
            caption_justification,
            fractional_widths,
            magnification,
            label_font,
            caption_font,
            label_size,
            caption_size,
            label_face,
            caption_face,
            label_color,
            caption_color,
            label_justification,
            fix_inplace_extent,
            fix_inplace_gap,
            cartridge_data,
            window_is_zoomed,
            window_position,
            window_size,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        
        let mut properties = Vec::new();
        
        // Optional properties - encode using BinaryCodec
        if let Some(ref val) = self.creation_user_name {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CREATION_USER_NAME,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.creation_date {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CREATION_DATE,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.creation_program {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CREATION_PROGRAM,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.modification_user_name {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MODIFICATION_USER_NAME,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.modification_date {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MODIFICATION_DATE,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.modification_program {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MODIFICATION_PROGRAM,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.name {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NAME,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.comment {
            properties.push(RawCdxProperty {
                tag: CDXPROP_COMMENT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDING_BOX,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.atom_show_query {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_QUERY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.atom_show_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.atom_show_atom_number {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_ATOM_NUMBER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_show_query {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_QUERY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_show_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.bond_show_rxn {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOND_SHOW_RXN,
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
        if let Some(ref val) = self.mac_print_info {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MAC_PRINT_INFO,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.win_print_info {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WIN_PRINT_INFO,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.print_margins {
            properties.push(RawCdxProperty {
                tag: CDXPROP_PRINT_MARGINS,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.chain_angle {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHAIN_ANGLE,
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
        if let Some(val) = self.caption_justification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CAPTION_JUSTIFICATION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.fractional_widths {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FRACTIONAL_WIDTHS,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.magnification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_MAGNIFICATION,
                value: val.encode()?,
            });
        }
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
        if let Some(val) = self.label_justification {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_JUSTIFICATION,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.fix_inplace_extent {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FIX_INPLACE_EXTENT,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.fix_inplace_gap {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FIX_INPLACE_GAP,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.cartridge_data {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CARTRIDGE_DATA,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.window_is_zoomed {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WINDOW_IS_ZOOMED,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.window_position {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WINDOW_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.window_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_WINDOW_SIZE,
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
