use crate::render::CdxRenderer;
use cdx_file_rs::{CdxDocument, Point2d};
use eframe::egui;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub tag: u16,
    pub id: u32,
}

pub struct ModeContext<'a> {
    pub doc: &'a mut CdxDocument,
    pub node_positions: &'a mut HashMap<u32, Point2d>,
    pub renderer: &'a CdxRenderer<'a>,
    pub mouse_pos: egui::Pos2,
    pub hovered_object: Option<Hit>,
    pub ui: &'a egui::Ui,
    pub drag_delta: egui::Vec2,

    pub view_offset: &'a mut egui::Vec2,
    pub selected_ids: &'a mut HashSet<u32>,
    pub lasso_path: &'a mut Vec<egui::Pos2>,
    pub clipboard: &'a mut Vec<cdx_file_rs::CdxObject>,
    pub config: &'a crate::config::AppConfig,
}

pub trait ModeHandler {
    fn handle_click(&mut self, ctx: &mut ModeContext);
    fn handle_drag(&mut self, ctx: &mut ModeContext);
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter);
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool;
}
