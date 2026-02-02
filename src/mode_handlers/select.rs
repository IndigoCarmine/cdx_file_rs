use crate::modes::{ModeContext, ModeHandler};
use eframe::egui;

pub struct SelectMode;

impl ModeHandler for SelectMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        // Clear lasso path on click
        ctx.lasso_path.clear();
    }

    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        if ctx.is_dragging {
            // Add current mouse position to lasso path
            ctx.lasso_path.push(ctx.mouse_pos);
        }
    }

    fn handle_drag_end(&mut self, ctx: &mut ModeContext) {
        // Select objects inside lasso when drag ends
        if ctx.lasso_path.len() > 2 {
            ctx.selected_ids.clear();

            // Check each node to see if it's inside the lasso
            for (node_id, node_pos) in ctx.node_positions.iter() {
                let screen_pos = ctx.cdx_to_screen(node_pos);

                if is_point_in_polygon(screen_pos, ctx.lasso_path) {
                    ctx.selected_ids.insert(*node_id);
                }
            }
        }

        // Clear lasso path after selection
        ctx.lasso_path.clear();
    }

    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        // Draw lasso path
        if ctx.lasso_path.len() > 1 {
            let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255));
            painter.add(egui::Shape::Path(egui::epaint::PathShape {
                points: ctx.lasso_path.clone(),
                closed: false,
                fill: egui::Color32::TRANSPARENT,
                stroke: stroke.into(),
            }));
        }

        // Draw selected objects with highlight
        for node_id in ctx.selected_ids.iter() {
            if let Some(pos) = ctx.node_positions.get(node_id) {
                let screen_pos = ctx.cdx_to_screen(pos);
                let radius = 20.0 * ctx.zoom;
                painter.circle_stroke(
                    screen_pos,
                    radius,
                    egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 255)),
                );
            }
        }
    }

    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool {
        match key {
            egui::Key::Delete | egui::Key::Backspace => {
                // TODO: Delete selected objects
                ctx.selected_ids.clear();
                true
            }
            egui::Key::C if ctx.ui.input(|i| i.modifiers.command) => {
                // Copy selected objects to clipboard
                if !ctx.selected_ids.is_empty() {
                    // TODO: Get cdx_file from context (need to pass it through ModeContext)
                    // For now, we'll store the clipboard in the context
                    // ctx.clipboard = Some(cdx_file.extract_selected_subtree(&ctx.selected_ids).ok());
                    true
                } else {
                    false
                }
            }
            egui::Key::V if ctx.ui.input(|i| i.modifiers.command) => {
                // TODO: Paste copied objects
                true
            }
            egui::Key::Escape => {
                ctx.selected_ids.clear();
                true
            }
            _ => false,
        }
    }
}

// Ray casting algorithm to check if point is inside polygon
fn is_point_in_polygon(point: egui::Pos2, polygon: &[egui::Pos2]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}
