mod config;
mod export;
mod mode_handlers;
mod modes;
mod render;

use crate::mode_handlers::{BondMode, EraserMode, SelectMode, ViewMode};
use crate::modes::{Hit, ModeContext, ModeHandler};
use crate::render::CdxRenderer;
use cdx_file_rs::{CdxDocument, CdxNode, CdxParser, CdxValue, Point2d, tags};
use eframe::egui;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let initial_file = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CDX Viewer",
        options,
        Box::new(|_cc| {
            let mut app = CdxApp::default();

            // Load config
            if let Ok(content) = std::fs::read_to_string("config.toml") {
                if let Ok(cfg) = toml::from_str(&content) {
                    app.config = cfg;
                } else {
                    eprintln!("Failed to parse config.toml");
                }
            }

            if let Some(path) = initial_file {
                app.load_file(&path);
            }
            Ok(Box::new(app))
        }),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AppMode {
    Select,
    Bond,
    Eraser,
    View,
}

struct CdxApp {
    doc: Option<CdxDocument>,
    node_positions: HashMap<u32, Point2d>,
    error: Option<String>,
    zoom: f32,
    offset: egui::Vec2,
    color_table: Vec<egui::Color32>,
    bg_index: usize,
    fg_index: usize,
    mode: AppMode,
    selected_ids: std::collections::HashSet<u32>,
    lasso_path: Vec<egui::Pos2>,
    clipboard: Vec<cdx_file_rs::CdxObject>,
    config: crate::config::AppConfig,
}

impl Default for CdxApp {
    fn default() -> Self {
        Self {
            doc: None,
            node_positions: HashMap::new(),
            error: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            color_table: vec![egui::Color32::BLACK, egui::Color32::WHITE],
            bg_index: 1,
            fg_index: 0,
            mode: AppMode::Select,
            selected_ids: std::collections::HashSet::new(),
            lasso_path: Vec::new(),
            clipboard: Vec::new(),
            config: crate::config::AppConfig::default(),
        }
    }
}

impl CdxApp {
    fn load_file(&mut self, path: &str) {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut parser = CdxParser::new(reader);
                match parser.parse() {
                    Ok(doc) => {
                        self.node_positions.clear();
                        self.extract_positions(&doc);
                        self.extract_colors(&doc);
                        self.doc = Some(doc);
                        self.error = None;
                        self.reset_view();
                    }
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
            Err(e) => self.error = Some(e.to_string()),
        }
    }

    fn extract_colors(&mut self, doc: &CdxDocument) {
        if let Some(obj) = doc.document_object() {
            if let Some(CdxValue::ColorList(list)) = obj.get_property(tags::COLOR_TABLE) {
                self.color_table = list
                    .iter()
                    .map(|(v1, v2, v3)| {
                        egui::Color32::from_rgb(
                            (*v3 >> 8) as u8,
                            (*v2 >> 8) as u8,
                            (*v1 >> 8) as u8,
                        )
                    })
                    .collect();
            }
            if let Some(CdxValue::Int16(idx)) = obj.get_property(tags::BG_COLOR) {
                self.bg_index = *idx as usize;
            }
            if let Some(CdxValue::Uint16(idx)) = obj.get_property(tags::FG_COLOR) {
                self.fg_index = *idx as usize;
            }
        }
    }

    fn sync_colors_to_doc(&mut self) {
        if let Some(doc) = self.doc.as_mut()
            && let Some(obj) = doc.document_object_mut()
        {
            let list = self
                .color_table
                .iter()
                .map(|c| {
                    (
                        (c.b() as u16) << 8,
                        (c.g() as u16) << 8,
                        (c.r() as u16) << 8,
                    )
                })
                .collect::<Vec<(u16, u16, u16)>>();
            obj.set_property(tags::COLOR_TABLE, CdxValue::ColorList(list));
        }
    }

    fn reset_view(&mut self) {
        self.zoom = 5.0;
        self.offset = egui::Vec2::ZERO;

        if !self.node_positions.is_empty() {
            let mut min_x = f64::MAX;
            let mut max_x = f64::MIN;
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;
            for p in self.node_positions.values() {
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
                min_y = min_y.min(p.y);
                max_y = max_y.max(p.y);
            }
            let mid_x = (min_x + max_x) / 2.0;
            let mid_y = (min_y + max_y) / 2.0;
            self.offset = egui::vec2(-(mid_x as f32 * self.zoom), -(mid_y as f32 * self.zoom));
        }
    }

    fn extract_positions(&mut self, doc: &CdxDocument) {
        for node in &doc.root {
            self.find_nodes_recursive(node);
        }
    }

