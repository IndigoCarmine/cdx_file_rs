use crate::cdx::table::Table;
use crate::cdx::values::Point2d;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;

impl Drawable for Table {
    fn draw(&self, ctx: &RenderContext) {
        // Get the bounding box - if not available, nothing to draw
        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        // Get the line width (default from table or document)
        let line_width = self.line_width
            .unwrap_or_else(|| ctx.default_line_width());
        let scale = ctx.zoom * ctx.auto_scale;
        let stroke_width = (line_width * scale as f64) as f32;

        // Get the color (use foreground color if available, otherwise default)
        let color = if let Some(color_idx) = self.foreground_color {
            ctx.document
                .get_color_table()
                .and_then(|ct| ct.get(color_idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(egui::Color32::BLACK)
        } else {
            ctx.default_label_color()
        };

        let stroke = egui::Stroke::new(stroke_width, color);

        // Draw the outer bounding box
        let top_left = ctx.cdx_to_screen(&Point2d {
            x: bbox.left,
            y: bbox.top,
        });
        let bottom_right = ctx.cdx_to_screen(&Point2d {
            x: bbox.right,
            y: bbox.bottom,
        });
        let rect = egui::Rect::from_two_pos(top_left, bottom_right);
        
        // Draw the outer rectangle
        ctx.painter.rect_stroke(rect, 0.0, stroke);
        
        // Note: Internal grid lines are drawn by CdxRenderer::draw_table_grid
        // based on the bounds_in_parent properties of child Page objects.
    }
}
