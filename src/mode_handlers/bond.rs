use crate::modes::{Hit, ModeContext, ModeHandler};
use cdx_file_rs::{CdxNode, CdxValue, Point2d, tags};
use eframe::egui;
use std::collections::HashMap;

pub struct BondMode;

impl ModeHandler for BondMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        if let Some(hit) = ctx.hovered_object {
            if hit.tag == tags::NODE {
                add_bond_from_node(ctx, hit.id);
            } else if hit.tag == tags::BOND
                && let Some(bond_obj) = ctx.doc.find_object_mut(hit.id)
            {
                let current_order = bond_obj.get_bond_order();
                let new_order = if current_order == 1 { 2 } else { 1 };
                bond_obj.set_property(tags::BOND_ORDER, CdxValue::Int16(new_order));
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
                    12.0 * (ctx.renderer.zoom / 5.0).max(0.5),
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                );
            }
        }
    }
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool {
        if let Some(Hit {
            tag: tags::NODE,
            id,
        }) = ctx.hovered_object
        {
            match key {
                egui::Key::Num1 => {
                    add_fragment(ctx, id, "Et");
                    true
                }
                egui::Key::Num2 => {
                    add_fragment(ctx, id, "COMe");
                    true
                }
                egui::Key::Num3 => {
                    add_fragment(ctx, id, "Ph");
                    true
                }
                egui::Key::N => {
                    if let Some(obj) = ctx.doc.find_object_mut(id) {
                        obj.set_property(tags::ELEMENT, CdxValue::Int16(7)); // Nitrogen
                        true
                    } else {
                        false
                    }
                }
                egui::Key::Backspace | egui::Key::Delete => {
                    ctx.doc.delete_object(id);
                    ctx.node_positions.remove(&id);
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

fn add_bond_from_node(ctx: &mut ModeContext, node_id: u32) {
    let mut angles = Vec::new();
    for node in &ctx.doc.root {
        find_angles_recursive(node, node_id, ctx.node_positions, &mut angles);
    }
    let angle = if let Some(last_angle) = angles.last() {
        *last_angle + ctx.config.bond.default_angle_deg.to_radians()
    } else {
        30.0f64.to_radians()
    };
    let bond_length = ctx.config.bond.default_length;
    let base_pos = ctx.node_positions.get(&node_id).cloned().unwrap();
    let new_pos = Point2d {
        x: base_pos.x + angle.cos() * bond_length,
        y: base_pos.y + angle.sin() * bond_length,
    };
    let mut next_id = ctx.doc.max_id() + 1;
    let new_node_id = next_id;
    next_id += 1;
    let new_bond_id = next_id;
    let mut new_node_obj = cdx_file_rs::CdxObject {
        tag: tags::NODE,
        id: new_node_id,
        children: vec![],
    };
    new_node_obj.set_property(tags::POSITION, CdxValue::Point2d(new_pos.clone()));
    let mut new_bond_obj = cdx_file_rs::CdxObject {
        tag: tags::BOND,
        id: new_bond_id,
        children: vec![],
    };
    new_bond_obj.set_property(tags::BOND_BEGIN, CdxValue::Uint32(node_id));
    new_bond_obj.set_property(tags::BOND_END, CdxValue::Uint32(new_node_id));
    new_bond_obj.set_property(tags::BOND_ORDER, CdxValue::Int16(1));
    ctx.doc
        .add_to_parent_of(node_id, CdxNode::Object(new_node_obj));
    ctx.doc
        .add_to_parent_of(node_id, CdxNode::Object(new_bond_obj));
    ctx.node_positions.insert(new_node_id, new_pos);
}

fn add_fragment(ctx: &mut ModeContext, node_id: u32, label: &str) {
    if let Some(obj) = ctx.doc.find_object_mut(node_id) {
        obj.set_property(
            tags::TEXT_STRING,
            CdxValue::String(cdx_file_rs::StyledString {
                text: label.to_string(),
                runs: vec![],
            }),
        );
        obj.set_property(tags::ELEMENT, CdxValue::Int16(0)); // Generic label
    }
}

fn find_angles_recursive(
    node: &CdxNode,
    center_id: u32,
    positions: &HashMap<u32, Point2d>,
    angles: &mut Vec<f64>,
) {
    if let CdxNode::Object(obj) = node {
        if obj.tag == tags::BOND
            && let Some((bid, eid)) = obj.get_bond_endpoints()
        {
            let other_id = if bid == center_id {
                Some(eid)
            } else if eid == center_id {
                Some(bid)
            } else {
                None
            };
            if let Some(oid) = other_id
                && let (Some(p1), Some(p2)) = (positions.get(&center_id), positions.get(&oid))
            {
                angles.push((p2.y - p1.y).atan2(p2.x - p1.x));
            }
        }
        for child in &obj.children {
            find_angles_recursive(child, center_id, positions, angles);
        }
    }
}
