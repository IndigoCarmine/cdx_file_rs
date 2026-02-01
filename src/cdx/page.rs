use crate::cdx::values::{CDXString, Rectangle};
use serde::{Deserialize, Serialize};

/// Page Object: A drawing space that divides objects into separate drawing areas
/// A Page is used to divide objects into separate drawing spaces. If there is only one
/// drawing space, all the graphical items should be placed in one page object. In most
/// documents, a Page object will correspond to a physical piece of paper when printed.
/// A Page has no required objects or properties.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub id: u32,

    // Geometry and visibility
    /// The smallest rectangle that encloses the graphical representation (Optional)
    pub bounding_box: Option<Rectangle>,
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Suppress chemical warnings (Optional)
    pub ignore_warnings: Option<bool>,
    /// Chemical warning text (Optional)
    pub chemical_warning: Option<CDXString>,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,

    // Color properties
    /// The foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// The background color index (Optional)
    pub background_color: Option<i16>,

    // Page dimensions
    /// The width of the document in pages (Optional)
    pub width_pages: Option<i16>,
    /// The height of the document in pages (Optional)
    pub height_pages: Option<i16>,
    /// The type of drawing space used for this document (Optional)
    pub drawing_space_type: Option<i8>,
    /// The width of an object in CDX coordinate units (Optional)
    pub width: Option<f64>,
    /// The height of an object in CDX coordinate units (Optional)
    pub height: Option<f64>,
    /// The amount of overlap of pages when a poster is tiled (Optional)
    pub page_overlap: Option<f64>,

    // Header and footer
    /// The text of the header (Optional)
    pub header: Option<CDXString>,
    /// The vertical offset of the header baseline from the top (Optional)
    pub header_position: Option<f64>,
    /// The text of the footer (Optional)
    pub footer: Option<CDXString>,
    /// The vertical offset of the footer baseline from the bottom (Optional)
    pub footer_position: Option<f64>,

    // Printing properties
    /// If present, trim marks are printed in the margins (Optional)
    pub print_trim_marks: Option<bool>,
    /// An array of vertical positions that subdivide a page into regions (Optional)
    pub splitter_positions: Option<Vec<u32>>,
    /// A description of the type of formatting used by the page (Optional)
    pub page_definition: Option<i8>,
    /// The rectangle containing a page in the coordinate space of the containing page (Optional)
    pub bounds_in_parent: Option<Rectangle>,
}

impl Page {
    /// Create a new Page with no required properties
    pub fn new(id: u32) -> Self {
        Page {
            id,
            bounding_box: None,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            foreground_color: None,
            background_color: None,
            width_pages: None,
            height_pages: None,
            drawing_space_type: None,
            width: None,
            height: None,
            page_overlap: None,
            header: None,
            header_position: None,
            footer: None,
            footer_position: None,
            print_trim_marks: None,
            splitter_positions: None,
            page_definition: None,
            bounds_in_parent: None,
        }
    }
}
