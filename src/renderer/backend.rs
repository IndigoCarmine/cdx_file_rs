/// Backend abstraction layer for rendering
/// 
/// This module defines backend-agnostic types and traits for rendering,
/// allowing multiple rendering backends (egui, SVG, PDF, etc.) to be supported.

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

/// Specific font families for rich text rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RichTextFontFamily {
    /// Arial (sans-serif)
    #[default]
    Arial,
    /// Times New Roman (serif)
    TimesNewRoman,
    /// Symbol font (for Greek letters and math symbols)
    Symbol,
}

impl RichTextFontFamily {
    /// Get the CSS/SVG font-family string
    pub fn to_css_family(&self) -> &'static str {
        match self {
            RichTextFontFamily::Arial => "Arial, sans-serif",
            RichTextFontFamily::TimesNewRoman => "Times New Roman, serif",
            RichTextFontFamily::Symbol => "Symbol, serif",
        }
    }
    
    /// Get the font name for system font lookup
    pub fn font_name(&self) -> &'static str {
        match self {
            RichTextFontFamily::Arial => "Arial",
            RichTextFontFamily::TimesNewRoman => "Times New Roman",
            RichTextFontFamily::Symbol => "Symbol",
        }
    }
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

/// Text style flags for rich text rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TextStyle {
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underline text
    pub underline: bool,
    /// Subscript (lowered position, smaller size)
    pub subscript: bool,
    /// Superscript (raised position, smaller size)
    pub superscript: bool,
}

impl TextStyle {
    pub const PLAIN: TextStyle = TextStyle {
        bold: false,
        italic: false,
        underline: false,
        subscript: false,
        superscript: false,
    };
    
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    
    pub fn subscript(mut self) -> Self {
        self.subscript = true;
        self
    }
    
    pub fn superscript(mut self) -> Self {
        self.superscript = true;
        self
    }
}

/// A span of text with consistent styling for rich text rendering
#[derive(Debug, Clone)]
pub struct TextSpan {
    /// The text content
    pub text: String,
    /// Font size (in points)
    pub font_size: f32,
    /// Text color
    pub color: Color,
    /// Text style flags (bold, italic, etc.)
    pub style: TextStyle,
    /// Font family (Arial, Times New Roman, Symbol)
    pub font_family: RichTextFontFamily,
}

impl TextSpan {
    pub fn new(text: impl Into<String>, font_size: f32, color: Color) -> Self {
        TextSpan {
            text: text.into(),
            font_size,
            color,
            style: TextStyle::default(),
            font_family: RichTextFontFamily::default(),
        }
    }
    
    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }
    
    pub fn with_font_family(mut self, font_family: RichTextFontFamily) -> Self {
        self.font_family = font_family;
        self
    }
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
    
    /// Draw rich text (styled text spans) at specified position
    /// 
    /// This method renders a sequence of styled text spans horizontally.
    /// Each span can have different font size, color, and style (bold, italic, etc.).
    /// Subscript and superscript are handled via Y-offset (approximately 30% of font size).
    fn rich_text(&self, pos: Point2d, align: Align2, spans: &[TextSpan]);
}
