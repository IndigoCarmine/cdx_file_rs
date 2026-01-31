use crate::modes::{ModeContext, ModeHandler};
use eframe::egui;

pub struct ViewMode;

impl ModeHandler for ViewMode {
    fn handle_click(&mut self, _ctx: &mut ModeContext) {}
    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        *ctx.view_offset += ctx.drag_delta;
    }
    fn handle_drag_end(&mut self, _ctx: &mut ModeContext) {}
    fn handle_hover(&self, _ctx: &ModeContext, _painter: &egui::Painter) {}
    fn handle_key(&mut self, _ctx: &mut ModeContext, _key: egui::Key) -> bool {
        false
    }
}
