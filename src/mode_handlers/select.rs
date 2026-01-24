use crate::modes::{ModeContext, ModeHandler};
use cdx_file_rs::{CdxNode, CdxValue, Point2d, tags};
use eframe::egui;

pub struct SelectMode;

impl ModeHandler for SelectMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        if let Some(hit) = ctx.hovered_object {
            if !ctx.ui.input(|i| i.modifiers.shift) {
                ctx.selected_ids.clear();
            }
            ctx.selected_ids.insert(hit.id);
        } else {
            ctx.selected_ids.clear();
        }
    }

    fn handle_drag(&mut self, ctx: &mut ModeContext) {
        if ctx.hovered_object.is_some() || !ctx.selected_ids.is_empty() {
            let zoom = ctx.renderer.zoom;
            let delta_world = ctx.drag_delta / zoom;
            for id in ctx.selected_ids.iter() {
                if let Some(pos) = ctx.node_positions.get_mut(id) {
                    pos.x += delta_world.x as f64;
                    pos.y += delta_world.y as f64;
                    if let Some(obj) = ctx.doc.find_object_mut(*id) {
                        obj.set_property(tags::POSITION, CdxValue::Point2d(pos.clone()));
                    }
                }
            }
        } else {
            ctx.lasso_path.push(ctx.mouse_pos);
        }
    }

    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        let center = ctx.ui.available_rect_before_wrap().center() + ctx.renderer.offset;
        for id in ctx.selected_ids.iter() {
            if let Some(pos) = ctx.node_positions.get(id) {
                let p = ctx.renderer.to_screen(pos, center);
                painter.circle_stroke(
                    p,
                    14.0 * (ctx.renderer.zoom / 5.0).max(0.5),
                    egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 255)),
                );
            }
        }
        if let Some(hit) = ctx.hovered_object
            && let Some(pos) = ctx.node_positions.get(&hit.id)
        {
            let p = ctx.renderer.to_screen(pos, center);
            painter.circle_stroke(
                p,
                12.0 * (ctx.renderer.zoom / 5.0).max(0.5),
                egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
            );
        }
        if ctx.lasso_path.len() > 1 {
            let stroke = egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(100, 100, 255, 128),
            );
            for i in 0..ctx.lasso_path.len() - 1 {
                painter.line_segment([ctx.lasso_path[i], ctx.lasso_path[i + 1]], stroke);
            }
            if ctx.lasso_path.len() > 2 {
                painter.line_segment(
                    [ctx.lasso_path[ctx.lasso_path.len() - 1], ctx.lasso_path[0]],
                    stroke,
                );
            }
        }
    }

    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool {
        let modifiers = ctx.ui.input(|i| i.modifiers);

        if modifiers.command && key == egui::Key::C {
            ctx.clipboard.clear();
            for id in ctx.selected_ids.iter() {
                if let Some(obj) = ctx.doc.find_object(*id) {
                    ctx.clipboard.push(obj.clone());
                }
            }
            return true;
        }

        if modifiers.command && key == egui::Key::V {
            if ctx.clipboard.is_empty() {
                return false;
            }
            ctx.selected_ids.clear();
            let mut next_id = ctx.doc.max_id() + 1;
            let center = ctx.ui.available_rect_before_wrap().center() + ctx.renderer.offset;
            let mouse_world = Point2d {
                x: ((ctx.mouse_pos.x - center.x) / ctx.renderer.zoom) as f64,
                y: ((ctx.mouse_pos.y - center.y) / ctx.renderer.zoom) as f64,
            };
            for old_obj in ctx.clipboard.iter() {
                let mut new_obj = old_obj.clone();
                new_obj.id = next_id;
                next_id += 1;
                if let Some(pos) = new_obj.get_pos2d_mut() {
                    *pos = mouse_world.clone();
                }
                let nid = new_obj.id;
                if let Some(pos) = new_obj.get_pos2d() {
                    ctx.node_positions.insert(nid, pos.clone());
                }
                ctx.doc.root.push(CdxNode::Object(new_obj));
                ctx.selected_ids.insert(nid);
            }
            return true;
        }

        if key == egui::Key::C && !ctx.selected_ids.is_empty() {
            self.cleanup_structure(ctx);
            return true;
        }

        // Handle number keys (0-9) to assign color table index
        if !ctx.selected_ids.is_empty() {
            let color_index = match key {
                egui::Key::Num0 => Some(0),
                egui::Key::Num1 => Some(1),
                egui::Key::Num2 => Some(2),
                egui::Key::Num3 => Some(3),
                egui::Key::Num4 => Some(4),
                egui::Key::Num5 => Some(5),
                egui::Key::Num6 => Some(6),
                egui::Key::Num7 => Some(7),
                egui::Key::Num8 => Some(8),
                egui::Key::Num9 => Some(9),
                _ => None,
            };

            if let Some(idx) = color_index {
                for id in ctx.selected_ids.iter() {
                    if let Some(obj) = ctx.doc.find_object_mut(*id) {
                        obj.set_property(tags::BG_COLOR, CdxValue::Int16(idx as i16));
                    }
                }
                return true;
            }
        }

        match key {
            egui::Key::Backspace | egui::Key::Delete => {
                let ids: Vec<u32> = ctx.selected_ids.iter().cloned().collect();
                for id in ids {
                    ctx.doc.delete_object(id);
                    ctx.node_positions.remove(&id);
                }
                ctx.selected_ids.clear();
                true
            }
            _ => false,
        }
    }
}

