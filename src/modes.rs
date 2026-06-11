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
    /// Convert CDX coordinates to screen coordinates.
    /// cdx_pos stores raw CDX fixed-point values (i32 as f64, must divide by 65536 to get CDX pts).
    pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> egui::Pos2 {
        let scale = self.zoom * self.auto_scale;
        // Convert raw fixed-point to CDX pts before applying scale
        let cdx_pts_x = cdx_pos.x / 65536.0;
        let cdx_pts_y = cdx_pos.y / 65536.0;
        egui::Pos2 {
            x: self.center_offset.x + self.offset.x + (cdx_pts_x as f32 * scale),
            y: self.center_offset.y + self.offset.y + (cdx_pts_y as f32 * scale),
        }
    }

    /// Convert screen coordinates to CDX raw fixed-point coordinates.
    /// Returns raw CDX fixed-point values (CDX pts * 65536) for storage in position_2d.
    pub fn screen_to_cdx(&self, screen_pos: egui::Pos2) -> Point2d {
        let scale = self.zoom * self.auto_scale;
        // (screen - origin) / scale → CDX pts; * 65536 → raw fixed-point for position_2d storage
        let cdx_pts_x = (screen_pos.x - self.center_offset.x - self.offset.x) / scale;
        let cdx_pts_y = (screen_pos.y - self.center_offset.y - self.offset.y) / scale;
        Point2d {
            x: (cdx_pts_x as f64) * 65536.0,
            y: (cdx_pts_y as f64) * 65536.0,
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
