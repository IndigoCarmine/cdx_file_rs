use serde::{Deserialize, Serialize};
use crate::cdx::values::*;

/// Graphic (図形) object
/// CDX ID: 0x8007
/// Represents a graphic element like line, arc, circle, or rectangle
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graphic {
    pub id: u32,
    
    // Core properties
    pub z_order: Option<i16>,                  // 0x000A
    pub ignore_warnings: Option<bool>,         // 0x000F
    pub chemical_warning: Option<String>,      // 0x0010
    pub visible: Option<bool>,                 // 0x0011
    pub superseded_by: Option<u32>,            // 0x0012
    
    // Positioning
    pub bounding_box: Option<Rectangle>,       // 0x0204 - Required: smallest rectangle enclosing the graphic
    pub head_3d: Option<Point3d>,              // 0x0207
    pub tail_3d: Option<Point3d>,              // 0x0208
    
    // Color
    pub foreground_color: Option<u16>,         // 0x0301
    pub background_color: Option<i16>,         // 0x0302
    
    // Line/Width
    pub bold_width: Option<f64>,               // 0x0806
    pub line_width: Option<f64>,               // 0x0807
    
    // Caption style
    pub caption_style: Option<u16>,            // 0x080B (not used)
    pub caption_style_font: Option<i16>,       // 0x081B
    pub caption_style_size: Option<i16>,       // 0x081D
    pub caption_style_face: Option<i16>,       // 0x081F
    
    // Graphic type properties
    pub graphic_type: Option<i16>,             // 0x0A00
    pub line_type: Option<i16>,                // 0x0A01
    pub arrow_type: Option<i16>,               // 0x0A02
    pub rectangle_type: Option<i16>,           // 0x0A03
    pub oval_type: Option<i16>,                // 0x0A04
    pub orbital_type: Option<i16>,             // 0x0A05
    pub bracket_type: Option<i16>,             // 0x0A06
    pub symbol_type: Option<i16>,              // 0x0A07
    
    // Graphic-specific properties
    pub arrowhead_size: Option<i16>,           // 0x0A20
    pub arc_angular_size: Option<i16>,         // 0x0A21
    pub bracket_lip_size: Option<i16>,         // 0x0A22
    pub bracket_usage: Option<i8>,             // 0x0A24
    pub polymer_repeat_pattern: Option<i8>,    // 0x0A25
    pub polymer_flip_type: Option<i8>,         // 0x0A26
    pub corner_radius: Option<i16>,            // 0x0A3C
    pub frame_type: Option<i16>,               // 0x0A3D
}

impl Graphic {
    pub fn new(id: u32) -> Self {
        Graphic {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            superseded_by: None,
            bounding_box: None,
            head_3d: None,
            tail_3d: None,
            foreground_color: None,
            background_color: None,
            bold_width: None,
            line_width: None,
            caption_style: None,
            caption_style_font: None,
            caption_style_size: None,
            caption_style_face: None,
            graphic_type: None,
            line_type: None,
            arrow_type: None,
            rectangle_type: None,
            oval_type: None,
            orbital_type: None,
            bracket_type: None,
            symbol_type: None,
            arrowhead_size: None,
            arc_angular_size: None,
            bracket_lip_size: None,
            bracket_usage: None,
            polymer_repeat_pattern: None,
            polymer_flip_type: None,
            corner_radius: None,
            frame_type: None,
        }
    }
}
