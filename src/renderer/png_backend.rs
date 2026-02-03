/// PNG backend implementation for AbstractPainter using tiny-skia
/// 
/// This module provides an adapter that implements AbstractPainter using tiny-skia for rasterization.

use super::backend::*;
use std::cell::RefCell;
use tiny_skia::{
    Color as SkiaColor, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, PixmapMut, Rect as SkiaRect,
    Stroke as SkiaStroke, Transform,
};

/// Convert backend Color to tiny-skia Color
fn color_to_skia(c: Color) -> SkiaColor {
    SkiaColor::from_rgba8(c.r, c.g, c.b, c.a)
}

/// PNG backend wrapper implementing AbstractPainter
pub struct PngBackend {
    pixmap: RefCell<Pixmap>,
    clip_rect: Rect,
}

impl PngBackend {
    pub fn new(width: u32, height: u32, background: Color) -> Option<Self> {
        let mut pixmap = Pixmap::new(width, height)?;
        pixmap.fill(color_to_skia(background));

        Some(PngBackend {
            pixmap: RefCell::new(pixmap),
            clip_rect: Rect {
                min: Point2d { x: 0.0, y: 0.0 },
                max: Point2d {
                    x: width as f32,
                    y: height as f32,
                },
            },
        })
    }

    /// Save the pixmap to a PNG file
    pub fn save_png(&self, path: &std::path::Path) -> Result<(), String> {
        self.pixmap
            .borrow()
            .save_png(path)
            .map_err(|e| e.to_string())
    }

    /// Get a reference to the internal pixmap
    pub fn pixmap(&self) -> std::cell::Ref<'_, Pixmap> {
        self.pixmap.borrow()
    }
}

impl AbstractPainter for PngBackend {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        let mut pb = PathBuilder::new();
        pb.move_to(start.x, start.y);
        pb.line_to(end.x, end.y);

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(stroke.color));

            let skia_stroke = SkiaStroke {
                width: stroke.width,
                line_cap: LineCap::Round,
                ..Default::default()
            };

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
        }
    }

    fn circle_filled(&self, center: Point2d, radius: f32, color: Color) {
        let mut pb = PathBuilder::new();
        pb.push_circle(center.x, center.y, radius);

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(color));

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke) {
        let mut pb = PathBuilder::new();
        pb.push_circle(center.x, center.y, radius);

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(stroke.color));

            let skia_stroke = SkiaStroke {
                width: stroke.width,
                ..Default::default()
            };

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
        }
    }

    fn rect_filled(&self, rect: Rect, rounding: f32, color: Color) {
        let mut pb = PathBuilder::new();
        
        if rounding > 0.0 {
            // Create rounded rectangle
            let x = rect.min.x;
            let y = rect.min.y;
            let w = rect.width();
            let h = rect.height();
            let r = rounding.min(w / 2.0).min(h / 2.0);

            pb.move_to(x + r, y);
            pb.line_to(x + w - r, y);
            pb.quad_to(x + w, y, x + w, y + r);
            pb.line_to(x + w, y + h - r);
            pb.quad_to(x + w, y + h, x + w - r, y + h);
            pb.line_to(x + r, y + h);
            pb.quad_to(x, y + h, x, y + h - r);
            pb.line_to(x, y + r);
            pb.quad_to(x, y, x + r, y);
            pb.close();
        } else {
            pb.push_rect(SkiaRect::from_xywh(rect.min.x, rect.min.y, rect.width(), rect.height()).unwrap());
        }

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(color));

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn rect_stroke(&self, rect: Rect, rounding: f32, stroke: Stroke) {
        let mut pb = PathBuilder::new();
        
        if rounding > 0.0 {
            // Create rounded rectangle
            let x = rect.min.x;
            let y = rect.min.y;
            let w = rect.width();
            let h = rect.height();
            let r = rounding.min(w / 2.0).min(h / 2.0);

            pb.move_to(x + r, y);
            pb.line_to(x + w - r, y);
            pb.quad_to(x + w, y, x + w, y + r);
            pb.line_to(x + w, y + h - r);
            pb.quad_to(x + w, y + h, x + w - r, y + h);
            pb.line_to(x + r, y + h);
            pb.quad_to(x, y + h, x, y + h - r);
            pb.line_to(x, y + r);
            pb.quad_to(x, y, x + r, y);
            pb.close();
        } else {
            pb.push_rect(SkiaRect::from_xywh(rect.min.x, rect.min.y, rect.width(), rect.height()).unwrap());
        }

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(stroke.color));

            let skia_stroke = SkiaStroke {
                width: stroke.width,
                ..Default::default()
            };

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
        }
    }

    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke) {
        // Draw filled first, then stroke
        self.rect_filled(rect, rounding, fill);
        if stroke.width > 0.0 {
            self.rect_stroke(rect, rounding, stroke);
        }
    }

    fn text(&self, pos: Point2d, _align: Align2, _text: &str, _font: FontId, color: Color) {
        // Note: tiny-skia doesn't support text rendering directly
        // For now, we'll render a placeholder or use a font rendering library
        // This is a simplified implementation that could be enhanced with a font library
        
        // As a simple placeholder, we could draw a small rectangle at the text position
        // In a production implementation, you'd want to integrate with a font rasterizer
        // like fontdue, ab_glyph, or rusttype
        
        // For now, let's just mark the text position with a small dot
        let marker_radius = 2.0;
        self.circle_filled(pos, marker_radius, color);
        
        // TODO: Implement proper text rendering using a font library
        // This would involve:
        // 1. Load font from system or embedded font
        // 2. Shape the text into glyphs
        // 3. Rasterize each glyph
        // 4. Apply alignment transformations
        // 5. Draw glyphs to pixmap
    }

    fn polyline(&self, points: &[Point2d], stroke: Stroke) {
        if points.len() < 2 {
            return;
        }

        let mut pb = PathBuilder::new();
        pb.move_to(points[0].x, points[0].y);

        for point in &points[1..] {
            pb.line_to(point.x, point.y);
        }

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(stroke.color));

            let skia_stroke = SkiaStroke {
                width: stroke.width,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            };

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
        }
    }

    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke) {
        if points.len() < 2 {
            return;
        }

        let mut pb = PathBuilder::new();
        pb.move_to(points[0].x, points[0].y);

        for point in &points[1..] {
            pb.line_to(point.x, point.y);
        }
        pb.close();

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(stroke.color));

            let skia_stroke = SkiaStroke {
                width: stroke.width,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            };

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.stroke_path(&path, &paint, &skia_stroke, Transform::identity(), None);
        }
    }

    fn convex_polygon(&self, points: &[Point2d], fill: Color) {
        if points.len() < 3 {
            return;
        }

        let mut pb = PathBuilder::new();
        pb.move_to(points[0].x, points[0].y);

        for point in &points[1..] {
            pb.line_to(point.x, point.y);
        }
        pb.close();

        if let Some(path) = pb.finish() {
            let mut paint = Paint::default();
            paint.set_color(color_to_skia(fill));

            let mut pixmap = self.pixmap.borrow_mut();
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
        }
    }

    fn layout_no_wrap(&self, text: String, font: FontId, _color: Color) -> Galley {
        // Simple text width estimation
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
