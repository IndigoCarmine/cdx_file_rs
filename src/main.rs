mod cdx;
mod cdx_parse_impl;
mod cdx_tags;
mod error;
mod mode_handlers;
mod modes;
mod renderer;

use crate::cdx::file::CdxFile;
use crate::modes::{ModeContext, ModeHandler};
use crate::renderer::CdxRenderer;
use eframe::{App, egui};
use std::fs;

struct ModeHandlers {
    view: mode_handlers::view::ViewMode,
    select: mode_handlers::select::SelectMode,
    bond: mode_handlers::bond::BondMode,
    eraser: mode_handlers::eraser::EraserMode,
    debug: mode_handlers::debug::DebugMode,
}

impl Default for ModeHandlers {
    fn default() -> Self {
        Self {
            view: mode_handlers::view::ViewMode,
            select: mode_handlers::select::SelectMode,
            bond: mode_handlers::bond::BondMode,
            eraser: mode_handlers::eraser::EraserMode,
            debug: mode_handlers::debug::DebugMode::new(),
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CDX Viewer",
        options,
        Box::new(|_cc| {
            let app = CdxApp::default();
            Ok(Box::new(app))
        }),
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum AppMode {
    View,
    Select,
    Bond,
    Eraser,
    Debug,
}

struct CdxApp {
    cdx_file: Option<CdxFile>,
    error: Option<String>,
    zoom: f32,
    offset: egui::Vec2,
    center_offset: egui::Vec2, // Store the auto-calculated center offset
    auto_scale: f32,           // Store the auto-calculated scale
    mode: AppMode,             // Current editing mode
    selected_ids: std::collections::HashSet<u32>, // Selected object IDs
    lasso_path: Vec<egui::Pos2>, // Lasso selection path
    mode_handlers: ModeHandlers, // Mode handlers
}

impl Default for CdxApp {
    fn default() -> Self {
        Self {
            cdx_file: None,
            error: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            center_offset: egui::Vec2::ZERO,
            auto_scale: 1.0,
            mode: AppMode::View,
            selected_ids: std::collections::HashSet::new(),
            lasso_path: Vec::new(),
            mode_handlers: ModeHandlers::default(),
        }
    }
}

impl CdxApp {
    fn load_file(&mut self, path: &str) {
        match fs::read(path) {
            Ok(data) => {
                match CdxFile::from_bytes(&data) {
                    Ok(cdx) => {
                        self.cdx_file = Some(cdx);
                        self.error = None;
                        self.reset_view();

                        // Calculate center offset for auto-scaling
                        if let Some(ref cdx_file) = self.cdx_file {
                            let (offset, scale) = self.calculate_center_offset(cdx_file);
                            self.center_offset = offset;
                            self.auto_scale = scale;
                        }
                    }
                    Err(e) => self.error = Some(format!("Failed to parse CDX: {}", e)),
                }
            }
            Err(e) => self.error = Some(format!("Failed to read file: {}", e)),
        }
    }

    fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.offset = egui::Vec2::ZERO;
        // center_offset remains as calculated when file was loaded
    }

    fn calculate_center_offset(&self, cdx_file: &CdxFile) -> (egui::Vec2, f32) {
        let mut node_positions: std::collections::HashMap<u32, crate::cdx::values::Point2d> =
            std::collections::HashMap::new();

        // Collect all node positions
        self.collect_node_positions(&cdx_file.tree.root(), &mut node_positions);

        // Calculate bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for pos in node_positions.values() {
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
        }

        let doc_width = max_x - min_x;
        let doc_height = max_y - min_y;

        if doc_width <= 0.0 || doc_height <= 0.0 {
            return (egui::Vec2::ZERO, 1.0);
        }

        // Calculate scale to fit in window with padding
        let padding = 50.0;
        let available_width = 1200.0 - padding * 2.0; // window width
        let available_height = 800.0 - padding * 2.0; // window height

        let scale_x = available_width / doc_width as f32;
        let scale_y = available_height / doc_height as f32;
        let auto_scale = scale_x.min(scale_y);

        // Calculate center offset
        let doc_center_x = ((min_x + max_x) / 2.0) as f32;
        let doc_center_y = ((min_y + max_y) / 2.0) as f32;
        let window_center_x = 1200.0 / 2.0;
        let window_center_y = 800.0 / 2.0;

        let offset = egui::Vec2::new(
            window_center_x - doc_center_x * auto_scale,
            window_center_y + doc_center_y * auto_scale,
        );

        (offset, auto_scale)
    }

    fn collect_node_positions(
        &self,
        root: &dendron::Node<crate::cdx::file::NodePayload>,
        node_positions: &mut std::collections::HashMap<u32, crate::cdx::values::Point2d>,
    ) {
        let data = root.borrow_data();

        if let crate::cdx::file::NodePayload::Node(node_obj) = &*data {
            if let Some(pos) = &node_obj.position_2d {
                node_positions.insert(node_obj.id, pos.clone());
            }
        }

        for child in root.children() {
            self.collect_node_positions(&child, node_positions);
        }
    }
}

impl eframe::App for CdxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("CDX File Viewer");

