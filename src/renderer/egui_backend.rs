/// egui backend implementation for AbstractPainter
/// 
/// This module provides an adapter that implements AbstractPainter using egui primitives.

use super::backend::*;
use eframe::egui;

/// Convert backend Color to egui::Color32
impl From<Color> for egui::Color32 {
    fn from(c: Color) -> Self {
        egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
    }
}

/// Convert egui::Color32 to backend Color
impl From<egui::Color32> for Color {
    fn from(c: egui::Color32) -> Self {
        Color::from_rgba(c.r(), c.g(), c.b(), c.a())
    }
}

/// Convert backend Point2d to egui::Pos2
impl From<Point2d> for egui::Pos2 {
    fn from(p: Point2d) -> Self {
        egui::Pos2::new(p.x, p.y)
    }
}

/// Convert egui::Pos2 to backend Point2d
impl From<egui::Pos2> for Point2d {
    fn from(p: egui::Pos2) -> Self {
        Point2d::new(p.x, p.y)
    }
}

/// Convert backend Stroke to egui::Stroke
impl From<Stroke> for egui::Stroke {
    fn from(s: Stroke) -> Self {
        egui::Stroke::new(s.width, egui::Color32::from(s.color))
    }
}

/// Convert backend Rect to egui::Rect
impl From<Rect> for egui::Rect {
    fn from(r: Rect) -> Self {
        egui::Rect::from_two_pos(r.min.into(), r.max.into())
    }
}

/// Convert egui::Rect to backend Rect
impl From<egui::Rect> for Rect {
    fn from(r: egui::Rect) -> Self {
        Rect::from_two_pos(r.min.into(), r.max.into())
    }
}

/// Convert backend Align to egui::Align
impl From<Align> for egui::Align {
    fn from(a: Align) -> Self {
        match a {
            Align::Left => egui::Align::LEFT,
            Align::Center => egui::Align::Center,
            Align::Right => egui::Align::RIGHT,
        }
    }
}

/// Convert backend VerticalAlign to egui::Align
impl From<VerticalAlign> for egui::Align {
    fn from(a: VerticalAlign) -> Self {
        match a {
            VerticalAlign::Top => egui::Align::TOP,
            VerticalAlign::Center => egui::Align::Center,
            VerticalAlign::Bottom => egui::Align::BOTTOM,
        }
    }
}

/// Convert backend Align2 to egui::Align2
impl From<Align2> for egui::Align2 {
    fn from(a: Align2) -> Self {
        egui::Align2([a.x.into(), a.y.into()])
    }
}

/// Convert backend FontFamily to egui::FontFamily
impl From<FontFamily> for egui::FontFamily {
    fn from(f: FontFamily) -> Self {
        match f {
            FontFamily::Proportional => egui::FontFamily::Proportional,
            FontFamily::Monospace => egui::FontFamily::Monospace,
        }
    }
}

/// Convert backend FontId to egui::FontId
impl From<FontId> for egui::FontId {
    fn from(f: FontId) -> Self {
        egui::FontId::new(f.size, f.family.into())
    }
}

/// egui backend wrapper implementing AbstractPainter
pub struct EguiBackend<'a> {
    painter: &'a egui::Painter,
}

impl<'a> EguiBackend<'a> {
    pub fn new(painter: &'a egui::Painter) -> Self {
        EguiBackend { painter }
    }
}

impl<'a> AbstractPainter for EguiBackend<'a> {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        let egui_points: [egui::Pos2; 2] = [start.into(), end.into()];
        self.painter.line_segment(egui_points, egui::Stroke::from(stroke));
    }
    
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color) {
        self.painter.circle_filled(center.into(), radius, egui::Color32::from(color));
    }
    
    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke) {
        self.painter.circle_stroke(center.into(), radius, egui::Stroke::from(stroke));
    }
    
    fn rect_filled(&self, rect: Rect, rounding: f32, color: Color) {
        self.painter.rect_filled(rect.into(), rounding, egui::Color32::from(color));
    }
    
    fn rect_stroke(&self, rect: Rect, rounding: f32, stroke: Stroke) {
        self.painter.rect_stroke(rect.into(), rounding, egui::Stroke::from(stroke));
    }
    
    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke) {
        self.painter.rect(rect.into(), rounding, egui::Color32::from(fill), egui::Stroke::from(stroke));
    }
    
    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color) {
        self.painter.text(pos.into(), align.into(), text, font.into(), egui::Color32::from(color));
    }
    
    fn polyline(&self, points: &[Point2d], stroke: Stroke) {
        let egui_points: Vec<egui::Pos2> = points.iter().map(|&p| p.into()).collect();
        self.painter.add(egui::Shape::line(egui_points, egui::Stroke::from(stroke)));
    }
    
    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke) {
        let mut egui_points: Vec<egui::Pos2> = points.iter().map(|&p| p.into()).collect();
        if !egui_points.is_empty() {
            egui_points.push(egui_points[0]);
        }
        self.painter.add(egui::Shape::line(egui_points, egui::Stroke::from(stroke)));
    }
    
    fn convex_polygon(&self, points: &[Point2d], fill: Color) {
        let egui_points: Vec<egui::Pos2> = points.iter().map(|&p| p.into()).collect();
        self.painter.add(egui::Shape::convex_polygon(
            egui_points,
            egui::Color32::from(fill),
            egui::Stroke::NONE,
        ));
    }
    
    fn layout_no_wrap(&self, text: String, font: FontId, color: Color) -> Galley {
        let galley = self.painter.layout_no_wrap(text.clone(), font.into(), egui::Color32::from(color));
        Galley {
            size: (galley.size().x, galley.size().y),
            text,
        }
    }
    
    fn clip_rect(&self) -> Rect {
        self.painter.clip_rect().into()
    }
}
