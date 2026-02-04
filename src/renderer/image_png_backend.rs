/// PNG backend implementation using image + imageproc + ab_glyph
///
/// This module provides an AbstractPainter implementation for PNG rendering
/// using the `image`, `imageproc`, and `ab_glyph` crates.

use super::backend::*;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{
    draw_antialiased_line_segment_mut, draw_filled_circle_mut, draw_filled_rect_mut,
    draw_hollow_circle_mut, draw_hollow_rect_mut, draw_polygon_mut, draw_text_mut,
};
use imageproc::pixelops::interpolate;
use imageproc::point::Point as ImagePoint;
use imageproc::rect::Rect as ImageRect;
use std::cell::RefCell;

/// Convert backend Color to image Rgba
fn color_to_rgba(c: Color) -> Rgba<u8> {
    Rgba([c.r, c.g, c.b, c.a])
}

/// Embedded default font (DejaVu Sans Mono)
const DEFAULT_FONT_DATA: &[u8] = include_bytes!("../../assets/fonts/DejaVuSansMono.ttf");

/// PNG backend using image + imageproc + ab_glyph
pub struct ImagePngBackend<'a> {
    image: RefCell<RgbaImage>,
    clip_rect: Rect,
    font: FontRef<'a>,
}

impl<'a> ImagePngBackend<'a> {
    /// Create a new ImagePngBackend with the specified dimensions and background color
    pub fn new(width: u32, height: u32, background: Color) -> Self {
        let mut image = RgbaImage::new(width, height);
        let bg = color_to_rgba(background);
        
        // Fill background
        for pixel in image.pixels_mut() {
            *pixel = bg;
        }
        
        let font = FontRef::try_from_slice(DEFAULT_FONT_DATA)
            .expect("Failed to load embedded font");
        
        ImagePngBackend {
            image: RefCell::new(image),
            clip_rect: Rect {
                min: Point2d::new(0.0, 0.0),
                max: Point2d::new(width as f32, height as f32),
            },
            font,
        }
    }
    
    /// Create with a custom font
    pub fn with_font(width: u32, height: u32, background: Color, font_data: &'a [u8]) -> Option<Self> {
        let mut image = RgbaImage::new(width, height);
        let bg = color_to_rgba(background);
        
        for pixel in image.pixels_mut() {
            *pixel = bg;
        }
        
        let font = FontRef::try_from_slice(font_data).ok()?;
        
        Some(ImagePngBackend {
            image: RefCell::new(image),
            clip_rect: Rect {
                min: Point2d::new(0.0, 0.0),
                max: Point2d::new(width as f32, height as f32),
            },
            font,
        })
    }
    
    /// Save the image to a PNG file
    pub fn save_png(&self, path: &std::path::Path) -> Result<(), String> {
        self.image
            .borrow()
            .save(path)
            .map_err(|e| e.to_string())
    }
    
    /// Get the raw image data as bytes (PNG encoded)
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = Vec::new();
        let image = self.image.borrow();
        image::DynamicImage::ImageRgba8(image.clone())
            .write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .map_err(|e| e.to_string())?;
        Ok(bytes)
    }
    
    /// Get a reference to the internal image
    pub fn image(&self) -> std::cell::Ref<'_, RgbaImage> {
        self.image.borrow()
    }
    
    /// Get mutable access to the internal image
    pub fn image_mut(&self) -> std::cell::RefMut<'_, RgbaImage> {
        self.image.borrow_mut()
    }
    
    /// Draw a line with specified width using multiple parallel lines
    fn draw_thick_line(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        let mut img = self.image.borrow_mut();
        let color = color_to_rgba(stroke.color);
        
        if stroke.width <= 1.0 {
            // Use antialiased line for thin strokes
            draw_antialiased_line_segment_mut(
                &mut *img,
                (start.x as i32, start.y as i32),
                (end.x as i32, end.y as i32),
                color,
                interpolate,
            );
        } else {
            // Draw thick line by drawing multiple parallel lines
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let len = (dx * dx + dy * dy).sqrt();
            
            if len < 0.001 {
                return;
            }
            
            // Perpendicular direction
            let nx = -dy / len;
            let ny = dx / len;
            
            let half_width = stroke.width / 2.0;
            let steps = (stroke.width.ceil() as i32).max(1);
            
            for i in -steps..=steps {
                let offset = (i as f32 / steps as f32) * half_width;
                let ox = nx * offset;
                let oy = ny * offset;
                
                draw_antialiased_line_segment_mut(
                    &mut *img,
                    ((start.x + ox) as i32, (start.y + oy) as i32),
                    ((end.x + ox) as i32, (end.y + oy) as i32),
                    color,
                    interpolate,
                );
            }
        }
    }
    
    /// Draw a filled circle using the midpoint circle algorithm with antialiasing
    fn draw_circle_filled_impl(&self, center: Point2d, radius: f32, color: Color) {
        let mut img = self.image.borrow_mut();
        let rgba = color_to_rgba(color);
        
        draw_filled_circle_mut(
            &mut *img,
            (center.x as i32, center.y as i32),
            radius as i32,
            rgba,
        );
    }
    
    /// Draw a circle outline
    fn draw_circle_stroke_impl(&self, center: Point2d, radius: f32, stroke: Stroke) {
        let mut img = self.image.borrow_mut();
        let color = color_to_rgba(stroke.color);
        
        // Draw multiple concentric circles for thick strokes
        let half_width = stroke.width / 2.0;
        let steps = (stroke.width.ceil() as i32).max(1);
        
        for i in -steps..=steps {
            let offset = (i as f32 / steps as f32) * half_width;
            let r = (radius + offset).max(0.0) as i32;
            if r > 0 {
                draw_hollow_circle_mut(
                    &mut *img,
                    (center.x as i32, center.y as i32),
                    r,
                    color,
                );
            }
        }
    }
    
    /// Calculate text width for layout
    fn calculate_text_width(&self, text: &str, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scaled_font = self.font.as_scaled(scale);

        let mut width: f32 = 0.0;
        for c in text.chars() {
            let glyph_id = self.font.glyph_id(c);
            width += scaled_font.h_advance(glyph_id);
        }

        width
    }

    /// Calculate text height
    fn calculate_text_height(&self, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scaled_font = self.font.as_scaled(scale);
        scaled_font.height()
    }
}

