use crate::cdx::file::NodePayload;
use crate::cdx::table::Table;
use crate::cdx::values::Point2d;
use crate::renderer::{Drawable, RenderContext};
use dendron::Node;
use eframe::egui;
use std::collections::HashSet;

impl Drawable for Table {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &crate::renderer::RenderContext<P>) {
        use crate::renderer::backend::Rect;
        
        // Get the bounding box - if not available, nothing to draw
        let bbox = match &self.bounding_box {
            Some(b) => b,
            None => return,
        };

        let stroke = self.get_stroke(ctx);

        // Draw the outer bounding box
        let top_left = ctx.cdx_to_screen(&Point2d {
            x: bbox.left,
            y: bbox.top,
        });
        let bottom_right = ctx.cdx_to_screen(&Point2d {
            x: bbox.right,
            y: bbox.bottom,
        });
        let rect = Rect::from_min_max(top_left, bottom_right);
        
        // Draw the outer rectangle
        ctx.painter.rect_stroke(rect, 0.0, stroke);
    }
    
    fn draw_with_node<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>, node: &Node<NodePayload>) {
        // First draw the outer bounding box
        self.draw(ctx);
        
        // Then draw grid lines based on child Page bounds
        self.draw_grid_lines(ctx, node);
    }
}

impl Table {
    /// Get the stroke width for this table
    fn get_line_width<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) -> f64 {
        self.line_width.unwrap_or_else(|| ctx.default_line_width())
    }
    
    /// Get the stroke color for this table
    fn get_color<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) -> crate::renderer::backend::Color {
        use crate::renderer::backend::Color as BackendColor;
        
        if let Some(color_idx) = self.foreground_color {
            ctx.document
                .get_color_table()
                .and_then(|ct| ct.get(color_idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or_else(|| ctx.default_foreground_color())
        } else {
            ctx.default_foreground_color()
        }
    }
    
    /// Get the stroke for drawing table lines
    fn get_stroke<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) -> crate::renderer::backend::Stroke {
        use crate::renderer::backend::Stroke;
        
        let line_width = self.get_line_width(ctx);
        let scale = ctx.zoom * ctx.auto_scale;
        let stroke_width = (line_width * scale as f64) as f32;
        let color = self.get_color(ctx);
        Stroke::new(stroke_width, color)
    }
    
    /// Draw internal grid lines based on child Page bounds
    fn draw_grid_lines<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>, table_node: &Node<NodePayload>) {
        // Collect bounds_in_parent from all child Page objects
        let mut cell_bounds = Vec::new();
        for child in table_node.children() {
            let child_data = child.borrow_data();
            if let NodePayload::Page(page) = &*child_data {
                if let Some(bounds) = &page.bounds_in_parent {
                    cell_bounds.push(bounds.clone());
                }
            }
        }

        if cell_bounds.is_empty() {
            // No cells with bounds, nothing to draw
            return;
        }

        let stroke = self.get_stroke(ctx);

        // Collect unique x and y coordinates for grid lines
        let mut x_coords = HashSet::new();
        let mut y_coords = HashSet::new();

        for bounds in &cell_bounds {
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
            ctx.painter.line_segment(top, bottom, stroke);
        }

        // Draw horizontal lines
        for &y in &y_sorted {
            let left = ctx.cdx_to_screen(&Point2d { x: x_min, y });
            let right = ctx.cdx_to_screen(&Point2d { x: x_max, y });
            ctx.painter.line_segment(left, right, stroke);
        }
    }
}
