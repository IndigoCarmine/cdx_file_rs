use crate::cdx::arrow::Arrow;
use crate::cdx::values::{Point2d as CdxPoint2d, Point3d, Rectangle};
use crate::renderer::{
    Drawable, RenderContext,
    backend::{Color, Point2d as BackendPoint2d, Stroke},
};

impl Drawable for Arrow {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &crate::renderer::RenderContext<P>,
    ) {
        // Draw arrow based on 3D head and tail positions
        // If 3D coordinates exist, project them to 2D using z as depth

        let (head_pos, tail_pos) =
            if let (Some(Point3d { x: hx, y: hy, .. }), Some(Point3d { x: tx, y: ty, .. })) =
                (&self.head_3d, &self.tail_3d)
            {
                // 3D points are stored as IEEE-754 doubles already in CDX pts (no /65536 needed)
                let hp = CdxPoint2d { x: *hx, y: *hy };
                let tp = CdxPoint2d { x: *tx, y: *ty };
                (ctx.cdx_to_screen(&hp), ctx.cdx_to_screen(&tp))
            } else if let Some(Rectangle {
                left: x1,
                top: y1,
                right: x2,
                bottom: y2,
            }) = &self.bounding_box
            {
                // bounding_box (16-byte format) stores raw CDX fixed-point integers;
                // divide by 65536 to convert to CDX pts before passing to cdx_to_screen.
                // For a horizontal arrow: left/top = tail, right/bottom = head.
                let tail_pt = CdxPoint2d { x: *x1 / 65536.0, y: *y1 / 65536.0 };
                let head_pt = CdxPoint2d { x: *x2 / 65536.0, y: *y2 / 65536.0 };
                (ctx.cdx_to_screen(&head_pt), ctx.cdx_to_screen(&tail_pt))
            } else {
                return;
            };

        // line_width is a raw CDX fixed-point (÷65536 = CDX pts); apply auto_scale→px
        let line_width = ctx.cdx_length_to_screen(self.line_width.unwrap_or(ctx.default_line_width()) / 65536.0);
        let color = ctx.resolve_color(self.foreground_color, Color::BLACK);

        let stroke = Stroke::new(line_width, color);

        // Draw main line
        ctx.painter.line_segment(tail_pos, head_pos, stroke);

        // Draw arrowhead if specified
        if self.arrowhead_head.is_some() {
            // Use a fixed arrowhead size proportional to bond length on screen
            let arrowhead_px = ctx.style.arrowhead_size_default * ctx.auto_scale * ctx.zoom;
            draw_arrowhead(ctx, head_pos, tail_pos, stroke, arrowhead_px);
        }
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}

fn draw_arrowhead<P: crate::renderer::backend::AbstractPainter>(
    ctx: &crate::renderer::RenderContext<P>,
    tip: BackendPoint2d,
    tail: BackendPoint2d,
    stroke: Stroke,
    size: f32,
) {
    let dx = tip.x - tail.x;
    let dy = tip.y - tail.y;
    let len = (dx * dx + dy * dy).sqrt();

    if len < ctx.style.arrowhead_min_length {
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

    let p1 = BackendPoint2d::new(
        tip.x - ux * arrow_len + px * arrow_width,
        tip.y - uy * arrow_len + py * arrow_width,
    );
    let p2 = BackendPoint2d::new(
        tip.x - ux * arrow_len - px * arrow_width,
        tip.y - uy * arrow_len - py * arrow_width,
    );

    // Draw arrowhead as two lines forming a triangle
    ctx.painter.line_segment(tip, p1, stroke);
    ctx.painter.line_segment(tip, p2, stroke);
    ctx.painter.line_segment(p1, p2, stroke);
}
