use crate::cdx::file::CdxFile;
use crate::cdx::values::Point2d;
use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub tag: u16,
    pub id: u32,
}

/// Bond position data: (begin_node_id, end_node_id, begin_pos, end_pos)
pub type BondPosition = (u32, u32, Point2d, Point2d);

pub struct ModeContext<'a> {
    pub mouse_pos: egui::Pos2,
    pub ui: &'a egui::Ui,
    pub drag_delta: egui::Vec2,
    pub view_offset: &'a mut egui::Vec2,
    pub zoom: f32,
    pub auto_scale: f32,
    pub center_offset: egui::Vec2,
    pub offset: egui::Vec2,
    pub node_positions: &'a HashMap<u32, Point2d>,
    pub bond_positions: &'a HashMap<u32, BondPosition>,
    pub selected_ids: &'a mut HashSet<u32>,
    pub lasso_path: &'a mut Vec<egui::Pos2>,
    pub is_dragging: bool,
    pub clipboard: &'a mut Option<CdxFile>,
    pub cdx_file: &'a RefCell<Option<CdxFile>>,
}

impl<'a> ModeContext<'a> {
    /// Convert CDX coordinates to screen coordinates
    pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> egui::Pos2 {
        let scale = self.zoom * self.auto_scale;
        egui::Pos2 {
            x: self.center_offset.x + self.offset.x + (cdx_pos.x as f32 * scale),
            y: self.center_offset.y + self.offset.y + (cdx_pos.y as f32 * scale), // CDX Y increases downward (same as screen)
        }
    }

    /// Convert screen coordinates to CDX coordinates
    pub fn screen_to_cdx(&self, screen_pos: egui::Pos2) -> Point2d {
        let scale = self.zoom * self.auto_scale;
        Point2d {
            x: ((screen_pos.x - self.center_offset.x - self.offset.x) / scale) as f64,
            y: ((screen_pos.y - self.center_offset.y - self.offset.y) / scale) as f64, // CDX Y increases downward (same as screen)
        }
    }

    /// Find node at screen position within hit radius
    pub fn hit_test_node(&self, screen_pos: egui::Pos2, radius: f32) -> Option<u32> {
        let hit_radius = radius * self.zoom;
        for (node_id, cdx_pos) in self.node_positions.iter() {
            let node_screen = self.cdx_to_screen(cdx_pos);
            if node_screen.distance(screen_pos) <= hit_radius {
                return Some(*node_id);
            }
        }
        None
    }

    /// Find bond at screen position (within distance from line segment)
    pub fn hit_test_bond(&self, screen_pos: egui::Pos2, threshold: f32) -> Option<u32> {
        let hit_threshold = threshold * self.zoom;
        for (bond_id, (_begin_id, _end_id, begin_pos, end_pos)) in self.bond_positions.iter() {
            let begin_screen = self.cdx_to_screen(begin_pos);
            let end_screen = self.cdx_to_screen(end_pos);
            let dist = point_to_segment_distance(screen_pos, begin_screen, end_screen);
            if dist <= hit_threshold {
                return Some(*bond_id);
            }
        }
        None
    }
}

/// Calculate distance from point to line segment
fn point_to_segment_distance(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let ab_len_sq = ab.length_sq();
    
    if ab_len_sq == 0.0 {
        return ap.length();
    }
    
    let t = (ap.x * ab.x + ap.y * ab.y) / ab_len_sq;
    let t = t.clamp(0.0, 1.0);
    
    let closest = egui::Pos2::new(a.x + t * ab.x, a.y + t * ab.y);
    (p - closest).length()
}

pub trait ModeHandler {
    fn handle_click(&mut self, ctx: &mut ModeContext);
    fn handle_drag(&mut self, ctx: &mut ModeContext);
    fn handle_drag_end(&mut self, ctx: &mut ModeContext);
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter);
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool;
}