impl<'a> AbstractPainter for ImagePngBackend<'a> {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        if stroke.width <= 0.0 {
            return;
        }
        self.draw_thick_line(start, end, stroke);
    }
    
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color) {
        if radius <= 0.0 {
            return;
        }
        self.draw_circle_filled_impl(center, radius, color);
    }
    
    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke) {
        if radius <= 0.0 || stroke.width <= 0.0 {
            return;
        }
        self.draw_circle_stroke_impl(center, radius, stroke);
    }
    
    fn rect_filled(&self, rect: Rect, _rounding: f32, color: Color) {
        let mut img = self.image.borrow_mut();
        let rgba = color_to_rgba(color);
        
        let x = rect.min.x as i32;
        let y = rect.min.y as i32;
        let w = rect.width() as u32;
        let h = rect.height() as u32;
        
        if w == 0 || h == 0 {
            return;
        }
        
        // Note: imageproc doesn't support rounded rectangles directly
        // For now, we draw a regular rectangle
        let img_rect = ImageRect::at(x, y).of_size(w, h);
        draw_filled_rect_mut(&mut *img, img_rect, rgba);
    }

    fn rect_stroke(&self, rect: Rect, _rounding: f32, stroke: Stroke) {
        let mut img = self.image.borrow_mut();
        let color = color_to_rgba(stroke.color);
        
        let x = rect.min.x as i32;
        let y = rect.min.y as i32;
        let w = rect.width() as u32;
        let h = rect.height() as u32;
        
        if w == 0 || h == 0 {
            return;
        }
        
        let img_rect = ImageRect::at(x, y).of_size(w, h);
        draw_hollow_rect_mut(&mut *img, img_rect, color);

        // Draw additional rectangles for thick strokes
        if stroke.width > 1.0 {
            let half = (stroke.width / 2.0) as i32;
            for i in 1..=half {
                // Outer
                let outer = ImageRect::at(x - i, y - i).of_size(w + 2 * i as u32, h + 2 * i as u32);
                draw_hollow_rect_mut(&mut *img, outer, color);
                // Inner
                if w > 2 * i as u32 && h > 2 * i as u32 {
                    let inner =
                        ImageRect::at(x + i, y + i).of_size(w - 2 * i as u32, h - 2 * i as u32);
                    draw_hollow_rect_mut(&mut *img, inner, color);
                }
            }
        }
    }
    
    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke) {
        self.rect_filled(rect, rounding, fill);
        if stroke.width > 0.0 {
            self.rect_stroke(rect, rounding, stroke);
        }
    }
    
    fn polyline(&self, points: &[Point2d], stroke: Stroke) {
        if points.len() < 2 || stroke.width <= 0.0 {
            return;
        }
        
        for window in points.windows(2) {
            self.draw_thick_line(window[0], window[1], stroke);
        }
    }
    
    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke) {
        if points.len() < 2 || stroke.width <= 0.0 {
            return;
        }
        
        // Draw all segments including closing segment
        for window in points.windows(2) {
            self.draw_thick_line(window[0], window[1], stroke);
        }
        
        // Close the polygon
        if let (Some(first), Some(last)) = (points.first(), points.last()) {
            self.draw_thick_line(*last, *first, stroke);
        }
    }
    
    fn convex_polygon(&self, points: &[Point2d], fill: Color) {
        if points.len() < 3 {
            return;
        }
        
        let mut img = self.image.borrow_mut();
        let rgba = color_to_rgba(fill);
        
        let image_points: Vec<ImagePoint<i32>> = points
            .iter()
            .map(|p| ImagePoint::new(p.x as i32, p.y as i32))
            .collect();
        
        draw_polygon_mut(&mut *img, &image_points, rgba);
    }
    
    fn layout_no_wrap(&self, text: String, font: FontId, _color: Color) -> Galley {
        let width = self.calculate_text_width(&text, font.size);
        let height = self.calculate_text_height(font.size);

        Galley {
            size: (width, height),
            text,
        }
    }
    
    fn clip_rect(&self) -> Rect {
        self.clip_rect
    }

    fn rich_text(&self, pos: Point2d, align: Align2, spans: &[super::backend::TextSpan]) {
        use ab_glyph::PxScale;
        use super::backend::Stroke;
        
        if spans.is_empty() {
            return;
        }

        // Calculate total width for alignment
        let total_width: f32 = spans.iter().map(|span| {
            self.calculate_text_width(&span.text, span.font_size)
        }).sum();
        
        let first_span_height = self.calculate_text_height(spans[0].font_size);

        // Determine starting X position based on horizontal alignment
        let start_x = match align.x {
            Align::Left => pos.x,
            Align::Center => pos.x - total_width / 2.0,
            Align::Right => pos.x - total_width,
        };

        // Calculate base Y position based on vertical alignment
        let base_y = match align.y {
            VerticalAlign::Top => pos.y,
            VerticalAlign::Center => pos.y - first_span_height / 2.0,
            VerticalAlign::Bottom => pos.y - first_span_height,
        };

        // Draw each span
        let mut current_x = start_x;
        for span in spans {
            let scale = PxScale::from(span.font_size);
            let rgba = color_to_rgba(span.color);
            
            // Calculate Y offset for subscript/superscript
            let y_offset = if span.style.superscript {
                -span.font_size * 0.3
            } else if span.style.subscript {
                span.font_size * 0.3
            } else {
                0.0
            };

            let span_y = base_y + y_offset;
            
            {
                let mut img = self.image.borrow_mut();
                draw_text_mut(&mut *img, rgba, current_x as i32, span_y as i32, scale, &self.font, &span.text);
            }

            // Draw underline if needed
            if span.style.underline {
                let span_width = self.calculate_text_width(&span.text, span.font_size);
                let underline_y = span_y + span.font_size * 0.9;
                self.draw_thick_line(
                    Point2d::new(current_x, underline_y),
                    Point2d::new(current_x + span_width, underline_y),
                    Stroke::new(1.0, span.color),
                );
            }

            // Advance X position
            current_x += self.calculate_text_width(&span.text, span.font_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_backend() {
        let backend = ImagePngBackend::new(100, 100, Color::WHITE);
        assert_eq!(backend.clip_rect().width(), 100.0);
        assert_eq!(backend.clip_rect().height(), 100.0);
    }
    
    #[test]
    fn test_draw_line() {
        let backend = ImagePngBackend::new(100, 100, Color::WHITE);
        backend.line_segment(
            Point2d::new(10.0, 10.0),
            Point2d::new(90.0, 90.0),
            Stroke::new(2.0, Color::BLACK),
        );
    }
    
    #[test]
    fn test_draw_circle() {
        let backend = ImagePngBackend::new(100, 100, Color::WHITE);
        backend.circle_filled(Point2d::new(50.0, 50.0), 20.0, Color::RED);
        backend.circle_stroke(Point2d::new(50.0, 50.0), 30.0, Stroke::new(2.0, Color::BLUE));
    }
    
    #[test]
    fn test_draw_rect() {
        let backend = ImagePngBackend::new(100, 100, Color::WHITE);
        let rect = Rect::from_min_max(Point2d::new(10.0, 10.0), Point2d::new(90.0, 90.0));
        backend.rect(rect, 0.0, Color::GREEN, Stroke::new(1.0, Color::BLACK));
    }
    
    #[test]
    fn test_draw_text() {
        let backend = ImagePngBackend::new(200, 100, Color::WHITE);
        backend.text(
            Point2d::new(100.0, 50.0),
            Align2::CENTER_CENTER,
            "Hello",
            FontId::new(24.0, FontFamily::Proportional),
            Color::BLACK,
        );
    }
}
