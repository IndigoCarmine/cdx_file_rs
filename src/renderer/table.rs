use crate::cdx::table::Table;
use crate::cdx::values::Point2d;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;
use std::collections::HashSet;

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

        // Collect all unique vertical and horizontal lines from child Pages
        // Pages are children of the Table in the tree structure
        // We need to extract the bounds of each Page and draw grid lines between them
        
        // For now, draw the table's outer bounding box
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
        
        // Note: To draw internal grid lines, we would need access to child Page objects
        // which contain bounds_in_parent property. Since we only have access to the Table
        // itself in this draw method, we can only draw the outer border.
        // The grid lines should be drawn based on the child Pages' bounds_in_parent properties.
        // This would require traversing the tree structure to get child nodes.
    }
}

impl Table {
    /// Helper method to draw grid lines based on cell positions
    /// This would be called if we had access to child pages
    #[allow(dead_code)]
    fn draw_grid_lines(
        &self,
        ctx: &RenderContext,
        cell_bounds: &[crate::cdx::values::Rectangle],
        stroke: egui::Stroke,
    ) {
        // Collect unique x and y coordinates for grid lines
        let mut x_coords = HashSet::new();
        let mut y_coords = HashSet::new();

        for bounds in cell_bounds {
            x_coords.insert(bounds.left.to_bits());
            x_coords.insert(bounds.right.to_bits());
            y_coords.insert(bounds.top.to_bits());
            y_coords.insert(bounds.bottom.to_bits());
        }

        // Convert back to f64 and sort
        let mut x_sorted: Vec<f64> = x_coords.iter().map(|&bits| f64::from_bits(bits)).collect();
        let mut y_sorted: Vec<f64> = y_coords.iter().map(|&bits| f64::from_bits(bits)).collect();
        x_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if x_sorted.is_empty() || y_sorted.is_empty() {
            return;
        }

        let y_min = *y_sorted.first().unwrap();
        let y_max = *y_sorted.last().unwrap();
        let x_min = *x_sorted.first().unwrap();
        let x_max = *x_sorted.last().unwrap();

        // Draw vertical lines
        for &x in &x_sorted {
            let top = ctx.cdx_to_screen(&Point2d { x, y: y_min });
            let bottom = ctx.cdx_to_screen(&Point2d { x, y: y_max });
            ctx.painter.line_segment([top, bottom], stroke);
        }

        // Draw horizontal lines
        for &y in &y_sorted {
            let left = ctx.cdx_to_screen(&Point2d { x: x_min, y });
            let right = ctx.cdx_to_screen(&Point2d { x: x_max, y });
            ctx.painter.line_segment([left, right], stroke);
        }
    }
}