    fn find_nodes_recursive(&mut self, node: &CdxNode) {
        if let CdxNode::Object(obj) = node {
            if obj.tag == tags::NODE
                && let Some(pos) = obj.get_pos2d()
            {
                self.node_positions.insert(obj.id, pos.clone());
            }
            for child in &obj.children {
                self.find_nodes_recursive(child);
            }
        }
    }

    fn create_new_file(&mut self) {
        let doc_obj = cdx_file_rs::CdxObject {
            tag: tags::DOCUMENT,
            id: 0,
            children: vec![],
        };

        // Setup default colors
        self.color_table = vec![egui::Color32::BLACK, egui::Color32::WHITE];
        self.bg_index = 1;
        self.fg_index = 0;

        self.doc = Some(CdxDocument {
            header: cdx_file_rs::CdxHeader::default(),
            root: vec![CdxNode::Object(doc_obj)],
        });

        // Sync the default state
        if let Some(doc) = self.doc.as_mut()
            && let Some(obj) = doc.document_object_mut()
        {
            obj.set_property(tags::BG_COLOR, CdxValue::Int16(self.bg_index as i16));
            obj.set_property(tags::FG_COLOR, CdxValue::Uint16(self.fg_index as u16));
        }
        self.sync_colors_to_doc();

        self.node_positions.clear();
        self.selected_ids.clear();
        self.lasso_path.clear();
        self.error = None;
        self.reset_view();
    }

    fn export_svg(&mut self) {
        if let Some(doc) = &self.doc
            && let Some(path) = rfd::FileDialog::new()
                .add_filter("SVG", &["svg"])
                .save_file()
        {
            let options = crate::export::ExportOptions {
                bg_index: self.bg_index,
                fg_index: self.fg_index,
                color_table: self.color_table.clone(),
                margin: 10.0,
                scale: 1.0,
            };
            if let Err(e) = crate::export::save_as_svg(doc, &self.node_positions, &path, &options) {
                self.error = Some(format!("Export failed: {}", e));
            }
        }
    }

    fn export_png(&mut self) {
        if let Some(doc) = &self.doc
            && let Some(path) = rfd::FileDialog::new()
                .add_filter("PNG", &["png"])
                .save_file()
        {
            let options = crate::export::ExportOptions {
                bg_index: self.bg_index,
                fg_index: self.fg_index,
                color_table: self.color_table.clone(),
                margin: 10.0,
                scale: 2.0,
            };
            if let Err(e) = crate::export::save_as_png(doc, &self.node_positions, &path, &options) {
                self.error = Some(format!("Export failed: {}", e));
            }
        }
    }
}

