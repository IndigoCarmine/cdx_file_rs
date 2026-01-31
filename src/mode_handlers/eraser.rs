use crate::modes::{ModeContext, ModeHandler};
use eframe::egui;

pub struct EraserMode;

impl ModeHandler for EraserMode {
    fn handle_click(&mut self, _ctx: &mut ModeContext) {
        // TODO: Implement object deletion
    }
    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        *ctx.view_offset += ctx.drag_delta;
    }
    fn handle_drag_end(&mut self, _ctx: &mut ModeContext) {
        // TODO: Complete eraser action
    }
    fn handle_hover(&self, _ctx: &ModeContext, _painter: &egui::Painter) {
        // TODO: Implement eraser preview
    }
    fn handle_key(&mut self, _ctx: &mut ModeContext, _key: egui::Key) -> bool {
        false
    }
}
