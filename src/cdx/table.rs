use crate::cdx::values::Rectangle;
use serde::{Deserialize, Serialize};

/// Table object (0x8016)
/// A grid-like arrangement of drawing spaces where each cell is a Page object.
/// Cells are stored row-wise from the top-left.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub id: u32,

    // Core properties
    pub z_order: Option<i16>,          // 0x000A - Back-to-front ordering
    pub visible: Option<bool>,         // 0x0011 - Visibility flag
    pub bounding_box: Option<Rectangle>, // 0x0204 - Smallest enclosing rectangle

    // Color properties
    pub foreground_color: Option<u16>, // 0x0301 - Foreground color index
    pub background_color: Option<i16>, // 0x0302 - Background color index

    // Line/styling properties
    pub bold_width: Option<f64>,   // 0x0806 - Default bold bond width
    pub line_width: Option<f64>,   // 0x0807 - Default line width
    pub margin_width: Option<f64>, // 0x0808 - Default margin around labels

    // Label font properties
    pub label_style_font: Option<i16>, // 0x081A - Default label font family
    pub label_style_size: Option<i16>, // 0x081C - Default label font size
    pub label_style_face: Option<i16>, // 0x081E - Default label font style
}

impl Table {
    pub fn new(id: u32) -> Self {
        Table {
            id,
            z_order: None,
            visible: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
            bold_width: None,
            line_width: None,
            margin_width: None,
            label_style_font: None,
            label_style_size: None,
            label_style_face: None,
        }
    }
}
