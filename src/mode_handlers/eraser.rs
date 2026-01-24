use crate::modes::{ModeContext, ModeHandler};
use cdx_file_rs::tags;
use eframe::egui;

pub struct EraserMode;

impl ModeHandler for EraserMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        if let Some(hit) = ctx.hovered_object {
            ctx.doc.delete_object(hit.id);
            if hit.tag == tags::NODE {
                ctx.node_positions.remove(&hit.id);
            }
        }
    }
    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        *ctx.view_offset += ctx.drag_delta;
    }
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        if let Some(hit) = ctx.hovered_object {
            let center = ctx.ui.available_rect_before_wrap().center() + ctx.renderer.offset;
            if let Some(pos) = ctx.node_positions.get(&hit.id) {
                let p = ctx.renderer.to_screen(pos, center);
                painter.circle_stroke(
                    p,
                    15.0 * (ctx.renderer.zoom / 5.0).max(0.5),
                    egui::Stroke::new(2.0, egui::Color32::RED),
                );
            }
        }
    }
    fn handle_key(&mut self, _ctx: &mut ModeContext, _key: egui::Key) -> bool {
        false
    }
}
