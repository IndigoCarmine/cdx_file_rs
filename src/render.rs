use cdx_file_rs::{CdxDocument, CdxNode, Point2d, tags};
use eframe::egui;
use std::collections::HashMap;

pub struct CdxRenderer<'a> {
    pub zoom: f32,
    pub offset: egui::Vec2,
    pub color_table: &'a [egui::Color32],
    pub bg_index: usize,
    pub fg_index: usize,
}

impl<'a> CdxRenderer<'a> {
    pub fn render_all(
        &self,
        ui: &egui::Ui,
        painter: &egui::Painter,
        doc: &CdxDocument,
        node_positions: &HashMap<u32, Point2d>,
    ) {
        let center = ui.available_rect_before_wrap().center() + self.offset;

        // Build adjacency map for better bond rendering (e.g. double bond side)
        let mut adjacency = HashMap::new();
        for node in &doc.root {
            self.collect_adjacency_recursive(node, &mut adjacency);
        }

        // Draw Bonds
        for node in &doc.root {
            self.draw_bonds_recursive(painter, node, center, node_positions, &adjacency);
        }

        // Draw Nodes and other objects
        for node in &doc.root {
            self.draw_objects_recursive(painter, node, center, node_positions);
        }
    }

    fn collect_adjacency_recursive(&self, node: &CdxNode, map: &mut HashMap<u32, Vec<u32>>) {
        if let CdxNode::Object(obj) = node {
            if obj.tag == tags::BOND
                && let Some((b, e)) = obj.get_bond_endpoints() {
                    map.entry(b).or_default().push(e);
                    map.entry(e).or_default().push(b);
                }
            for child in &obj.children {
                self.collect_adjacency_recursive(child, map);
            }
        }
    }

    fn draw_objects_recursive(
        &self,
        painter: &egui::Painter,
        node: &CdxNode,
        origin: egui::Pos2,
        node_positions: &HashMap<u32, Point2d>,
    ) {
        if let CdxNode::Object(obj) = node {
            match obj.tag {
                tags::NODE => {
                    if let Some(pos) = node_positions.get(&obj.id) {
                        let screen_pos = self.to_screen(pos, origin);

                        let label = if let Some(s) = obj.get_text_styled() {
                            Some(s.text.clone())
                        } else if let Some(elem) = obj.get_element_id() {
                            Self::element_symbol(elem).map(|s| s.to_string())
                        } else {
                            None
                        };

                        if let Some(text) = label {
                            let color = self.get_color(obj);
                            painter.text(
                                screen_pos,
                                egui::Align2::CENTER_CENTER,
                                text,
                                egui::FontId::proportional(16.0 * (self.zoom / 5.0).max(0.5)),
                                color,
                            );
                        }
                    }
                }
                tags::BOND => { /* Handled separately */ }
                tags::TEXT => {
                    if let Some(pos) = obj.get_pos2d()
                        && let Some(s) = obj.get_text_styled() {
                            let screen_pos = self.to_screen(pos, origin);
                            let color = self.get_color(obj);
                            painter.text(
                                screen_pos,
                                egui::Align2::CENTER_CENTER,
                                &s.text,
                                egui::FontId::proportional(14.0 * (self.zoom / 5.0).max(0.5)),
                                if color == egui::Color32::TRANSPARENT {
                                    egui::Color32::LIGHT_BLUE
                                } else {
                                    color
                                },
                            );
                        }
                }
                tags::ARROW => {
                    if let Some((s, e)) = obj.get_arrow_start_end() {
                        let start = self.to_screen(&s, origin);
                        let end = self.to_screen(&e, origin);
                        let color = self.get_color(obj);
                        let stroke = egui::Stroke::new(
                            2.0 * (self.zoom / 5.0).max(0.5),
                            if color == egui::Color32::BLACK && self.bg_index == 1 {
                                egui::Color32::YELLOW
                            } else {
                                color
                            },
                        );
                        painter.line_segment([start, end], stroke);
                        let dir = (end - start).normalized();
                        let perp = egui::vec2(-dir.y, dir.x) * 6.0 * (self.zoom / 5.0).max(0.5);
                        let tip_len = 12.0 * (self.zoom / 5.0).max(0.5);
                        painter.line_segment([end, end - dir * tip_len + perp], stroke);
                        painter.line_segment([end, end - dir * tip_len - perp], stroke);
                    }
                }
                tags::STOICHIOMETRY_GRID => {
                    if let Some(p) = obj.get_pos2d() {
                        let top_left = self.to_screen(p, origin);
                        let mut current_pos = top_left;
                        for comp in obj.find_objects(tags::SG_COMPONENT) {
                            let mut y_offset = 0.0;
                            for datum in comp.find_objects(tags::SG_DATUM) {
                                if let Some((text, color)) = self.find_sg_info(datum) {
                                    painter.text(
                                        current_pos + egui::vec2(0.0, y_offset),
                                        egui::Align2::LEFT_TOP,
                                        text,
                                        egui::FontId::proportional(
                                            12.0 * (self.zoom / 5.0).max(0.5),
                                        ),
                                        color,
                                    );
                                    y_offset += 15.0 * (self.zoom / 5.0).max(0.5);
                                }
                            }
                            current_pos.x += 100.0 * (self.zoom / 5.0).max(0.5);
                        }
                    }
                }
                _ => {
                    for child in &obj.children {
                        self.draw_objects_recursive(painter, child, origin, node_positions);
                    }
                }
            }
        }
    }

