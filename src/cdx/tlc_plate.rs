use serde::{Deserialize, Serialize};

/// TLC Plate Object: A rectangular object representing a Thin Layer Chromatography (TLC) plate
/// Each plate contains a series of Lanes. Those lanes should be arranged on the plate from
/// left to right in the order that they appear in the cdx file. TLC Plates should not be
/// assumed to be positioned vertically. The actual orientation can be determined from the
/// four corner properties (TopLeft, TopRight, BottomRight, BottomLeft). Similarly, they
/// should not be assumed to be orthogonal, although in most practical cases there will be
/// 90° angles at each corner.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TLCPlate {
    pub id: u32,

    // Optional properties
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,
    /// The object is visible if non-zero (Optional)
    pub visible: Option<bool>,
    /// The smallest rectangle that encloses the graphical representation of the object (Optional)
    pub bounding_box: Option<crate::cdx::values::Rectangle>,
    /// The location of the top-left corner of a quadrilateral object (Optional)
    pub top_left: Option<crate::cdx::values::Point2d>,
    /// The location of the top-right corner of a quadrilateral object (Optional)
    pub top_right: Option<crate::cdx::values::Point2d>,
    /// The location of the bottom-right corner of a quadrilateral object (Optional)
    pub bottom_right: Option<crate::cdx::values::Point2d>,
    /// The location of the bottom-left corner of a quadrilateral object (Optional)
    pub bottom_left: Option<crate::cdx::values::Point2d>,
    /// The foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// The background color index (Optional)
    pub background_color: Option<i16>,
    /// The default bold bond width (Optional)
    pub bold_width: Option<f64>,
    /// The default line width (Optional)
    pub line_width: Option<f64>,
    /// The default amount of space surrounding atom labels (Optional)
    pub margin_width: Option<f64>,
    /// The default font family for atom labels (Optional)
    pub label_font: Option<i16>,
    /// The default font size for atom labels (Optional)
    pub label_size: Option<i16>,
    /// The default font style for atom labels (Optional)
    pub label_face: Option<i16>,
    /// The distance of the origin line from the bottom of a TLC Plate, as a fraction of the total height (Optional)
    pub tlc_origin_fraction: Option<f64>,
    /// The distance of the solvent front from the top of a TLC Plate, as a fraction of the total height (Optional)
    pub tlc_solvent_front_fraction: Option<f64>,
    /// Show the origin line near the base of the TLC Plate if non-zero (Optional)
    pub tlc_show_origin: Option<bool>,
    /// Show the solvent front line near the top of the TLC Plate if non-zero (Optional)
    pub tlc_show_solvent_front: Option<bool>,
    /// Show borders around the edges of the TLC Plate if non-zero (Optional)
    pub tlc_show_borders: Option<bool>,
}

impl TLCPlate {
    /// Create a new TLC Plate with no required properties
    pub fn new(id: u32) -> Self {
        TLCPlate {
            id,
            z_order: None,
            visible: None,
            bounding_box: None,
            top_left: None,
            top_right: None,
            bottom_right: None,
            bottom_left: None,
            foreground_color: None,
            background_color: None,
            bold_width: None,
            line_width: None,
            margin_width: None,
            label_font: None,
            label_size: None,
            label_face: None,
            tlc_origin_fraction: None,
            tlc_solvent_front_fraction: None,
            tlc_show_origin: None,
            tlc_show_solvent_front: None,
            tlc_show_borders: None,
        }
    }
}
