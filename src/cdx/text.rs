
use serde::{Deserialize, Serialize};
use crate::cdx::values::{Point2d, CDXString};

/// Text object - An arbitrary block of (possibly styled) text.
/// Text objects can be used in various contexts and may or may not have chemical meaning.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextObject {
    pub id: u32,
    
    // Required properties
    pub text: CDXString, // CDXPROP_TEXT (0x0700)
    
    // Optional common properties
    pub z_order: Option<i16>,
    pub ignore_warnings: Option<bool>,
    pub chemical_warning: Option<CDXString>,
    pub visible: Option<bool>,
    
    // Position and geometry
    pub position_2d: Option<Point2d>,
    pub bounding_box: Option<(f64, f64, f64, f64)>, // (left, top, right, bottom)
    pub rotation_angle: Option<i32>,
    
    // Text formatting
    pub justification: Option<i8>,
    pub line_height: Option<u16>,
    pub word_wrap_width: Option<i16>,
    pub line_starts: Option<Vec<i16>>,
    pub label_alignment: Option<i8>,
    pub label_line_height: Option<i16>,
    pub caption_line_height: Option<i16>,
    pub interpret_chemically: Option<bool>,
    
    // Font/style properties
    pub label_font: Option<i16>,
    pub caption_font: Option<i16>,
    pub label_size: Option<i16>,
    pub caption_size: Option<i16>,
    pub label_face: Option<i16>,
    pub caption_face: Option<i16>,
    pub label_color: Option<i16>,
    pub caption_color: Option<i16>,
    pub caption_justification: Option<i8>,
    pub label_justification: Option<i8>,
}

impl TextObject {
    /// Create a new Text object with required properties
    pub fn new(id: u32, text: CDXString) -> Self {
        TextObject {
            id,
            text,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            position_2d: None,
            bounding_box: None,
            rotation_angle: None,
            justification: None,
            line_height: None,
            word_wrap_width: None,
            line_starts: None,
            label_alignment: None,
            label_line_height: None,
            caption_line_height: None,
            interpret_chemically: None,
            label_font: None,
            caption_font: None,
            label_size: None,
            caption_size: None,
            label_face: None,
            caption_face: None,
            label_color: None,
            caption_color: None,
            caption_justification: None,
            label_justification: None,
        }
    }
}
