/// SVG backend implementation for AbstractPainter
/// 
/// This module provides an adapter that implements AbstractPainter by generating SVG elements.

use super::backend::*;
use std::cell::RefCell;

/// Convert backend Color to SVG color string
fn color_to_svg(c: Color) -> String {
    if c.a == 255 {
        format!("rgb({},{},{})", c.r, c.g, c.b)
    } else {
        format!("rgba({},{},{},{})", c.r, c.g, c.b, c.a as f32 / 255.0)
    }
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// SVG backend wrapper implementing AbstractPainter
pub struct SvgBackend {
    elements: RefCell<Vec<String>>,
    clip_rect: Rect,
}

impl SvgBackend {
    pub fn new(clip_rect: Rect) -> Self {
        SvgBackend {
            elements: RefCell::new(Vec::new()),
            clip_rect,
        }
    }

    /// Get the accumulated SVG elements
    pub fn get_elements(&self) -> Vec<String> {
        self.elements.borrow().clone()
    }

    /// Generate complete SVG document
    pub fn to_svg_document(&self, width: f32, height: f32) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="{} {} {} {}">"#,
            width,
            height,
            self.clip_rect.min.x,
            self.clip_rect.min.y,
            self.clip_rect.width(),
            self.clip_rect.height()
        ));
        svg.push('\n');

        for element in self.elements.borrow().iter() {
            svg.push_str("  ");
            svg.push_str(element);
            svg.push('\n');
        }

        svg.push_str("</svg>");
        svg
    }
}

impl AbstractPainter for SvgBackend {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        let element = format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-linecap="round" />"#,
            start.x, start.y, end.x, end.y, color_to_svg(stroke.color), stroke.width
        );
        self.elements.borrow_mut().push(element);
    }

    fn circle_filled(&self, center: Point2d, radius: f32, color: Color) {
        let element = format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="{}" />"#,
            center.x, center.y, radius, color_to_svg(color)
        );
        self.elements.borrow_mut().push(element);
    }

    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke) {
        let element = format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="{}" stroke-width="{}" />"#,
            center.x, center.y, radius, color_to_svg(stroke.color), stroke.width
        );
        self.elements.borrow_mut().push(element);
    }

    fn rect_filled(&self, rect: Rect, rounding: f32, color: Color) {
        let element = if rounding > 0.0 {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}" fill="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), rounding, rounding, color_to_svg(color)
            )
        } else {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), color_to_svg(color)
            )
        };
        self.elements.borrow_mut().push(element);
    }

    fn rect_stroke(&self, rect: Rect, rounding: f32, stroke: Stroke) {
        let element = if rounding > 0.0 {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}" fill="none" stroke="{}" stroke-width="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), rounding, rounding, color_to_svg(stroke.color), stroke.width
            )
        } else {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="{}" stroke-width="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), color_to_svg(stroke.color), stroke.width
            )
        };
        self.elements.borrow_mut().push(element);
    }

    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke) {
        let element = if rounding > 0.0 {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" rx="{}" ry="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), rounding, rounding, color_to_svg(fill), color_to_svg(stroke.color), stroke.width
            )
        } else {
            format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}" />"#,
                rect.min.x, rect.min.y, rect.width(), rect.height(), color_to_svg(fill), color_to_svg(stroke.color), stroke.width
            )
        };
        self.elements.borrow_mut().push(element);
    }

    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color) {
        let text_anchor = match align.x {
            Align::Left => "start",
            Align::Center => "middle",
            Align::Right => "end",
        };

        let dominant_baseline = match align.y {
            VerticalAlign::Top => "hanging",
            VerticalAlign::Center => "middle",
            VerticalAlign::Bottom => "baseline",
        };

        let font_family = match font.family {
            FontFamily::Proportional => "Arial, sans-serif",
            FontFamily::Monospace => "monospace",
        };

        let element = format!(
            r#"<text x="{}" y="{}" fill="{}" font-family="{}" font-size="{}" text-anchor="{}" dominant-baseline="{}">{}</text>"#,
            pos.x, pos.y, color_to_svg(color), font_family, font.size, text_anchor, dominant_baseline, escape_xml(text)
        );
        self.elements.borrow_mut().push(element);
    }

    fn polyline(&self, points: &[Point2d], stroke: Stroke) {
        if points.is_empty() {
            return;
        }

        let points_str = points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ");

        let element = format!(
            r#"<polyline points="{}" fill="none" stroke="{}" stroke-width="{}" stroke-linecap="round" stroke-linejoin="round" />"#,
            points_str, color_to_svg(stroke.color), stroke.width
        );
        self.elements.borrow_mut().push(element);
    }

    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke) {
        if points.is_empty() {
            return;
        }

        let points_str = points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ");

        let element = format!(
            r#"<polygon points="{}" fill="none" stroke="{}" stroke-width="{}" stroke-linecap="round" stroke-linejoin="round" />"#,
            points_str, color_to_svg(stroke.color), stroke.width
        );
        self.elements.borrow_mut().push(element);
    }

    fn convex_polygon(&self, points: &[Point2d], fill: Color) {
        if points.is_empty() {
            return;
        }

        let points_str = points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<_>>()
            .join(" ");

        let element = format!(
            r#"<polygon points="{}" fill="{}" />"#,
            points_str, color_to_svg(fill)
        );
        self.elements.borrow_mut().push(element);
    }

    fn layout_no_wrap(&self, text: String, font: FontId, _color: Color) -> Galley {
        // Simple text width estimation for SVG
        // This is a rough approximation; actual width depends on font metrics
        let char_width = font.size * 0.6; // Average character width
        let width = text.len() as f32 * char_width;
        let height = font.size * 1.2; // Line height

        Galley {
            size: (width, height),
            text,
        }
    }

    fn clip_rect(&self) -> Rect {
        self.clip_rect
    }
}