            ui.horizontal(|ui| {
                if ui.button("📁 Open File...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("ChemDraw CDX", &["cdx"])
                        .pick_file()
                    {
                        self.load_file(&path.to_string_lossy());
                    }
                }

                if ui.button("Open benzene.cdx").clicked() {
                    self.load_file("sample_cdx/benzene.cdx");
                }

                if ui.button("Open Reaction.cdx").clicked() {
                    self.load_file("sample_cdx/Reaction.cdx");
                }

                if ui.button("Open ReactionAnalysis.cdx").clicked() {
                    self.load_file("sample_cdx/ReactionAnalysis.cdx");
                }
                if ui.button("Open Analysis.cdx").clicked() {
                    self.load_file("sample_cdx/Analysis.cdx");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(&mut self.mode, AppMode::View, "🔍 View");
                ui.selectable_value(&mut self.mode, AppMode::Select, "⬚ Select");
                ui.selectable_value(&mut self.mode, AppMode::Bond, "➖ Bond");
                ui.selectable_value(&mut self.mode, AppMode::Eraser, "🗑 Eraser");
                ui.selectable_value(&mut self.mode, AppMode::Debug, "🐛 Debug");
            });

            ui.horizontal(|ui| {
                ui.label(format!("Zoom: {:.2}", self.zoom));
                if ui.button("Reset View").clicked() {
                    self.zoom = 1.0;
                    self.offset = egui::Vec2::ZERO;
                }
                if ui.button("Zoom In").clicked() {
                    self.zoom *= 1.2;
                }
                if ui.button("Zoom Out").clicked() {
                    self.zoom /= 1.2;
                }
            });

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref cdx_file) = self.cdx_file {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

                // Get mouse position
                let mouse_pos = response.interact_pointer_pos().unwrap_or(egui::Pos2::ZERO);
                let drag_delta = response.drag_delta();
                let is_dragging = response.dragged();
                let clicked = response.clicked();
                let drag_stopped = response.drag_stopped();

                // Collect node positions
                let mut node_positions: std::collections::HashMap<
                    u32,
                    crate::cdx::values::Point2d,
                > = std::collections::HashMap::new();
                self.collect_node_positions(&cdx_file.tree.root(), &mut node_positions);

                // Create renderer
                let window_size = egui::Vec2::new(rect.width(), rect.height());
                let renderer = CdxRenderer::with_center_offset(
                    self.zoom,
                    self.offset,
                    self.center_offset,
                    self.auto_scale,
                    cdx_file,
                    window_size,
                );

                // Handle mode-specific input in a separate scope
                {
                    let mut mode_ctx = ModeContext {
                        mouse_pos,
                        ui,
                        drag_delta,
                        view_offset: &mut self.offset,
                        renderer: &renderer,
                        node_positions: &node_positions,
                        selected_ids: &mut self.selected_ids,
                        lasso_path: &mut self.lasso_path,
                        is_dragging,
                    };

                    let handler: &mut dyn ModeHandler = match self.mode {
                        AppMode::View => &mut self.mode_handlers.view,
                        AppMode::Select => &mut self.mode_handlers.select,
                        AppMode::Bond => &mut self.mode_handlers.bond,
                        AppMode::Eraser => &mut self.mode_handlers.eraser,
                        AppMode::Debug => &mut self.mode_handlers.debug,
                    };

                    if clicked {
                        handler.handle_click(&mut mode_ctx);
                    }

                    if is_dragging {
                        handler.handle_drag(&mut mode_ctx);
                    } else if drag_stopped {
                        handler.handle_drag_end(&mut mode_ctx);
                    }

                    // Handle keyboard input
                    ui.input(|i| {
                        for key in &i.keys_down {
                            handler.handle_key(&mut mode_ctx, *key);
                        }
                    });
                }

                // Handle zoom with scroll - zoom around mouse position
                if ui.rect_contains_pointer(rect) {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll != 0.0 {
                        // Get mouse position in screen coordinates
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            // Calculate mouse position relative to rect
                            let mouse_rel = mouse_pos - rect.min;

                            // Get current world position of mouse
                            let origin = egui::Vec2::new(
                                self.center_offset.x + self.offset.x,
                                self.center_offset.y + self.offset.y,
                            );
                            let world_pos = (mouse_rel - origin) / (self.zoom * self.auto_scale);

                            // Apply zoom
                            let zoom_factor = if scroll > 0.0 { 1.1 } else { 0.9 };
                            self.zoom *= zoom_factor;

                            // Adjust offset so the same world position stays under the mouse
                            let new_origin = mouse_rel - world_pos * self.zoom * self.auto_scale;
                            self.offset = new_origin - self.center_offset;
                        }
                    }
                }

                // Render the document
                let painter = ui.painter_at(rect);
                renderer.render_all(&painter, cdx_file);

                // Render mode-specific overlay in a separate scope
                {
                    let mode_ctx = ModeContext {
                        mouse_pos,
                        ui,
                        drag_delta,
                        view_offset: &mut self.offset,
                        renderer: &renderer,
                        node_positions: &node_positions,
                        selected_ids: &mut self.selected_ids,
                        lasso_path: &mut self.lasso_path,
                        is_dragging,
                    };

                    let handler: &dyn ModeHandler = match self.mode {
                        AppMode::View => &self.mode_handlers.view,
                        AppMode::Select => &self.mode_handlers.select,
                        AppMode::Bond => &self.mode_handlers.bond,
                        AppMode::Eraser => &self.mode_handlers.eraser,
                        AppMode::Debug => &self.mode_handlers.debug,
                    };

                    handler.handle_hover(&mode_ctx, &painter);
                }
            } else {
                let rect = ui.available_rect_before_wrap();
                let painter = ui.painter();
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Select a CDX file to view",
                    egui::FontId::proportional(24.0),
                    egui::Color32::GRAY,
                );
            }
        });
    }
}
