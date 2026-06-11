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
use crate::renderer::to_points::ToBackendF32;
use cdx_file_rs::renderer::font_loader;
use eframe::{App, egui};
use std::cell::RefCell;
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
            bond: mode_handlers::bond::BondMode::new(),
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
        Box::new(|cc| {
            // Load system fonts for rich text rendering
            font_loader::configure_egui_fonts(&cc.egui_ctx);

            // Print loaded fonts info
            for info in font_loader::get_loaded_font_info() {
                println!("{}", info);
            }

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
    cdx_file: RefCell<Option<CdxFile>>,
    error: Option<String>,
    zoom: f32,
    offset: egui::Vec2,
    center_offset: egui::Vec2, // Store the auto-calculated center offset
    auto_scale: f32,           // Store the auto-calculated scale
    mode: AppMode,             // Current editing mode
    selected_ids: std::collections::HashSet<u32>, // Selected object IDs
    lasso_path: Vec<egui::Pos2>, // Lasso selection path
    mode_handlers: ModeHandlers, // Mode handlers
    clipboard: Option<CdxFile>, // Clipboard for copy/paste
}

impl Default for CdxApp {
    fn default() -> Self {
        Self {
            cdx_file: RefCell::new(None),
            error: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            center_offset: egui::Vec2::ZERO,
            auto_scale: 1.0,
            mode: AppMode::View,
            selected_ids: std::collections::HashSet::new(),
            lasso_path: Vec::new(),
            mode_handlers: ModeHandlers::default(),
            clipboard: None,
        }
    }
}

impl CdxApp {
    fn load_file(&mut self, path: &str) {
        match fs::read(path) {
            Ok(data) => {
                match CdxFile::from_bytes(&data) {
                    Ok(cdx) => {
                        *self.cdx_file.borrow_mut() = Some(cdx);
                        self.error = None;
                        self.reset_view();

                        // auto_scale and center_offset will be recalculated each frame
                        // using the actual panel rect size, so no pre-calculation needed here.
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

    // calculate_center_offsetは不要なので削除

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

    fn collect_bond_positions(
        &self,
        root: &dendron::Node<crate::cdx::file::NodePayload>,
        node_positions: &std::collections::HashMap<u32, crate::cdx::values::Point2d>,
        bond_positions: &mut std::collections::HashMap<u32, crate::modes::BondPosition>,
    ) {
        let data = root.borrow_data();

        if let crate::cdx::file::NodePayload::Bond(bond_obj) = &*data {
            // Get positions of begin and end nodes
            if let (Some(begin_id), Some(end_id)) = (bond_obj.begin, bond_obj.end) {
                if let (Some(begin_pos), Some(end_pos)) =
                    (node_positions.get(&begin_id), node_positions.get(&end_id))
                {
                    bond_positions.insert(
                        bond_obj.id,
                        (begin_id, end_id, begin_pos.clone(), end_pos.clone()),
                    );
                }
            }
        }

        for child in root.children() {
            self.collect_bond_positions(&child, node_positions, bond_positions);
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
            let has_file = self.cdx_file.borrow().is_some();
            if has_file {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

                // Get mouse position
                let mouse_pos = response.interact_pointer_pos().unwrap_or(egui::Pos2::ZERO);
                let drag_delta = response.drag_delta();
                let is_dragging = response.dragged();
                let clicked = response.clicked();
                let drag_stopped = response.drag_stopped();

                // Collect node positions and bond positions from immutable borrow
                let (node_positions, bond_positions) = {
                    let cdx_borrow = self.cdx_file.borrow();
                    let cdx_file = cdx_borrow.as_ref().unwrap();

                    let mut node_positions: std::collections::HashMap<
                        u32,
                        crate::cdx::values::Point2d,
                    > = std::collections::HashMap::new();
                    self.collect_node_positions(&cdx_file.tree.root(), &mut node_positions);

                    let mut bond_positions: std::collections::HashMap<
                        u32,
                        crate::modes::BondPosition,
                    > = std::collections::HashMap::new();
                    self.collect_bond_positions(
                        &cdx_file.tree.root(),
                        &node_positions,
                        &mut bond_positions,
                    );

                    (node_positions, bond_positions)
                };

                // Recalculate auto_scale and center_offset from the actual panel size each frame.
                // This fixes the misalignment caused by using a hardcoded 1200×800 at load time
                // when the CentralPanel is actually smaller (top panel subtracts height).
                {
                    let cdx_borrow = self.cdx_file.borrow();
                    let cdx_file = cdx_borrow.as_ref().unwrap();
                    let window_size = egui::Vec2::new(rect.width(), rect.height());
                    let renderer = CdxRenderer::new(1.0, egui::Vec2::ZERO, cdx_file, window_size);
                    let (auto_scale, center_offset) = renderer.calculate_auto_scale(&node_positions);
                    self.center_offset = center_offset;
                    self.auto_scale = auto_scale;
                }

                // Handle mode-specific input FIRST (before borrowing for renderer)
                // This allows mode handlers to mutably borrow cdx_file
                {
                    // Create a temporary renderer just for coordinate conversion
                    // (we'll create another one for actual rendering after input is handled)
                    let cdx_borrow = self.cdx_file.borrow();
                    let cdx_file = cdx_borrow.as_ref().unwrap();
                    let window_size = egui::Vec2::new(rect.width(), rect.height());
                    let renderer = CdxRenderer::with_center_offset(
                        self.zoom,
                        self.offset,
                        self.center_offset,
                        self.auto_scale,
                        cdx_file,
                        window_size,
                    );

                    // Store renderer settings we need for coordinate conversion
                    let zoom = renderer.zoom;
                    let auto_scale = renderer.auto_scale;
                    let center_offset = renderer.center_offset;
                    let offset = renderer.offset;

                    drop(cdx_borrow); // Drop the borrow before calling handlers

                    let mut mode_ctx = ModeContext {
                        mouse_pos,
                        ui,
                        drag_delta,
                        view_offset: &mut self.offset,
                        zoom,
                        auto_scale,
                        center_offset,
                        offset,
                        node_positions: &node_positions,
                        bond_positions: &bond_positions,
                        selected_ids: &mut self.selected_ids,
                        lasso_path: &mut self.lasso_path,
                        is_dragging,
                        clipboard: &mut self.clipboard,
                        cdx_file: &self.cdx_file,
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

                // Handle zoom (scroll wheel / touchpad / pinch) - zoom around pointer position
                if ui.rect_contains_pointer(rect) {
                    let zoom_delta = ui.input(|i| i.zoom_delta());
                    let mut zoom_factor = 1.0;

                    if (zoom_delta - 1.0).abs() > f32::EPSILON {
                        zoom_factor = zoom_delta;
                    } else {
                        let scroll = ui.input(|i| i.raw_scroll_delta.y);
                        if scroll != 0.0 {
                            zoom_factor = if scroll > 0.0 { 1.1 } else { 0.9 };
                        }
                    }

                    if (zoom_factor - 1.0).abs() > f32::EPSILON {
                        let focus_pos = ui
                            .input(|i| i.pointer.interact_pos())
                            .unwrap_or(rect.center());
                        let focus_rel = focus_pos - rect.min;

                        let origin = egui::Vec2::new(
                            self.center_offset.x + self.offset.x,
                            self.center_offset.y + self.offset.y,
                        );
                        let world_pos = (focus_rel - origin) / (self.zoom * self.auto_scale);

                        self.zoom *= zoom_factor;

                        let new_origin = focus_rel - world_pos * self.zoom * self.auto_scale;
                        self.offset = new_origin - self.center_offset;
                    }
                }

                // Now create renderer for actual rendering (with fresh borrow)
                let cdx_borrow = self.cdx_file.borrow();
                let cdx_file = cdx_borrow.as_ref().unwrap();
                let window_size = egui::Vec2::new(rect.width(), rect.height());
                let renderer = CdxRenderer::with_center_offset(
                    self.zoom,
                    self.offset,
                    self.center_offset,
                    self.auto_scale,
                    cdx_file,
                    window_size,
                );

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
                        zoom: renderer.zoom,
                        auto_scale: renderer.auto_scale,
                        center_offset: renderer.center_offset,
                        offset: renderer.offset,
                        node_positions: &node_positions,
                        bond_positions: &bond_positions,
                        selected_ids: &mut self.selected_ids,
                        lasso_path: &mut self.lasso_path,
                        is_dragging,
                        clipboard: &mut self.clipboard,
                        cdx_file: &self.cdx_file,
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