impl eframe::App for CdxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("CDX Viewer");

            if ui.button("📁 Open File...").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("ChemDraw CDX", &["cdx"])
                    .pick_file()
            {
                self.load_file(&path.to_string_lossy());
            }

            if ui.button("📄 New File").clicked() {
                self.create_new_file();
            }

            ui.separator();
            ui.label("Export:");
            if ui.button("💾 Export to SVG").clicked() {
                self.export_svg();
            }
            if ui.button("💾 Export to PNG").clicked() {
                self.export_png();
            }

            ui.separator();
            ui.label("Samples:");

            if ui.button("Open benzene.cdx").clicked() {
                self.load_file("sample_cdx/benzene.cdx");
            }
            if ui.button("Open Reaction.cdx").clicked() {
                self.load_file("sample_cdx/Reaction.cdx");
            }

            ui.separator();
            ui.label(format!("Zoom: {:.2}", self.zoom));
            ui.add(egui::Slider::new(&mut self.zoom, 0.1..=50.0));

            if ui.button("Reset View").clicked() {
                self.reset_view();
            }

            if self.doc.is_some() {
                ui.separator();
                ui.label("Mode:");
                ui.radio_value(&mut self.mode, AppMode::Select, "Select (S)");
                ui.radio_value(&mut self.mode, AppMode::Bond, "Bond (B)");
                ui.radio_value(&mut self.mode, AppMode::Eraser, "Eraser (E)");
                ui.radio_value(&mut self.mode, AppMode::View, "View (V)");
            }

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }

            if ui.input(|i| i.key_pressed(egui::Key::S)) {
                self.mode = AppMode::Select;
            }
            if ui.input(|i| i.key_pressed(egui::Key::B)) {
                self.mode = AppMode::Bond;
            }
            if ui.input(|i| i.key_pressed(egui::Key::E)) {
                self.mode = AppMode::Eraser;
            }
            if ui.input(|i| i.key_pressed(egui::Key::V)) {
                self.mode = AppMode::View;
            }

            if self.doc.is_some() {
                ui.separator();
                ui.collapsing("🎨 Color Table", |ui| {
                    let mut doc_changed = false;

                    ui.horizontal(|ui| {
                        ui.label("BG Index:");
                        if egui::ComboBox::from_id_salt("bg_idx")
                            .selected_text(self.bg_index.to_string())
                            .show_ui(ui, |ui| {
                                let mut changed = false;
                                for i in 0..self.color_table.len() {
                                    if ui
                                        .selectable_value(&mut self.bg_index, i, i.to_string())
                                        .clicked()
                                    {
                                        changed = true;
                                    }
                                }
                                changed
                            })
                            .inner
                            .unwrap_or(false)
                        {
                            doc_changed = true;
                            if let Some(doc) = self.doc.as_mut()
                                && let Some(obj) = doc.document_object_mut()
                            {
                                obj.set_property(
                                    tags::BG_COLOR,
                                    CdxValue::Int16(self.bg_index as i16),
                                );
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("FG Index:");
                        if egui::ComboBox::from_id_salt("fg_idx")
                            .selected_text(self.fg_index.to_string())
                            .show_ui(ui, |ui| {
                                let mut changed = false;
                                for i in 0..self.color_table.len() {
                                    if ui
                                        .selectable_value(&mut self.fg_index, i, i.to_string())
                                        .clicked()
                                    {
                                        changed = true;
                                    }
                                }
                                changed
                            })
                            .inner
                            .unwrap_or(false)
                        {
                            doc_changed = true;
                            if let Some(doc) = self.doc.as_mut()
                                && let Some(obj) = doc.document_object_mut()
                            {
                                obj.set_property(
                                    tags::FG_COLOR,
                                    CdxValue::Uint16(self.fg_index as u16),
                                );
                            }
                        }
                    });

                    ui.separator();

                    let mut changed = false;
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (i, color) in self.color_table.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{:2}:", i));
                                    if ui.color_edit_button_srgba(color).changed() {
                                        changed = true;
                                    }
                                    if i == self.bg_index {
                                        ui.small("(BG)");
                                    }
                                    if i == self.fg_index {
                                        ui.small("(FG)");
                                    }
                                });
                            }
                        });

                    if changed {
                        self.sync_colors_to_doc();
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap();

            let bg_color = self
                .color_table
                .get(self.bg_index)
                .cloned()
                .unwrap_or(egui::Color32::BLACK);
            painter.rect_filled(rect, 0.0, bg_color);

            // Handle drag and drop
            ctx.input(|i| {
                if !i.raw.dropped_files.is_empty()
                    && let Some(file) = i.raw.dropped_files.first()
                    && let Some(path) = &file.path
                {
                    self.load_file(&path.to_string_lossy());
                }
            });

            if self.doc.is_some() {
                let renderer = CdxRenderer {
                    zoom: self.zoom,
                    offset: self.offset,
                    color_table: &self.color_table,
                    bg_index: self.bg_index,
                    fg_index: self.fg_index,
                };

                let hit = self.doc.as_ref().and_then(|doc| {
                    renderer.render_all(ui, painter, doc, &self.node_positions);
                    ui.input(|i| i.pointer.hover_pos()).and_then(|mouse_pos| {
                        let center = rect.center() + self.offset;
                        self.find_hit(doc, center, mouse_pos, &renderer)
                    })
                });

                if let (Some(mouse_pos), Some(doc)) =
                    (ui.input(|i| i.pointer.hover_pos()), self.doc.as_mut())
                {
                    let mut handler: Box<dyn ModeHandler> = match self.mode {
                        AppMode::Select => Box::new(SelectMode),
                        AppMode::Bond => Box::new(BondMode),
                        AppMode::Eraser => Box::new(EraserMode),
                        AppMode::View => Box::new(ViewMode),
                    };

                    let response = ui.interact(rect, ui.id(), egui::Sense::click_and_drag());

                    let mut mode_ctx = ModeContext {
                        doc,
                        node_positions: &mut self.node_positions,
                        renderer: &renderer,
                        mouse_pos,
                        hovered_object: hit,
                        ui,
                        drag_delta: response.drag_delta(),

                        view_offset: &mut self.offset,
                        selected_ids: &mut self.selected_ids,
                        lasso_path: &mut self.lasso_path,
                        clipboard: &mut self.clipboard,
                        config: &self.config,
                    };

                    handler.handle_hover(&mode_ctx, painter);

                    let events = ui.input(|i| i.events.clone());
                    for event in events {
                        if let egui::Event::Key {
                            key, pressed: true, ..
                        } = event
                        {
                            handler.handle_key(&mut mode_ctx, key);
                        }
                    }

                    if response.clicked() {
                        handler.handle_click(&mut mode_ctx);
                    }
                    if response.dragged() {
                        handler.handle_drag(&mut mode_ctx);
                    }

                    if response.drag_stopped() && !self.lasso_path.is_empty() {
                        self.perform_lasso_selection(ctx, rect);
                        self.lasso_path.clear();
                    }
                }
            } else {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Select a CDX file to view",
                    egui::FontId::proportional(24.0),
                    egui::Color32::GRAY,
                );
                let response = ui.interact(rect, ui.id(), egui::Sense::drag());
                if response.dragged() {
                    self.offset += response.drag_delta();
                }
            }

            // Global scroll zoom
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
            if scroll_delta != 0.0 {
                let zoom_factor = (scroll_delta / 100.0).exp();
                let old_zoom = self.zoom;
                self.zoom *= zoom_factor;
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let panel_center = rect.center();
                    let p = (mouse_pos - panel_center - self.offset) / old_zoom;
                    self.offset = mouse_pos - panel_center - p * self.zoom;
                }
            }
        });
    }
}

