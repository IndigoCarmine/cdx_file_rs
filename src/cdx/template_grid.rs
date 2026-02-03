use crate::cdx::values::Point2d;
use serde::{Deserialize, Serialize};

/// Template Grid (テンプレートグリッド) Object
/// Grid layout for template documents
/// CDX ID: 0x800B
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateGrid {
    pub id: u32,

    // Required properties
    /// REQUIRED: Width and height (2D extent)
    pub extent_2d: Option<Point2d>,
    /// REQUIRED: Viewing window height
    pub template_pane_height: Option<f64>,
    /// REQUIRED: Number of rows
    pub template_num_rows: Option<i16>,
    /// REQUIRED: Number of columns
    pub template_num_columns: Option<i16>,
}

impl TemplateGrid {
    /// Create a new TemplateGrid with just an ID
    pub fn new(id: u32) -> Self {
        TemplateGrid {
            id,
            extent_2d: None,
            template_pane_height: None,
            template_num_rows: None,
            template_num_columns: None,
        }
    }
}
