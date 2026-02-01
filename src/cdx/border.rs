use serde::{Deserialize, Serialize};

/// Border (枠線) object
/// Represents border/edge information for an object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Border {
    pub id: u32,

    // Properties from specification
    pub foreground_color: Option<u16>, // kCDXProp_ForegroundColor (0x0301)
    pub line_width: Option<f64>,       // kCDXProp_LineWidth (0x0807)
    pub side: Option<u16>,             // kCDXProp_Side (0x0825) - Required
    pub line_type: Option<i16>,        // kCDXProp_Line_Type (0x0A01)
}

impl Border {
    pub fn new(id: u32) -> Self {
        Border {
            id,
            foreground_color: None,
            line_width: None,
            side: None,
            line_type: None,
        }
    }
}
