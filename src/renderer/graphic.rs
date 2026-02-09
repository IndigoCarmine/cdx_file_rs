use crate::cdx::graphic::Graphic;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;

// Graphic type constants
const GRAPHIC_TYPE_LINE: i16 = 1;
const GRAPHIC_TYPE_ARC: i16 = 2;
const GRAPHIC_TYPE_RECTANGLE: i16 = 3;
const GRAPHIC_TYPE_OVAL: i16 = 4;
#[allow(dead_code)]
const GRAPHIC_TYPE_ORBITAL: i16 = 5;
const GRAPHIC_TYPE_BRACKET: i16 = 6;
#[allow(dead_code)]
const GRAPHIC_TYPE_SYMBOL: i16 = 7;

impl Drawable for Graphic {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &crate::renderer::RenderContext<P>) {
        let graphic_type = self.graphic_type.unwrap_or(GRAPHIC_TYPE_LINE);

        match graphic_type {
            GRAPHIC_TYPE_LINE => self.draw_line(ctx),
            GRAPHIC_TYPE_RECTANGLE => self.draw_rectangle(ctx),
            GRAPHIC_TYPE_OVAL => self.draw_oval(ctx),
            GRAPHIC_TYPE_ARC => self.draw_arc(ctx),
            GRAPHIC_TYPE_BRACKET => self.draw_bracket(ctx),
            _ => {} // Orbital, Symbol are complex - skip for now
        }
    }
}

impl Graphic {
    fn draw_line<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;

        let (start, end) = if let (Some(head), Some(tail)) = (&self.head_3d, &self.tail_3d) {
            // Use 3D coordinates
            let start_pos = ctx.cdx_to_screen(&Point2d {
                x: head.x,
                y: head.y,
            });
            let end_pos = ctx.cdx_to_screen(&Point2d {
                x: tail.x,
                y: tail.y,
            });
            (start_pos, end_pos)
        } else if let Some(bbox) = &self.bounding_box {
            // Use bounding box: top-left to bottom-right
            let start_pos = ctx.cdx_to_screen(&Point2d {
                x: bbox.left,
                y: bbox.top,
            });
            let end_pos = ctx.cdx_to_screen(&Point2d {
                x: bbox.right,
                y: bbox.bottom,
            });
  

            (start_pos, end_pos)
        } else {
            return; // No position data
        };

        use crate::renderer::backend::{Stroke, Color as BackendColor};
        
        let color = self.get_color(ctx);
        let stroke = Stroke::new(self.get_line_width() as f32, color);

        ctx.painter.line_segment(start, end, stroke);

