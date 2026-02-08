use crate::cdx::file::NodePayload;
use crate::cdx::table::Table;
use crate::cdx::values::{Point2d, Rectangle};
use crate::renderer::{Drawable, RenderContext};

use dendron::Node;
use std::collections::HashSet;

impl Drawable for Table {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &crate::renderer::RenderContext<P>,
    ) {
        // Table outer box drawing - use bounding_box if available
        // If not, we rely on draw_with_node to calculate from children
        if let Some(bbox) = &self.bounding_box {
            use crate::renderer::backend::Rect;

            let stroke = self.get_stroke(ctx);

            let top_left = ctx.cdx_to_screen(&Point2d {
                x: bbox.left,
                y: bbox.top,
            });
            let bottom_right = ctx.cdx_to_screen(&Point2d {
                x: bbox.right,
                y: bbox.bottom,
            });
            let rect = Rect::from_min_max(top_left, bottom_right);

            ctx.painter.rect_stroke(rect, 0.0, stroke);
        }
    }

    fn draw_with_node<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        node: &Node<NodePayload>,
    ) {
        // Collect all cell positions from descendant TextObjects
        let cell_positions = self.collect_cell_positions(node);

        if cell_positions.is_empty() {
            // No cells found, just draw outer box if available
            self.draw(ctx);
            return;
        }

        let stroke = self.get_stroke(ctx);

        // Collect unique x and y coordinates for grid lines
        // Use i64 bits for precise HashSet comparison
        let mut x_coords: HashSet<i64> = HashSet::new();
        let mut y_coords: HashSet<i64> = HashSet::new();

        for pos in &cell_positions {
            x_coords.insert(pos.x.to_bits() as i64);
            y_coords.insert(pos.y.to_bits() as i64);
        }

        // Convert back to f64 and sort
        let mut x_sorted: Vec<f64> = x_coords
            .iter()
            .map(|&bits| f64::from_bits(bits as u64))
            .collect();
        let mut y_sorted: Vec<f64> = y_coords
            .iter()
            .map(|&bits| f64::from_bits(bits as u64))
            .collect();
        x_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Filter out positions at (0,0) which are hidden/invalid cells
        x_sorted.retain(|&x| x > 1.0);
        y_sorted.retain(|&y| y > 1.0);

        if x_sorted.len() < 2 || y_sorted.len() < 2 {
            // Not enough coordinates to form a grid
            self.draw(ctx);
            return;
        }

        // Calculate cell padding/margins from the positions
        // In stoichiometrygrid, positions are text positions, so we need to add margins
        let row_height = if y_sorted.len() >= 2 {
            y_sorted[1] - y_sorted[0]
        } else {
            15.0 // default row height
        };

        let col_width = if x_sorted.len() >= 2 {
            x_sorted[1] - x_sorted[0]
        } else {
            50.0 // default column width
        };

        // Grid boundaries with padding
        let padding_x = col_width * 0.5;
        let padding_y = row_height * 0.6;

        let x_min = *x_sorted.first().unwrap() - padding_x;
        let x_max = *x_sorted.last().unwrap() + padding_x;
        let y_min = *y_sorted.first().unwrap() - padding_y;
        let y_max = *y_sorted.last().unwrap() + padding_y;

        // Draw outer rectangle
        let top_left = ctx.cdx_to_screen(&Point2d { x: x_min, y: y_min });
        let bottom_right = ctx.cdx_to_screen(&Point2d { x: x_max, y: y_max });
        let outer_rect = crate::renderer::backend::Rect::from_min_max(top_left, bottom_right);
        ctx.painter.rect_stroke(outer_rect, 0.0, stroke);

        // Determine column boundaries from x positions
        // Each unique x represents a column's text start position
        // Column boundaries are midpoints between consecutive x positions
        let mut col_boundaries: Vec<f64> = Vec::new();
        col_boundaries.push(x_min); // left edge
        for i in 0..x_sorted.len() - 1 {
            let mid = (x_sorted[i] + x_sorted[i + 1]) / 2.0;
            col_boundaries.push(mid);
        }
        col_boundaries.push(x_max); // right edge

        // Determine row boundaries from y positions
        let mut row_boundaries: Vec<f64> = Vec::new();
        row_boundaries.push(y_min); // top edge
        for i in 0..y_sorted.len() - 1 {
            let mid = (y_sorted[i] + y_sorted[i + 1]) / 2.0;
            row_boundaries.push(mid);
        }
        row_boundaries.push(y_max); // bottom edge

        // Draw vertical column lines (skip first and last as they're part of outer rect)
        for &x in col_boundaries
            .iter()
            .skip(1)
            .take(col_boundaries.len().saturating_sub(2))
        {
            let top = ctx.cdx_to_screen(&Point2d { x, y: y_min });
            let bottom = ctx.cdx_to_screen(&Point2d { x, y: y_max });
            ctx.painter.line_segment(top, bottom, stroke);
        }

        // Draw horizontal row lines (skip first and last as they're part of outer rect)
        for &y in row_boundaries
            .iter()
            .skip(1)
            .take(row_boundaries.len().saturating_sub(2))
        {
            let left = ctx.cdx_to_screen(&Point2d { x: x_min, y });
            let right = ctx.cdx_to_screen(&Point2d { x: x_max, y });
            ctx.painter.line_segment(left, right, stroke);
        }
    }
    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}

impl Table {
    /// Get the stroke width for this table
    fn get_line_width<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
    ) -> f64 {
        self.line_width.unwrap_or_else(|| ctx.default_line_width())
    }

    /// Get the stroke color for this table
    fn get_color<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
    ) -> crate::renderer::backend::Color {
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
    fn get_stroke<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
    ) -> crate::renderer::backend::Stroke {
        use crate::renderer::backend::Stroke;

        let line_width = self.get_line_width(ctx);
        let scale = ctx.zoom * ctx.auto_scale;
        let stroke_width = (line_width * scale as f64) as f32;
        let color = self.get_color(ctx);
        Stroke::new(stroke_width, color)
    }

    /// Collect positions from all descendant TextObjects
    /// This is used to determine grid structure for stoichiometrygrid
    fn collect_cell_positions(&self, table_node: &Node<NodePayload>) -> Vec<Point2d> {
        let mut positions = Vec::new();
        self.collect_positions_recursive(table_node, &mut positions);
        positions
    }

    /// Recursively collect TextObject positions from all descendants
    fn collect_positions_recursive(&self, node: &Node<NodePayload>, positions: &mut Vec<Point2d>) {
        for child in node.children() {
            let child_data = child.borrow_data();

            // Extract position from TextObject
            if let NodePayload::TextObject(text) = &*child_data {
                if let Some(pos) = &text.position_2d {
                    positions.push(pos.clone());
                }
            }

            // Recurse into children
            drop(child_data);
            self.collect_positions_recursive(&child, positions);
        }
    }
}