impl CdxApp {
    fn perform_lasso_selection(&mut self, ctx: &egui::Context, rect: egui::Rect) {
        if self.lasso_path.len() < 3 {
            return;
        }
        let center = rect.center() + self.offset;
        let renderer = CdxRenderer {
            zoom: self.zoom,
            offset: self.offset,
            color_table: &self.color_table,
            bg_index: self.bg_index,
            fg_index: self.fg_index,
        };

        let mut new_selection = std::collections::HashSet::new();
        for (id, pos) in &self.node_positions {
            let screen_pos = renderer.to_screen(pos, center);
            if self.is_point_in_lasso(screen_pos) {
                new_selection.insert(*id);
            }
        }

        if !ctx.input(|i| i.modifiers.shift) {
            self.selected_ids = new_selection;
        } else {
            self.selected_ids.extend(new_selection);
        }
    }

    fn is_point_in_lasso(&self, p: egui::Pos2) -> bool {
        let mut inside = false;
        let mut j = self.lasso_path.len() - 1;
        for i in 0..self.lasso_path.len() {
            let pi = self.lasso_path[i];
            let pj = self.lasso_path[j];
            if ((pi.y > p.y) != (pj.y > p.y))
                && (p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    fn find_hit(
        &self,
        doc: &CdxDocument,
        origin: egui::Pos2,
        mouse_pos: egui::Pos2,
        renderer: &CdxRenderer,
    ) -> Option<Hit> {
        for node in &doc.root {
            if let Some(id) = self.check_node_click_recursive(node, origin, mouse_pos, renderer) {
                return Some(Hit {
                    tag: tags::NODE,
                    id,
                });
            }
        }
        for node in &doc.root {
            if let Some(id) = self.check_bond_click_recursive(node, origin, mouse_pos, renderer) {
                return Some(Hit {
                    tag: tags::BOND,
                    id,
                });
            }
        }
        None
    }

    fn check_node_click_recursive(
        &self,
        node: &CdxNode,
        origin: egui::Pos2,
        mouse_pos: egui::Pos2,
        renderer: &CdxRenderer,
    ) -> Option<u32> {
        if let CdxNode::Object(obj) = node {
            if obj.tag == tags::NODE
                && let Some(pos) = self.node_positions.get(&obj.id)
                && (renderer.to_screen(pos, origin) - mouse_pos).length() < 10.0
            {
                return Some(obj.id);
            }
            for child in &obj.children {
                if let Some(id) =
                    self.check_node_click_recursive(child, origin, mouse_pos, renderer)
                {
                    return Some(id);
                }
            }
        }
        None
    }

    fn check_bond_click_recursive(
        &self,
        node: &CdxNode,
        origin: egui::Pos2,
        mouse_pos: egui::Pos2,
        renderer: &CdxRenderer,
    ) -> Option<u32> {
        if let CdxNode::Object(obj) = node {
            if obj.tag == tags::BOND
                && let Some((bid, eid)) = obj.get_bond_endpoints()
                && let (Some(bp), Some(ep)) =
                    (self.node_positions.get(&bid), self.node_positions.get(&eid))
            {
                let p1 = renderer.to_screen(bp, origin);
                let p2 = renderer.to_screen(ep, origin);
                if (p1 + (p2 - p1) * 0.5 - mouse_pos).length() < 8.0 {
                    return Some(obj.id);
                }
            }
            for child in &obj.children {
                if let Some(id) =
                    self.check_bond_click_recursive(child, origin, mouse_pos, renderer)
                {
                    return Some(id);
                }
            }
        }
        None
    }
}
