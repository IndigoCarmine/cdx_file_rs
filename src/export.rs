use cdx_file_rs::{CdxDocument, CdxNode, CdxObject, Point2d, tags};
use eframe::egui;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tiny_skia::{Pixmap, Transform};

pub struct ExportOptions {
    pub bg_index: usize,
    pub fg_index: usize,
    pub color_table: Vec<egui::Color32>,
    pub margin: f32,
    pub scale: f32, // For PNG resolution
}

pub fn save_as_svg(
    doc: &CdxDocument,
    node_positions: &HashMap<u32, Point2d>,
    path: &std::path::Path,
    options: &ExportOptions,
) -> Result<(), String> {
    let svg_data = generate_svg(doc, node_positions, options);
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(svg_data.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn save_as_png(
    doc: &CdxDocument,
    node_positions: &HashMap<u32, Point2d>,
    path: &std::path::Path,
    options: &ExportOptions,
) -> Result<(), String> {
    let svg_data = generate_svg(doc, node_positions, options);

    let opt = usvg::Options::default();
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    let tree = usvg::Tree::from_str(&svg_data, &opt).map_err(|e| e.to_string())?;

    let size = tree.size();
    let width = (size.width() * options.scale).ceil() as u32;
    let height = (size.height() * options.scale).ceil() as u32;

    let mut pixmap = Pixmap::new(width, height).ok_or("Failed to allocate pixmap".to_string())?;

    // Fill background
    let bg_color = get_export_color(
        &options.color_table,
        options.bg_index,
        options.fg_index,
        options.bg_index as u16,
    );
    let skia_color =
        tiny_skia::Color::from_rgba8(bg_color.r(), bg_color.g(), bg_color.b(), bg_color.a());
    pixmap.fill(skia_color);

    let transform = Transform::from_scale(options.scale, options.scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    pixmap.save_png(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn generate_svg(
    doc: &CdxDocument,
    node_positions: &HashMap<u32, Point2d>,
    options: &ExportOptions,
) -> String {
    let (min_x, min_y, max_x, max_y) = calculate_bounds(node_positions);
    let width = (max_x - min_x).max(1.0) + options.margin * 2.0;
    let height = (max_y - min_y).max(1.0) + options.margin * 2.0;
    let view_box_x = min_x - options.margin;
    let view_box_y = min_y - options.margin;

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="{}" height="{}">"#,
        view_box_x, view_box_y, width, height, width, height
    ));

    // Optional background rect (if alpha > 0)
    let bg_color = get_export_color(&options.color_table, options.bg_index, options.fg_index, 0);
    if bg_color.a() > 0 {
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
            view_box_x,
            view_box_y,
            width,
            height,
            color_to_hex(bg_color)
        ));
    }

    let mut adjacency = HashMap::new();
    for node in &doc.root {
        collect_adjacency_recursive(node, &mut adjacency);
    }

    for node in &doc.root {
        draw_bonds_recursive(&mut svg, node, node_positions, &adjacency, options);
    }

    for node in &doc.root {
        draw_objects_recursive(&mut svg, node, node_positions, options);
    }

    svg.push_str("</svg>");
    svg
}

fn calculate_bounds(node_positions: &HashMap<u32, Point2d>) -> (f32, f32, f32, f32) {
    if node_positions.is_empty() {
        return (0.0, 0.0, 100.0, 100.0);
    }
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    for p in node_positions.values() {
        let x = p.x as f32;
        let y = p.y as f32;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn collect_adjacency_recursive(node: &CdxNode, map: &mut HashMap<u32, Vec<u32>>) {
    if let CdxNode::Object(obj) = node {
        if obj.tag == tags::BOND
            && let Some((b, e)) = obj.get_bond_endpoints() {
                map.entry(b).or_default().push(e);
                map.entry(e).or_default().push(b);
            }
        for child in &obj.children {
            collect_adjacency_recursive(child, map);
        }
    }
}

fn draw_bonds_recursive(
    svg: &mut String,
    node: &CdxNode,
    node_positions: &HashMap<u32, Point2d>,
    adjacency: &HashMap<u32, Vec<u32>>,
    options: &ExportOptions,
) {
    if let CdxNode::Object(obj) = node {
        if obj.tag == tags::BOND
            && let Some((bid, eid)) = obj.get_bond_endpoints()
                && let (Some(bp), Some(ep)) = (node_positions.get(&bid), node_positions.get(&eid)) {
                    let color = get_obj_color(obj, options);
                    let hex = color_to_hex(color);
                    let order = obj.get_bond_order();
                    let stroke_width = 1.5;

                    if (order & 2) != 0 {
                        let dir_x = (ep.x - bp.x) as f32;
                        let dir_y = (ep.y - bp.y) as f32;
                        let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
                        let (ndx, ndy) = if len > 0.001 {
                            (dir_x / len, dir_y / len)
                        } else {
                            (1.0, 0.0)
                        };

                        let perp_x = -ndy * 3.0;
                        let perp_y = ndx * 3.0;

                        let mut double_pos = obj.get_bond_double_position();
                        if double_pos == 0 {
                            let mut net_perp_dir = 0.0;
                            if let Some(neighbors) = adjacency.get(&bid) {
                                for &nid in neighbors {
                                    if nid != eid
                                        && let Some(np) = node_positions.get(&nid) {
                                            let vx = (np.x - bp.x) as f32;
                                            let vy = (np.y - bp.y) as f32;
                                            let vlen = (vx * vx + vy * vy).sqrt();
                                            if vlen > 0.001 {
                                                net_perp_dir +=
                                                    (vx / vlen) * perp_x + (vy / vlen) * perp_y;
                                            }
                                        }
                                }
                            }
                            if let Some(neighbors) = adjacency.get(&eid) {
                                for &nid in neighbors {
                                    if nid != bid
                                        && let Some(np) = node_positions.get(&nid) {
                                            let vx = (np.x - ep.x) as f32;
                                            let vy = (np.y - ep.y) as f32;
                                            let vlen = (vx * vx + vy * vy).sqrt();
                                            if vlen > 0.001 {
                                                net_perp_dir +=
                                                    (vx / vlen) * perp_x + (vy / vlen) * perp_y;
                                            }
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
                            2 => {
                                // Right
                                svg_line(
                                    svg,
                                    bp.x as f32,
                                    bp.y as f32,
                                    ep.x as f32,
                                    ep.y as f32,
                                    &hex,
                                    stroke_width,
                                );
                                let b_inner_x = bp.x as f32 + ndx * 2.0 + perp_x * 0.66;
                                let b_inner_y = bp.y as f32 + ndy * 2.0 + perp_y * 0.66;
                                let e_inner_x = ep.x as f32 - ndx * 2.0 + perp_x * 0.66;
                                let e_inner_y = ep.y as f32 - ndy * 2.0 + perp_y * 0.66;
                                svg_line(
                                    svg,
                                    b_inner_x,
                                    b_inner_y,
                                    e_inner_x,
                                    e_inner_y,
                                    &hex,
                                    stroke_width,
                                );
                            }
                            3 => {
                                // Left
                                svg_line(
                                    svg,
                                    bp.x as f32,
                                    bp.y as f32,
                                    ep.x as f32,
                                    ep.y as f32,
                                    &hex,
                                    stroke_width,
                                );
                                let b_inner_x = bp.x as f32 + ndx * 2.0 - perp_x * 0.66;
                                let b_inner_y = bp.y as f32 + ndy * 2.0 - perp_y * 0.66;
                                let e_inner_x = ep.x as f32 - ndx * 2.0 - perp_x * 0.66;
                                let e_inner_y = ep.y as f32 - ndy * 2.0 - perp_y * 0.66;
                                svg_line(
                                    svg,
                                    b_inner_x,
                                    b_inner_y,
                                    e_inner_x,
                                    e_inner_y,
                                    &hex,
                                    stroke_width,
                                );
                            }
                            _ => {
                                // Center
                                let off_x = perp_x * 0.33;
                                let off_y = perp_y * 0.33;
                                svg_line(
                                    svg,
                                    bp.x as f32 + off_x,
                                    bp.y as f32 + off_y,
                                    ep.x as f32 + off_x,
                                    ep.y as f32 + off_y,
                                    &hex,
                                    stroke_width,
                                );
                                svg_line(
                                    svg,
                                    bp.x as f32 - off_x,
                                    bp.y as f32 - off_y,
                                    ep.x as f32 - off_x,
                                    ep.y as f32 - off_y,
                                    &hex,
                                    stroke_width,
                                );
                            }
                        }
                    } else {
                        svg_line(
                            svg,
                            bp.x as f32,
                            bp.y as f32,
                            ep.x as f32,
                            ep.y as f32,
                            &hex,
                            stroke_width,
                        );
                    }
                }
        for child in &obj.children {
            draw_bonds_recursive(svg, child, node_positions, adjacency, options);
        }
    }
}

fn draw_objects_recursive(
    svg: &mut String,
    node: &CdxNode,
    node_positions: &HashMap<u32, Point2d>,
    options: &ExportOptions,
) {
    if let CdxNode::Object(obj) = node {
        let color = get_obj_color(obj, options);
        let hex = color_to_hex(color);

        match obj.tag {
            tags::NODE => {
                if let Some(pos) = node_positions.get(&obj.id) {
                    let label = if let Some(s) = obj.get_text_styled() {
                        Some(s.text.clone())
                    } else if let Some(elem) = obj.get_element_id() {
                        element_symbol(elem).map(|s| s.to_string())
                    } else {
                        None
                    };

                    if let Some(text) = label {
                        svg_text(svg, pos.x as f32, pos.y as f32, &text, &hex, 16.0);
                    }
                }
            }
            tags::TEXT => {
                if let Some(pos) = obj.get_pos2d()
                    && let Some(s) = obj.get_text_styled() {
                        svg_text(svg, pos.x as f32, pos.y as f32, &s.text, &hex, 14.0);
                    }
            }
            tags::ARROW => {
                if let Some((s, e)) = obj.get_arrow_start_end() {
                    let sx = s.x as f32;
                    let sy = s.y as f32;
                    let ex = e.x as f32;
                    let ey = e.y as f32;
                    let stroke_width = 2.0;
                    svg_line(svg, sx, sy, ex, ey, &hex, stroke_width);
                    // Arrow head logic
                    let dx = ex - sx;
                    let dy = ey - sy;
                    let len = (dx * dx + dy * dy).sqrt();
                    let (ndx, ndy) = if len > 0.001 {
                        (dx / len, dy / len)
                    } else {
                        (1.0, 0.0)
                    };
                    let tip_len = 12.0;
                    let perp_len = 6.0;
                    let perp_x = -ndy * perp_len;
                    let perp_y = ndx * perp_len;
                    let back_x = ex - ndx * tip_len;
                    let back_y = ey - ndy * tip_len;
                    svg_line(
                        svg,
                        ex,
                        ey,
                        back_x + perp_x,
                        back_y + perp_y,
                        &hex,
                        stroke_width,
                    );
                    svg_line(
                        svg,
                        ex,
                        ey,
                        back_x - perp_x,
                        back_y - perp_y,
                        &hex,
                        stroke_width,
                    );
                }
            }
            tags::STOICHIOMETRY_GRID => {
                if let Some(p) = obj.get_pos2d() {
                    let mut cur_x = p.x as f32;
                    let start_y = p.y as f32;
                    for comp in obj.find_objects(tags::SG_COMPONENT) {
                        let mut cur_y_offset = 0.0;
                        for datum in comp.find_objects(tags::SG_DATUM) {
                            let mut text_content = None;
                            for tag_obj in datum.find_objects(tags::OBJECT_TAG) {
                                for text_obj in tag_obj.find_objects(tags::TEXT) {
                                    if let Some(s) = text_obj.get_text_styled() {
                                        let c = get_obj_color(text_obj, options);
                                        text_content = Some((s.text.clone(), c));
                                        break;
                                    }
                                }
                            }
                            if let Some((txt, c)) = text_content {
                                let h = color_to_hex(c);
                                svg.push_str(&format!(
                                     r#"<text x="{}" y="{}" fill="{}" font-family="Arial, sans-serif" font-size="{}" dominant-baseline="hanging">{}</text>"#,
                                     cur_x, start_y + cur_y_offset, h, 12.0, escape_xml(&txt)
                                 ));
                                cur_y_offset += 15.0;
                            }
                        }
                        cur_x += 100.0;
                    }
                }
            }
            _ => {
                for child in &obj.children {
                    draw_objects_recursive(svg, child, node_positions, options);
                }
            }
        }
    }
}

fn svg_line(svg: &mut String, x1: f32, y1: f32, x2: f32, y2: f32, color: &str, width: f32) {
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}" stroke-linecap="round" />"#,
        x1, y1, x2, y2, color, width
    ));
}

fn svg_text(svg: &mut String, x: f32, y: f32, text: &str, color: &str, size: f32) {
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" fill="{}" font-family="Arial, sans-serif" font-size="{}" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
        x, y, color, size, escape_xml(text)
    ));
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}

fn get_obj_color(obj: &CdxObject, options: &ExportOptions) -> egui::Color32 {
    let index = obj.get_color_index().unwrap_or(1);
    get_export_color(
        &options.color_table,
        options.bg_index,
        options.fg_index,
        index as u16,
    )
}

fn get_export_color(
    table: &[egui::Color32],
    bg_index: usize,
    fg_index: usize,
    index: u16,
) -> egui::Color32 {
    if index == 0 {
        return table.get(bg_index).cloned().unwrap_or(egui::Color32::WHITE);
    }
    if index == 1 {
        return table.get(fg_index).cloned().unwrap_or(egui::Color32::BLACK);
    }
    table
        .get(index as usize)
        .cloned()
        .unwrap_or(egui::Color32::WHITE)
}

fn color_to_hex(c: egui::Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

fn element_symbol(elem: i16) -> Option<&'static str> {
    match elem {
        1 => Some("H"),
        2 => Some("He"),
        3 => Some("Li"),
        4 => Some("Be"),
        5 => Some("B"),
        6 => Some("C"),
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
