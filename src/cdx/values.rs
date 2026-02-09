use crate::cdx::text_styles::{FontEntry, StyledString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CdxValue {
    Raw(Vec<u8>),
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Float64(f64),
    Boolean(bool),
    BooleanImplied(bool),
    String(StyledString),
    Point2d(Point2d),
    Point3d(Point3d),
    Rectangle(Rectangle),
    Color { r: u16, g: u16, b: u16 },
    ColorList(Vec<(u16, u16, u16)>),
    FontList { os_type: u16, fonts: Vec<FontEntry> },
    ObjectIDArray(Vec<u32>),
    Int16List(Vec<i16>),
    Date(u32),
    Coordinate(f64),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Rectangle {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl Rectangle {
    pub fn zero() -> Self {
        Self {
            top: 0.0,
            left: 0.0,
            bottom: 0.0,
            right: 0.0,
        }
    }
    pub fn from_point(p: &Point2d) -> Self {
        Self {
            top: p.y,
            left: p.x,
            bottom: p.y,
            right: p.x,
        }
    }

    pub fn from_points(p1: &Point2d, p2: &Point2d) -> Self {
        Self {
            top: p1.y.min(p2.y),
            left: p1.x.min(p2.x),
            bottom: p1.y.max(p2.y),
            right: p1.x.max(p2.x),
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            top: self.top.min(other.top),
            left: self.left.min(other.left),
            bottom: self.bottom.max(other.bottom),
            right: self.right.max(other.right),
        }
    }
}

/// Font style run for CDXString
/// An 8-byte struct describing font styling for a range of text
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CDXStyleRun {
    /// Character index where this style starts
    pub char_index: u16,
    /// Zero-based index to font table
    pub font_index: u16,
    /// Font face/style (0x00=plain, 0x01=bold, 0x02=italic, 0x04=underline, etc.)
    pub font_face: u16,
    /// Font size in 20ths of a point
    pub font_size: u16,
    /// Font color index
    pub color_index: u16,
}

/// CDX String data type
/// A variable-length struct consisting of style runs followed by text
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CDXString {
    /// Style runs for the text (empty if plain text)
    pub style_runs: Vec<CDXStyleRun>,
    /// The actual text content (ISO Latin-1 encoding)
    pub text: String,
}



// Boolean with implied false (just for encode/decode convenience)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BooleanImplied(pub bool);