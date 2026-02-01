use crate::cdx::values::{CDXString, Point2d, Rectangle};
use serde::{Deserialize, Serialize};

/// Text Object: An arbitrary block of (possibly styled) text
/// Text objects can be used in various contexts and may or may not have chemical meaning.
/// Required: text property (CDXPROP_TEXT, 0x0700)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextObject {
    pub id: u32,

    // Required properties
    /// The text content (CDXPROP_TEXT, 0x0700) - REQUIRED
    pub text: CDXString,

    // Optional common properties
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Suppress chemical warnings (Optional)
    pub ignore_warnings: Option<bool>,
    /// Chemical warning text (Optional)
    pub chemical_warning: Option<CDXString>,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,

    // Position and geometry
    /// The 2D location of the text (Optional)
    pub position_2d: Option<Point2d>,
    /// The smallest rectangle that encloses the text (Optional)
    pub bounding_box: Option<Rectangle>,
    /// Angular orientation in degrees * 65536 (Optional)
    pub rotation_angle: Option<i32>,

    // Text formatting
    /// Horizontal justification (Optional)
    pub justification: Option<i8>,
    /// Line height in twips (Optional)
    pub line_height: Option<u16>,
    /// Word wrap width (Optional)
    pub word_wrap_width: Option<i16>,
    /// Array of line start positions (Optional)
    pub line_starts: Option<Vec<i16>>,
    /// Label alignment (Optional)
    pub label_alignment: Option<i8>,
    /// Label line height (Optional)
    pub label_line_height: Option<i16>,
    /// Caption line height (Optional)
    pub caption_line_height: Option<i16>,
    /// Interpret text chemically (Optional)
    pub interpret_chemically: Option<bool>,

    // Font/style properties
    /// Label font index (Optional)
    pub label_font: Option<i16>,
    /// Caption font index (Optional)
    pub caption_font: Option<i16>,
    /// Label font size in points (Optional)
    pub label_size: Option<i16>,
    /// Caption font size in points (Optional)
    pub caption_size: Option<i16>,
    /// Label font face (Optional)
    pub label_face: Option<i16>,
    /// Caption font face (Optional)
    pub caption_face: Option<i16>,
    /// Label color index (Optional)
    pub label_color: Option<i16>,
    /// Caption color index (Optional)
    pub caption_color: Option<i16>,
    /// Caption justification (Optional)
    pub caption_justification: Option<i8>,
    /// Label justification (Optional)
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
