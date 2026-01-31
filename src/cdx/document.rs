use serde::{Deserialize, Serialize};
use crate::cdx::values::{Point2d, Rectangle, CDXString};
use crate::cdx::color_table::ColorTable;
/// Document Object: The top-level CDX object
/// A Document is the top-level CDX object. It contains all CDX properties and objects.
/// It is necessary (by definition) for any valid CDX or CDXML file.
/// A Document must contain at least one Page object, but it has no required properties.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub id: u32,
    
    // Creation/Modification metadata
    pub creation_user_name: Option<CDXString>,
    pub creation_date: Option<u32>,
    pub creation_program: Option<CDXString>,
    pub modification_user_name: Option<CDXString>,
    pub modification_date: Option<u32>,
    pub modification_program: Option<CDXString>,
    
    // Document metadata
    pub name: Option<CDXString>,
    pub comment: Option<CDXString>,
    
    // Geometry / Appearance
    pub bounding_box: Option<Rectangle>,
    pub color_table: Option<ColorTable>,
    pub atom_show_query: Option<bool>,
    pub atom_show_stereo: Option<bool>,
    pub atom_show_atom_number: Option<bool>,
    pub bond_show_query: Option<bool>,
    pub bond_show_stereo: Option<bool>,
    pub bond_show_rxn: Option<bool>,
    
    // Text/Line Height settings
    pub label_line_height: Option<i16>,
    pub caption_line_height: Option<i16>,
    pub interpret_chemically: Option<bool>,
    
    // Printing / Layout
    pub mac_print_info: Option<Vec<u8>>,
    pub win_print_info: Option<Vec<u8>>,
    pub print_margins: Option<Rectangle>,
    
    // Bond/Chain defaults
    pub chain_angle: Option<i32>,
    pub bond_spacing: Option<i16>,
    pub bond_length: Option<f64>,
    pub bold_width: Option<f64>,
    pub line_width: Option<f64>,
    pub margin_width: Option<f64>,
    pub hash_spacing: Option<f64>,
    
    // Justification/Width settings
    pub caption_justification: Option<i8>,
    pub fractional_widths: Option<bool>,
    pub magnification: Option<i16>,
    
    // Font Defaults
    pub label_font: Option<i16>,
    pub caption_font: Option<i16>,
    pub label_size: Option<i16>,
    pub caption_size: Option<i16>,
    pub label_face: Option<i16>,
    pub caption_face: Option<i16>,
    pub label_color: Option<i16>,
    pub caption_color: Option<i16>,
    pub label_justification: Option<i8>,
    
    // OLE / External Data
    pub fix_inplace_extent: Option<Point2d>,
    pub fix_inplace_gap: Option<Point2d>,
    pub cartridge_data: Option<Vec<u8>>,
    
    // Window State
    pub window_is_zoomed: Option<bool>,
    pub window_position: Option<Point2d>,
    pub window_size: Option<Point2d>,
}

impl Document {
    /// Create a new Document with no properties set
    pub fn new(id: u32) -> Self {
        Document {
            id,
            creation_user_name: None,
            creation_date: None,
            creation_program: None,
            modification_user_name: None,
            modification_date: None,
            modification_program: None,
            name: None,
            comment: None,
            bounding_box: None,
            color_table: None,
            atom_show_query: None,
            atom_show_stereo: None,
            atom_show_atom_number: None,
            bond_show_query: None,
            bond_show_stereo: None,
            bond_show_rxn: None,
            label_line_height: None,
            caption_line_height: None,
            interpret_chemically: None,
            mac_print_info: None,
            win_print_info: None,
            print_margins: None,
            chain_angle: None,
            bond_spacing: None,
            bond_length: None,
            bold_width: None,
            line_width: None,
            margin_width: None,
            hash_spacing: None,
            caption_justification: None,
            fractional_widths: None,
            magnification: None,
            label_font: None,
            caption_font: None,
            label_size: None,
            caption_size: None,
            label_face: None,
            caption_face: None,
            label_color: None,
            caption_color: None,
            label_justification: None,
            fix_inplace_extent: None,
            fix_inplace_gap: None,
            cartridge_data: None,
            window_is_zoomed: None,
            window_position: None,
            window_size: None,
        }
    }
}