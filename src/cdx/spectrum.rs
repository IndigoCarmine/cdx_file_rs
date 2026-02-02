use crate::cdx::values::{CDXString, Rectangle};
use serde::{Deserialize, Serialize};

/// Spectrum (スペクトラム) Object
/// Represents NMR, MS, IR or other spectral plot data
/// CDX ID: 0x8010
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spectrum {
    pub id: u32,

    // Common properties
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// Suppress chemical warnings (Optional)
    pub ignore_warnings: Option<bool>,
    /// Chemical warning text (Optional)
    pub chemical_warning: Option<CDXString>,
    /// Visibility flag (Optional)
    pub visible: Option<bool>,

    // Geometry
    /// REQUIRED: Bounding rectangle for the spectrum display
    pub bounding_box: Option<Rectangle>,

    // Color
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,

    // Styling
    /// Bold width (Optional)
    pub bold_width: Option<f64>,
    /// Line width (Optional)
    pub line_width: Option<f64>,
    /// Label font family (Optional)
    pub label_style_font: Option<i16>,
    /// Label font size (Optional)
    pub label_style_size: Option<i16>,
    /// Label font face (Optional)
    pub label_style_face: Option<i16>,

    // Spectrum-specific properties - REQUIRED
    /// REQUIRED: X-axis spacing (ppm, Hz, etc.)
    pub spectrum_x_spacing: Option<f64>,
    /// REQUIRED: First X-axis data point
    pub spectrum_x_low: Option<f64>,
    /// X-axis unit type (enumerated) (Optional)
    pub spectrum_x_type: Option<i8>,
    /// Y-axis unit type (enumerated) (Optional)
    pub spectrum_y_type: Option<i8>,
    /// X-axis label text (Optional)
    pub spectrum_x_axis_label: Option<CDXString>,
    /// Y-axis label text (Optional)
    pub spectrum_y_axis_label: Option<CDXString>,
    /// REQUIRED: Y-axis values array
    pub spectrum_data_point: Option<Vec<f64>>,
    /// Spectrum type (NMR, IR, etc.) (enumerated) (Optional)
    pub spectrum_class: Option<i8>,
    /// Y offset for XML storage (Optional)
    pub spectrum_y_low: Option<f64>,
    /// Y scaling for XML storage (Optional)
    pub spectrum_y_scale: Option<f64>,
}

impl Spectrum {
    /// Create a new Spectrum with just an ID
    pub fn new(id: u32) -> Self {
        Spectrum {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
            bold_width: None,
            line_width: None,
            label_style_font: None,
            label_style_size: None,
            label_style_face: None,
            spectrum_x_spacing: None,
            spectrum_x_low: None,
            spectrum_x_type: None,
            spectrum_y_type: None,
            spectrum_x_axis_label: None,
            spectrum_y_axis_label: None,
            spectrum_data_point: None,
            spectrum_class: None,
            spectrum_y_low: None,
            spectrum_y_scale: None,
        }
    }
}
