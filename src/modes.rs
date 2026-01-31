use crate::renderer::CdxRenderer;
use crate::cdx::values::Point2d;
use crate::cdx::document::Document;
use eframe::egui;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub tag: u16,
    pub id: u32,
}

pub struct ModeContext<'a> {
    pub mouse_pos: egui::Pos2,
    pub ui: &'a egui::Ui,
    pub drag_delta: egui::Vec2,
    pub view_offset: &'a mut egui::Vec2,
    pub renderer: &'a CdxRenderer<'a>,
    pub node_positions: &'a HashMap<u32, Point2d>,
    pub selected_ids: &'a mut HashSet<u32>,
    pub lasso_path: &'a mut Vec<egui::Pos2>,
    pub is_dragging: bool,
}

impl<'a> ModeContext<'a> {
    /// Convert CDX coordinates to screen coordinates
    pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> egui::Pos2 {
        let scale = self.renderer.zoom * self.renderer.auto_scale;
        egui::Pos2 {
            x: self.renderer.center_offset.x + self.renderer.offset.x + (cdx_pos.x as f32 * scale),
            y: self.renderer.center_offset.y + self.renderer.offset.y - (cdx_pos.y as f32 * scale),
        }
    }
}

pub trait ModeHandler {
    fn handle_click(&mut self, ctx: &mut ModeContext);
    fn handle_drag(&mut self, ctx: &mut ModeContext);
    fn handle_drag_end(&mut self, ctx: &mut ModeContext);
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter);
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool;
}
