/// Backend abstraction layer for rendering
/// 
/// This module defines backend-agnostic types and traits for rendering,
/// allowing multiple rendering backends (egui, SVG, PDF, etc.) to be supported.

use std::fmt;

/// Backend-agnostic 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Point2d {
    pub fn new(x: f32, y: f32) -> Self {
        Point2d { x, y }
    }
}

/// Backend-agnostic color representation (RGBA)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };
    
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
    
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

/// Backend-agnostic stroke style
#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub width: f32,
    pub color: Color,
}

impl Stroke {
    pub const NONE: Stroke = Stroke {
        width: 0.0,
        color: Color::BLACK,
    };
    
    pub fn new(width: f32, color: Color) -> Self {
        Stroke { width, color }
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub const LEFT: Align = Align::Left;
    pub const CENTER: Align = Align::Center;
    pub const RIGHT: Align = Align::Right;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Align2 {
    pub x: Align,
    pub y: VerticalAlign,
}

impl Align2 {
    pub const LEFT_TOP: Align2 = Align2 { x: Align::Left, y: VerticalAlign::Top };
    pub const LEFT_CENTER: Align2 = Align2 { x: Align::Left, y: VerticalAlign::Center };
    pub const LEFT_BOTTOM: Align2 = Align2 { x: Align::Left, y: VerticalAlign::Bottom };
    pub const CENTER_TOP: Align2 = Align2 { x: Align::Center, y: VerticalAlign::Top };
    pub const CENTER_CENTER: Align2 = Align2 { x: Align::Center, y: VerticalAlign::Center };
    pub const CENTER_BOTTOM: Align2 = Align2 { x: Align::Center, y: VerticalAlign::Bottom };
    pub const RIGHT_TOP: Align2 = Align2 { x: Align::Right, y: VerticalAlign::Top };
    pub const RIGHT_CENTER: Align2 = Align2 { x: Align::Right, y: VerticalAlign::Center };
    pub const RIGHT_BOTTOM: Align2 = Align2 { x: Align::Right, y: VerticalAlign::Bottom };
    
    pub fn x(&self) -> Align {
        self.x
    }
}

/// Rectangle defined by two points
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Point2d,
    pub max: Point2d,
}

impl Rect {
    pub fn from_two_pos(p1: Point2d, p2: Point2d) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        Rect {
            min: Point2d::new(min_x, min_y),
            max: Point2d::new(max_x, max_y),
        }
    }
    
    pub fn from_min_max(min: Point2d, max: Point2d) -> Self {
        Rect { min, max }
    }
    
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
    
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

/// Font family enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Proportional,
    Monospace,
}

/// Font identifier
#[derive(Debug, Clone, Copy)]
pub struct FontId {
    pub size: f32,
    pub family: FontFamily,
}

impl FontId {
    pub fn new(size: f32, family: FontFamily) -> Self {
        FontId { size, family }
    }
}

/// Text layout result
#[derive(Debug, Clone)]
pub struct Galley {
    pub size: (f32, f32),  // (width, height)
    pub text: String,
}

/// Abstract painter trait for backend-agnostic rendering
/// 
/// This trait defines the minimal interface required for rendering CDX objects.
/// Different backends (egui, SVG, PDF) implement this trait.
pub trait AbstractPainter {
    /// Draw a line segment between two points
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke);
    
    /// Draw a filled circle
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color);
    
    /// Draw a circle outline
    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke);
    
    /// Draw a filled rectangle with rounded corners
    fn rect_filled(&self, rect: Rect, rounding: f32, color: Color);
    
    /// Draw a rectangle outline with rounded corners
    fn rect_stroke(&self, rect: Rect, rounding: f32, stroke: Stroke);
    
    /// Draw both filled and stroked rectangle
    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke);
    
    /// Draw text at specified position with alignment
    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color);
    
    /// Draw a polyline (connected line segments)
    fn polyline(&self, points: &[Point2d], stroke: Stroke);
    
    /// Draw a closed polygon (polyline with last point connecting to first)
    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke);
    
    /// Draw a filled convex polygon
    fn convex_polygon(&self, points: &[Point2d], fill: Color);
    
    /// Layout text without drawing (for width calculation)
    fn layout_no_wrap(&self, text: String, font: FontId, color: Color) -> Galley;
    
    /// Get the clip rectangle
    fn clip_rect(&self) -> Rect;
}