    pub fn find_sg_info(&self, obj: &cdx_file_rs::CdxObject) -> Option<(String, egui::Color32)> {
        for tag_obj in obj.find_objects(tags::OBJECT_TAG) {
            for text_obj in tag_obj.find_objects(tags::TEXT) {
                if let Some(s) = text_obj.get_text_styled() {
                    let mut color = self.get_color(text_obj);
                    if color == egui::Color32::WHITE && self.bg_index == 1 {
                        color = self
                            .color_table
                            .get(self.fg_index)
                            .cloned()
                            .unwrap_or(egui::Color32::BLACK);
                    }
                    return Some((s.text.clone(), color));
                }
            }
        }
        None
    }

    fn draw_bonds_recursive(
        &self,
        painter: &egui::Painter,
        node: &CdxNode,
        origin: egui::Pos2,
        node_positions: &HashMap<u32, Point2d>,
        adjacency: &HashMap<u32, Vec<u32>>,
    ) {
        if let CdxNode::Object(obj) = node {
            if obj.tag == tags::BOND
                && let Some((bid, eid)) = obj.get_bond_endpoints()
                    && let (Some(bp), Some(ep)) =
                        (node_positions.get(&bid), node_positions.get(&eid))
                    {
                        let b_screen = self.to_screen(bp, origin);
                        let e_screen = self.to_screen(ep, origin);
                        let color = self.get_color(obj);
                        let order = obj.get_bond_order();
                        let stroke = egui::Stroke::new(1.5 * (self.zoom / 5.0).max(0.5), color);

                        if (order & 2) != 0 {
                            let dir = (e_screen - b_screen).normalized();
                            let perp = egui::vec2(-dir.y, dir.x) * 3.0 * (self.zoom / 5.0).max(0.5);
                            let mut double_pos = obj.get_bond_double_position();

                            if double_pos == 0 {
                                // DoublePosition Auto
                                // Calculate side based on adjacent bonds
                                let mut net_perp_dir = 0.0;
                                if let Some(neighbors) = adjacency.get(&bid) {
                                    for &nid in neighbors {
                                        if nid != eid
                                            && let Some(np) = node_positions.get(&nid) {
                                                let v = egui::vec2(
                                                    (np.x - bp.x) as f32,
                                                    (np.y - bp.y) as f32,
                                                )
                                                .normalized();
                                                net_perp_dir += v.dot(perp);
                                            }
                                    }
                                }
                                if let Some(neighbors) = adjacency.get(&eid) {
                                    for &nid in neighbors {
                                        if nid != bid
                                            && let Some(np) = node_positions.get(&nid) {
                                                let v = egui::vec2(
                                                    (np.x - ep.x) as f32,
                                                    (np.y - ep.y) as f32,
                                                )
                                                .normalized();
                                                net_perp_dir += v.dot(perp);
                                            }
                                    }
                                }
                                double_pos = if net_perp_dir > 0.1 {
                                    2
                                } else if net_perp_dir < -0.1 {
                                    3
                                } else {
                                    1
                                };
                            }

                            match double_pos {
                                1 => {
                                    // Center
                                    painter
                                        .line_segment([b_screen + perp, e_screen + perp], stroke);
                                    painter
                                        .line_segment([b_screen - perp, e_screen - perp], stroke);
                                }
                                2 => {
                                    // Right
                                    painter.line_segment([b_screen, e_screen], stroke);
                                    let b_inner = b_screen + dir * 2.0 + perp * 2.0;
                                    let e_inner = e_screen - dir * 2.0 + perp * 2.0;
                                    painter.line_segment([b_inner, e_inner], stroke);
                                }
                                3 => {
                                    // Left
                                    painter.line_segment([b_screen, e_screen], stroke);
                                    let b_inner = b_screen + dir * 2.0 - perp * 2.0;
                                    let e_inner = e_screen - dir * 2.0 - perp * 2.0;
                                    painter.line_segment([b_inner, e_inner], stroke);
                                }
                                _ => {
                                    painter
                                        .line_segment([b_screen + perp, e_screen + perp], stroke);
                                    painter
                                        .line_segment([b_screen - perp, e_screen - perp], stroke);
                                }
                            }
                        } else {
                            painter.line_segment([b_screen, e_screen], stroke);
                        }
                    }
            for child in &obj.children {
                self.draw_bonds_recursive(painter, child, origin, node_positions, adjacency);
            }
        }
    }

    pub fn to_screen(&self, p: &Point2d, origin: egui::Pos2) -> egui::Pos2 {
        egui::pos2(
            origin.x + (p.x as f32 * self.zoom),
            origin.y + (p.y as f32 * self.zoom),
        )
    }

    fn get_color(&self, obj: &cdx_file_rs::CdxObject) -> egui::Color32 {
        let index = obj.get_color_index().unwrap_or(1);
        if index == 0 {
            return self
                .color_table
                .get(self.bg_index)
                .cloned()
                .unwrap_or(egui::Color32::WHITE);
        }
        if index == 1 {
            return self
                .color_table
                .get(self.fg_index)
                .cloned()
                .unwrap_or(egui::Color32::BLACK);
        }
        self.color_table
            .get(index)
            .cloned()
            .unwrap_or(egui::Color32::WHITE)
    }

    fn element_symbol(elem: i16) -> Option<&'static str> {
        match elem {
            1 => Some("H"),
            2 => Some("He"),
            3 => Some("Li"),
            4 => Some("Be"),
            5 => Some("B"),
            6 => None, // Carbon
            7 => Some("N"),
            8 => Some("O"),
            9 => Some("F"),
            10 => Some("Ne"),
            11 => Some("Na"),
            12 => Some("Mg"),
            13 => Some("Al"),
            14 => Some("Si"),
            15 => Some("P"),
            16 => Some("S"),
            17 => Some("Cl"),
            19 => Some("K"),
            20 => Some("Ca"),
            26 => Some("Fe"),
            29 => Some("Cu"),
            30 => Some("Zn"),
            35 => Some("Br"),
            53 => Some("I"),
            _ => None,
        }
    }
}
