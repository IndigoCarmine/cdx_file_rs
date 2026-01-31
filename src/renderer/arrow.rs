use eframe::egui;
use egui::{Pos2, Stroke, Color32};
use crate::cdx::arrow::Arrow;
use crate::cdx::values::{Point2d, Point3d, Rectangle};
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Arrow {
    fn draw(&self, ctx: &RenderContext) {
        // Draw arrow based on 3D head and tail positions
        // If 3D coordinates exist, project them to 2D using z as depth
        
        let (head_pos, tail_pos) = if let (Some(Point3d { x: hx, y: hy, .. }), Some(Point3d { x: tx, y: ty, .. })) = (&self.head_3d, &self.tail_3d) {
            // Use 3D coordinates projected to 2D (ignoring z for now)
            let hp = Point2d { x: *hx, y: *hy };
            let tp = Point2d { x: *tx, y: *ty };
            (ctx.cdx_to_screen(&hp), ctx.cdx_to_screen(&tp))
        } else if let Some(Rectangle { left: x1, top: y1, right: x2, bottom: y2 }) = &self.bounding_box {
            // Use bounding box as fallback: (x1,y1) is start, (x2,y2) is end
            let p1 = Point2d { x: *x1, y: *y1 };
            let p2 = Point2d { x: *x2, y: *y2 };
            (ctx.cdx_to_screen(&p1), ctx.cdx_to_screen(&p2))
        } else {
            // Not enough data to draw
            return;
        };

        // Get visual properties
        let line_width = self.line_width.unwrap_or(1.0) as f32;
        let color_idx = self.foreground_color.unwrap_or(0) as usize;
        let color = ctx.document.get_color_table()
            .and_then(|ct| ct.get(color_idx))
            .map(|c| c.to_color32())
            .unwrap_or(Color32::BLACK);

        let stroke = Stroke::new(line_width, color);

        // Draw main line
        ctx.painter.line_segment([tail_pos, head_pos], stroke);

        // Draw arrowhead if specified
        if self.arrowhead_head.is_some() {
            draw_arrowhead(ctx, head_pos, tail_pos, stroke, self.head_size.unwrap_or(10) as f32);
        }
    }
}

fn draw_arrowhead(ctx: &RenderContext, tip: Pos2, tail: Pos2, stroke: Stroke, size: f32) {
    let dx = tip.x - tail.x;
    let dy = tip.y - tail.y;
    let len = (dx * dx + dy * dy).sqrt();
    
    if len < 0.01 {
        return; // Too short
    }

    // Unit vector from tail to tip
    let ux = dx / len;
    let uy = dy / len;

    // Perpendicular vector
    let px = -uy;
    let py = ux;

    // Arrowhead points: tip and two base points
    let arrow_len = size;
    let arrow_width = size * 0.5;

    let p1 = Pos2::new(
        tip.x - ux * arrow_len + px * arrow_width, 
        tip.y - uy * arrow_len + py * arrow_width
    );
    let p2 = Pos2::new(
        tip.x - ux * arrow_len - px * arrow_width, 
        tip.y - uy * arrow_len - py * arrow_width
    );

    // Draw arrowhead as two lines forming a triangle
    ctx.painter.line_segment([tip, p1], stroke);
    ctx.painter.line_segment([tip, p2], stroke);
    ctx.painter.line_segment([p1, p2], stroke);
}