        // Draw arrowhead if specified
        if let Some(_arrow_type) = self.arrow_type {
            self.draw_arrowhead(ctx, start, end, color);
        }
    }

    fn draw_rectangle<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;

        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        let top_left = ctx.cdx_to_screen(&Point2d {
            x: bbox.left,
            y: bbox.top,
        });
        let bottom_right = ctx.cdx_to_screen(&Point2d {
            x: bbox.right,
            y: bbox.bottom,
        });
        use crate::renderer::backend::{Rect, Stroke, Color as BackendColor};
        
        let rect = Rect::from_min_max(top_left, bottom_right);
        let color = self.get_color(ctx);
        let stroke = Stroke::new(self.get_line_width() as f32, color);

        // Check if filled
        if let Some(bg_color_idx) = self.background_color.filter(|&idx| idx >= 0) {
            let fill_color = self.get_document_color(ctx, bg_color_idx as u16);
            ctx.painter.rect(rect, 0.0, fill_color, stroke);
            return;
        }

        // Just outline
        ctx.painter.rect_stroke(rect, 0.0, stroke);
    }

    fn draw_oval<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;

        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        let center_x = (bbox.left + bbox.right) / 2.0;
        let center_y = (bbox.top + bbox.bottom) / 2.0;
        let radius_x = (bbox.right - bbox.left).abs() / 2.0;
        let radius_y = (bbox.bottom - bbox.top).abs() / 2.0;

        let center = ctx.cdx_to_screen(&Point2d {
            x: center_x,
            y: center_y,
        });
        let radius_screen_x = ctx.cdx_length_to_screen(radius_x);
        let radius_screen_y = ctx.cdx_length_to_screen(radius_y);

        use crate::renderer::backend::{Stroke, Point2d as BackendPoint2d};
        
        let color = self.get_color(ctx);
        let stroke = Stroke::new(self.get_line_width() as f32, color);

        // Draw ellipse using circle if radii are equal, otherwise approximate
        if (radius_screen_x - radius_screen_y).abs() < 1.0 {
            // Circle
            ctx.painter.circle_stroke(center, radius_screen_x, stroke);
        } else {
            // Ellipse - draw as polygon approximation
            let num_points = 64;
            let mut points = Vec::with_capacity(num_points);
            for i in 0..num_points {
                let angle = (i as f32 / num_points as f32) * 2.0 * std::f32::consts::PI;
                let x = center.x + radius_screen_x * angle.cos();
                let y = center.y + radius_screen_y * angle.sin();
                points.push(BackendPoint2d::new(x, y));
            }
            ctx.painter.polyline_closed(&points, stroke);
        }
    }

    fn draw_arc<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;

        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        let center_x = (bbox.left + bbox.right) / 2.0;
        let center_y = (bbox.top + bbox.bottom) / 2.0;
        let radius = ((bbox.right - bbox.left).abs() + (bbox.bottom - bbox.top).abs()) / 4.0;

        let center = ctx.cdx_to_screen(&Point2d {
            x: center_x,
            y: center_y,
        });
        let radius_screen = ctx.cdx_length_to_screen(radius);

        use crate::renderer::backend::{Stroke, Point2d as BackendPoint2d};
        
        let color = self.get_color(ctx);
        let stroke = Stroke::new(self.get_line_width() as f32, color);

        // Arc angular size in degrees (default 90)
        let arc_size = self.arc_angular_size.unwrap_or(90) as f32;
        let arc_radians = arc_size.to_radians();

        // Draw arc as line segments
        let num_segments = ((arc_size / ctx.style.arc_segment_degrees) as usize).max(8);
        let mut points = Vec::with_capacity(num_segments + 1);
        for i in 0..=num_segments {
            let angle = (i as f32 / num_segments as f32) * arc_radians;
            let x = center.x + radius_screen * angle.cos();
            let y = center.y + radius_screen * angle.sin();
            points.push(BackendPoint2d::new(x, y));
        }
        ctx.painter.polyline(&points, stroke);
    }

    fn draw_bracket<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;

        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        let top_left = ctx.cdx_to_screen(&Point2d {
            x: bbox.left,
            y: bbox.top,
        });
        let bottom_left = ctx.cdx_to_screen(&Point2d {
            x: bbox.left,
            y: bbox.bottom,
        });
        let top_right = ctx.cdx_to_screen(&Point2d {
            x: bbox.right,
            y: bbox.top,
        });
        let bottom_right = ctx.cdx_to_screen(&Point2d {
            x: bbox.right,
            y: bbox.bottom,
        });

        use crate::renderer::backend::{Stroke, Point2d as BackendPoint2d};
        
        let color = self.get_color(ctx);
        let stroke = Stroke::new(self.get_line_width() as f32, color);

        // Bracket lip size (default 5% of height)
        let height = (bottom_left.y - top_left.y).abs();
        let lip_size = self
            .bracket_lip_size
            .map(|s| s as f32)
            .unwrap_or(height * ctx.style.bracket_lip_ratio);

        // Draw left bracket: [
        ctx.painter.line_segment(top_left, bottom_left, stroke);
        ctx.painter.line_segment(
            top_left,
            BackendPoint2d::new(top_left.x + lip_size, top_left.y),
            stroke,
        );
        ctx.painter.line_segment(
            bottom_left,
            BackendPoint2d::new(bottom_left.x + lip_size, bottom_left.y),
            stroke,
        );

        // Draw right bracket: ]
        ctx.painter.line_segment(top_right, bottom_right, stroke);
        ctx.painter.line_segment(
            top_right,
            BackendPoint2d::new(top_right.x - lip_size, top_right.y),
            stroke,
        );
        ctx.painter.line_segment(
            bottom_right,
            BackendPoint2d::new(bottom_right.x - lip_size, bottom_right.y),
            stroke,
        );
    }

    fn draw_arrowhead<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        _start: crate::renderer::backend::Point2d,
        end: crate::renderer::backend::Point2d,
        color: crate::renderer::backend::Color,
    ) {
        use crate::renderer::backend::Point2d as BackendPoint2d;
        
        // Simple triangle arrowhead

        // Vector pointing from start to end
        let dir_x = end.x - _start.x;
        let dir_y = end.y - _start.y;
        let length = (dir_x * dir_x + dir_y * dir_y).sqrt();

        if length < ctx.style.arrowhead_min_length {
            #[cfg(debug_assertions)]
            eprintln!("Arrowhead: line too short, length={}", length);
            return; // Avoid division by zero
        }

        // Normalize
        let norm_x = dir_x / length;
        let norm_y = dir_y / length;

        // Perpendicular vector
        let perp_x = -norm_y;
        let perp_y = norm_x;

        // Calculate arrowhead size:
        // arrowhead_size from CDX is in 0.01 inches (hundredths of an inch)
        // We'll use a proportional size: about 10% of the line length, or a reasonable default
        let scale = ctx.zoom * ctx.auto_scale;
        let size = if let Some(arrow_size) = self.arrowhead_size {
            // arrowhead_size is in 0.01 inches, convert to pixels
            // Typical screen DPI is 96, so 0.01 inch = 0.96 pixels
            // But we need to scale by zoom
            let base_size =
                (arrow_size as f32) * 0.01 * ctx.style.screen_dpi * scale / 100.0;
            base_size.min(length / 3.0) // Cap at 1/3 of line length
        } else {
            (length / 5.0).min(ctx.style.arrowhead_max_screen * scale)
        };


        // Arrowhead points
        let base_x = end.x - norm_x * size;
        let base_y = end.y - norm_y * size;
        let side_offset = size * ctx.style.arrowhead_side_ratio;

        let p1 = end;
        let p2 = BackendPoint2d::new(base_x + perp_x * side_offset, base_y + perp_y * side_offset);
        let p3 = BackendPoint2d::new(base_x - perp_x * side_offset, base_y - perp_y * side_offset);

        ctx.painter.convex_polygon(
            &[p1, p2, p3],
            color,
        );
    }

    fn get_color<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) -> crate::renderer::backend::Color {
        ctx.resolve_color(self.foreground_color, crate::renderer::backend::Color::BLACK)
    }

    fn get_document_color<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>, index: u16) -> crate::renderer::backend::Color {
        ctx.resolve_color(Some(index), crate::renderer::backend::Color::BLACK)
    }

    fn get_line_width(&self) -> f32 {
        self.line_width.map(|w| w as f32).unwrap_or(1.0)
    }
}
