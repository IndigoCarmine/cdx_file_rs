use crate::modes::{ModeContext, ModeHandler};
use eframe::egui;

pub struct BondMode;

impl ModeHandler for BondMode {
    fn handle_click(&mut self, _ctx: &mut ModeContext) {
        // TODO: Implement bond creation
    }
    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        *ctx.view_offset += ctx.drag_delta;
    }
    fn handle_drag_end(&mut self, _ctx: &mut ModeContext) {
        // TODO: Complete bond creation
    }
    fn handle_hover(&self, _ctx: &ModeContext, _painter: &egui::Painter) {
        // TODO: Implement hover preview
    }
    fn handle_key(&mut self, _ctx: &mut ModeContext, _key: egui::Key) -> bool {
        false
    }
}
