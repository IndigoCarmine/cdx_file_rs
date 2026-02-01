use crate::cdx::graphic::Graphic;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;

// Graphic type constants
const GRAPHIC_TYPE_LINE: i16 = 1;
const GRAPHIC_TYPE_ARC: i16 = 2;
const GRAPHIC_TYPE_RECTANGLE: i16 = 3;
const GRAPHIC_TYPE_OVAL: i16 = 4;
const GRAPHIC_TYPE_ORBITAL: i16 = 5;
const GRAPHIC_TYPE_BRACKET: i16 = 6;
const GRAPHIC_TYPE_SYMBOL: i16 = 7;

impl Drawable for Graphic {
    fn draw(&self, ctx: &RenderContext) {
        let graphic_type = self.graphic_type.unwrap_or(GRAPHIC_TYPE_LINE);

        // Debug: Log if this is an arrow
        if self.arrow_type.is_some() {
            eprintln!(
                "Drawing Graphic id={}: type={}, arrow_type={:?}, bbox={:?}",
                self.id, graphic_type, self.arrow_type, self.bounding_box
            );
        }

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
    fn draw_line(&self, ctx: &RenderContext) {
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

            // Debug: Log arrow coordinates
            if self.arrow_type.is_some() {
                eprintln!(
                    "Arrow line_seg: start={:?}, end={:?}, bbox={:?}",
                    start_pos, end_pos, bbox
                );
            }

            (start_pos, end_pos)
        } else {
            return; // No position data
        };

        let color = self.get_color(ctx);
        let stroke = egui::Stroke::new(self.get_line_width(), color);

        ctx.painter.line_segment([start, end], stroke);

        // Draw arrowhead if specified
        if let Some(_arrow_type) = self.arrow_type {
            self.draw_arrowhead(ctx, start, end, color);
        }
    }

    fn draw_rectangle(&self, ctx: &RenderContext) {
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
        let rect = egui::Rect::from_two_pos(top_left, bottom_right);

        let color = self.get_color(ctx);
        let stroke = egui::Stroke::new(self.get_line_width(), color);

        // Check if filled
        if let Some(bg_color_idx) = self.background_color.filter(|&idx| idx >= 0) {
            let fill_color = self.get_document_color(ctx, bg_color_idx as u16);
            ctx.painter.rect(rect, 0.0, fill_color, stroke);
            return;
        }

        // Just outline
        ctx.painter.rect_stroke(rect, 0.0, stroke);
    }

    fn draw_oval(&self, ctx: &RenderContext) {
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
        let scale = ctx.zoom * ctx.auto_scale;
        let radius_screen_x = (radius_x * scale as f64) as f32;
        let radius_screen_y = (radius_y * scale as f64) as f32;

        let color = self.get_color(ctx);
        let stroke = egui::Stroke::new(self.get_line_width(), color);

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
                points.push(egui::pos2(x, y));
            }
            points.push(points[0]); // Close the loop
            ctx.painter.add(egui::Shape::line(points, stroke));
        }
    }

    fn draw_arc(&self, ctx: &RenderContext) {
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
        let scale = ctx.zoom * ctx.auto_scale;
        let radius_screen = (radius * scale as f64) as f32;

        let color = self.get_color(ctx);
        let stroke = egui::Stroke::new(self.get_line_width(), color);

        // Arc angular size in degrees (default 90)
        let arc_size = self.arc_angular_size.unwrap_or(90) as f32;
        let arc_radians = arc_size.to_radians();

        // Draw arc as line segments
        let num_segments = ((arc_size / 5.0) as usize).max(8);
        let mut points = Vec::with_capacity(num_segments + 1);
        for i in 0..=num_segments {
            let angle = (i as f32 / num_segments as f32) * arc_radians;
            let x = center.x + radius_screen * angle.cos();
            let y = center.y + radius_screen * angle.sin();
            points.push(egui::pos2(x, y));
        }
        ctx.painter.add(egui::Shape::line(points, stroke));
    }

    fn draw_bracket(&self, ctx: &RenderContext) {
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

        let color = self.get_color(ctx);
        let stroke = egui::Stroke::new(self.get_line_width(), color);

        // Bracket lip size (default 5% of height)
        let height = (bottom_left.y - top_left.y).abs();
        let lip_size = self
            .bracket_lip_size
            .map(|s| s as f32)
            .unwrap_or(height * 0.05);

        // Draw left bracket: [
        ctx.painter.line_segment([top_left, bottom_left], stroke);
        ctx.painter.line_segment(
            [top_left, egui::pos2(top_left.x + lip_size, top_left.y)],
            stroke,
        );
        ctx.painter.line_segment(
            [
                bottom_left,
                egui::pos2(bottom_left.x + lip_size, bottom_left.y),
            ],
            stroke,
        );

        // Draw right bracket: ]
        ctx.painter.line_segment([top_right, bottom_right], stroke);
        ctx.painter.line_segment(
            [top_right, egui::pos2(top_right.x - lip_size, top_right.y)],
            stroke,
        );
        ctx.painter.line_segment(
            [
                bottom_right,
                egui::pos2(bottom_right.x - lip_size, bottom_right.y),
            ],
            stroke,
        );
    }

    fn draw_arrowhead(
        &self,
        ctx: &RenderContext,
        _start: egui::Pos2,
        end: egui::Pos2,
        color: egui::Color32,
    ) {
        // Simple triangle arrowhead

        // Vector pointing from start to end
        let dir_x = end.x - _start.x;
        let dir_y = end.y - _start.y;
        let length = (dir_x * dir_x + dir_y * dir_y).sqrt();

        if length < 0.001 {
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
            let base_size = (arrow_size as f32) * 0.01 * 96.0 * scale / 100.0;
            base_size.min(length / 3.0) // Cap at 1/3 of line length
        } else {
            (length / 5.0).min(20.0 * scale) // Default: 1/5 of line or 20 screen pixels
        };

        eprintln!(
            "Arrowhead: size={}, arrowhead_size={:?}, scale={}, line_length={}",
            size, self.arrowhead_size, scale, length
        );

        // Arrowhead points
        let base_x = end.x - norm_x * size;
        let base_y = end.y - norm_y * size;
        let side_offset = size * 0.4;

        let p1 = end;
        let p2 = egui::pos2(base_x + perp_x * side_offset, base_y + perp_y * side_offset);
        let p3 = egui::pos2(base_x - perp_x * side_offset, base_y - perp_y * side_offset);

        eprintln!("Arrowhead points: p1={:?}, p2={:?}, p3={:?}", p1, p2, p3);

        ctx.painter.add(egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            egui::Stroke::NONE,
        ));
    }

    fn get_color(&self, ctx: &RenderContext) -> egui::Color32 {
        if let Some(color_idx) = self.foreground_color {
            self.get_document_color(ctx, color_idx)
        } else {
            egui::Color32::BLACK
        }
    }

    fn get_document_color(&self, ctx: &RenderContext, index: u16) -> egui::Color32 {
        if let Some(color_table) = &ctx.document.color_table {
            if (index as usize) < color_table.colors.len() {
                return color_table.colors[index as usize].to_color32();
            }
        }
        egui::Color32::BLACK
    }

    fn get_line_width(&self) -> f32 {
        self.line_width.map(|w| w as f32).unwrap_or(1.0)
    }
}
