mod renderer;
mod cdx_parse_impl;
mod cdx;
mod cdx_tags;
mod error;

use eframe::egui;
use crate::cdx::file::CdxFile;
use renderer::CdxRenderer;
use std::env;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let path_arg = env::args().nth(1);
    eframe::run_native(
        "CDX File Viewer",
        options,
        Box::new(move |_cc| Ok(Box::new(CdxViewerApp::new(path_arg.clone())))),
    )
}

struct CdxViewerApp {
    cdx_file: Option<CdxFile>,
    error_message: Option<String>,
    zoom: f32,
    offset: egui::Vec2,
    source_path: String,
}

impl CdxViewerApp {
    fn new(path: Option<String>) -> Self {
        let source_path = path.unwrap_or_else(|| "sample_cdx/benzene.cdx".to_string());
        let mut app = CdxViewerApp {
            cdx_file: None,
            error_message: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            source_path,
        };

        let path_to_load = app.source_path.clone();
        if let Err(e) = app.load_file(&path_to_load) {
            app.error_message = Some(format!("Failed to load {}: {}", app.source_path, e));
        }

        app
    }

    fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
       
        self.cdx_file = Some(CdxFile::from_bytes(data.as_slice())?);

        self.error_message = None;
        Ok(())
    }
}

impl eframe::App for CdxViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("CDX File Viewer");
            ui.label(format!("File: {}", self.source_path));
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

            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref cdx_file) = self.cdx_file {
                // Create canvas area
                let (rect, response) = ui.allocate_exact_size(
                    ui.available_size(),
                    egui::Sense::drag(),
                );

                // Handle pan (drag) with middle mouse button
                if response.dragged_by(egui::PointerButton::Middle) {
                    self.offset += response.drag_delta();
                }

                // Handle zoom with scroll
                if ui.rect_contains_pointer(rect) {
                    let scroll = ui.input(|i| i.raw_scroll_delta.y);
                    if scroll != 0.0 {
                        let zoom_factor = if scroll > 0.0 { 1.1 } else { 0.9 };
                        self.zoom *= zoom_factor;
                    }
                }

                // Render CDX file
                let painter = ui.painter_at(rect);


                // Create renderer
                let window_size = egui::Vec2::new(rect.width(), rect.height());
                let renderer = CdxRenderer::new(self.zoom, self.offset, cdx_file, window_size);

                // Render all objects
                renderer.render_all(&painter, cdx_file);
            } else {
                ui.label("No CDX file loaded");
            }
        });
    }
}