impl SelectMode {
    fn cleanup_structure(&self, ctx: &mut ModeContext) {
        use std::collections::HashMap;
        let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut all_bonds = Vec::new();
        self.collect_bonds(&ctx.doc.root, &mut all_bonds);
        for (id1, id2) in all_bonds {
            adj.entry(id1).or_default().push(id2);
            adj.entry(id2).or_default().push(id1);
        }

        let bond_length = 14.4;
        let mut pos_map = ctx.node_positions.clone();

        let deg30 = 30.0f64.to_radians();
        let deg120 = 120.0f64.to_radians();

        for _ in 0..15 {
            let mut next_pos_map = pos_map.clone();
            for &id in ctx.selected_ids.iter() {
                let p_i = pos_map.get(&id).unwrap().clone();
                let neighbors = match adj.get(&id) {
                    Some(n) => n,
                    None => continue,
                };
                if neighbors.is_empty() {
                    continue;
                }

                let mut n_info: Vec<(f64, Point2d)> = neighbors
                    .iter()
                    .map(|&n_id| {
                        let p_n = pos_map.get(&n_id).unwrap().clone();
                        ((p_n.y - p_i.y).atan2(p_n.x - p_i.x), p_n)
                    })
                    .collect();

                let mut ideal_p_i = Point2d { x: 0.0, y: 0.0 };

                if n_info.len() == 1 {
                    let (ang, p_n) = &n_info[0];
                    let snapped_ang = (ang / deg30).round() * deg30;
                    ideal_p_i.x = p_n.x - snapped_ang.cos() * bond_length;
                    ideal_p_i.y = p_n.y - snapped_ang.sin() * bond_length;
                } else if n_info.len() == 2 {
                    n_info.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    let a1 = n_info[0].0;
                    let a2 = n_info[1].0;
                    let mut diff = a2 - a1;
                    let mut wrapped = false;
                    if diff > std::f64::consts::PI {
                        diff = 2.0 * std::f64::consts::PI - diff;
                        wrapped = true;
                    }

                    let target_diff = if diff > 150.0f64.to_radians() {
                        std::f64::consts::PI
                    } else {
                        deg120
                    };

                    let avg_ang = if wrapped {
                        (n_info[1].0 + n_info[0].0 + 2.0 * std::f64::consts::PI) / 2.0
                    } else {
                        (n_info[1].0 + n_info[0].0) / 2.0
                    };

                    let half = target_diff / 2.0;
                    let (ta1, ta2) = if wrapped {
                        (avg_ang + half, avg_ang - half)
                    } else {
                        (avg_ang - half, avg_ang + half)
                    };

                    let p1 = &n_info[0].1;
                    let p2 = &n_info[1].1;

                    let p_i_from_1 = Point2d {
                        x: p1.x - ta1.cos() * bond_length,
                        y: p1.y - ta1.sin() * bond_length,
                    };
                    let p_i_from_2 = Point2d {
                        x: p2.x - ta2.cos() * bond_length,
                        y: p2.y - ta2.sin() * bond_length,
                    };

                    ideal_p_i.x = (p_i_from_1.x + p_i_from_2.x) / 2.0;
                    ideal_p_i.y = (p_i_from_1.y + p_i_from_2.y) / 2.0;
                } else if n_info.len() == 3 {
                    n_info.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    let mut best_phi = 0.0;
                    let mut min_err = f64::MAX;
                    for step in 0..12 {
                        let phi = (step as f64 * 30.0).to_radians();
                        let mut err = 0.0;
                        for (i, item) in n_info.iter().enumerate() {
                            let mut d =
                                (item.0 - (phi + i as f64 * deg120)) % (2.0 * std::f64::consts::PI);
                            if d > std::f64::consts::PI {
                                d -= 2.0 * std::f64::consts::PI;
                            }
                            if d < -std::f64::consts::PI {
                                d += 2.0 * std::f64::consts::PI;
                            }
                            err += d * d;
                        }
                        if err < min_err {
                            min_err = err;
                            best_phi = phi;
                        }
                    }

                    let mut sum_x = 0.0;
                    let mut sum_y = 0.0;
                    for (i, item) in n_info.iter().enumerate() {
                        let target_ang = best_phi + i as f64 * deg120;
                        let p_n = &item.1;
                        sum_x += p_n.x - target_ang.cos() * bond_length;
                        sum_y += p_n.y - target_ang.sin() * bond_length;
                    }
                    ideal_p_i.x = sum_x / 3.0;
                    ideal_p_i.y = sum_y / 3.0;
                } else {
                    let mut sum_x = 0.0;
                    let mut sum_y = 0.0;
                    for (_, p_n) in &n_info {
                        let dx = p_i.x - p_n.x;
                        let dy = p_i.y - p_n.y;
                        let d = (dx * dx + dy * dy).sqrt();
                        if d > 0.0 {
                            sum_x += p_n.x + dx / d * bond_length;
                            sum_y += p_n.y + dy / d * bond_length;
                        }
                    }
                    ideal_p_i.x = sum_x / n_info.len() as f64;
                    ideal_p_i.y = sum_y / n_info.len() as f64;
                }

                next_pos_map.insert(
                    id,
                    Point2d {
                        x: p_i.x * 0.4 + ideal_p_i.x * 0.6,
                        y: p_i.y * 0.4 + ideal_p_i.y * 0.6,
                    },
                );
            }
            pos_map = next_pos_map;
        }

        for (id, pos) in pos_map {
            if ctx.selected_ids.contains(&id) {
                ctx.node_positions.insert(id, pos.clone());
                if let Some(obj) = ctx.doc.find_object_mut(id) {
                    obj.set_property(tags::POSITION, CdxValue::Point2d(pos));
                }
            }
        }
    }

    fn collect_bonds(&self, nodes: &[CdxNode], bonds: &mut Vec<(u32, u32)>) {
        for node in nodes {
            if let CdxNode::Object(obj) = node {
                if obj.tag == tags::BOND
                    && let Some(endpoints) = obj.get_bond_endpoints()
                {
                    bonds.push(endpoints);
                }
                self.collect_bonds(&obj.children, bonds);
            }
        }
    }
}
