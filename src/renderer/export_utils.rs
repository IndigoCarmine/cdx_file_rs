/// Export utilities for rendering CDX files to SVG and PNG using AbstractPainter
///
/// This module provides high-level functions to render CDX documents to SVG and PNG
/// using the abstracted rendering system.

use crate::cdx::document::Document;
use crate::cdx::file::{CdxFile, NodePayload};
use crate::cdx::values::Point2d as CdxPoint2d;
use crate::renderer::{backend::Color, ImagePngBackend, RenderContext, SvgBackend};
use dendron::Node;
use eframe::egui::Pos2;
use std::collections::HashMap;
use std::path::Path;

/// Options for exporting CDX documents
#[derive(Clone)]
pub struct RenderExportOptions {
    /// Background color (RGB)
    pub background_color: Color,
    /// Width of the output in pixels
    pub width: u32,
    /// Height of the output in pixels
    pub height: u32,
    /// Margin around the content in pixels
    pub margin: f32,
    /// Scale factor for PNG resolution
    pub scale: f32,
}

impl Default for RenderExportOptions {
    fn default() -> Self {
        RenderExportOptions {
            background_color: Color::WHITE,
            width: 800,
            height: 600,
            margin: 20.0,
            scale: 1.0,
        }
    }
}

/// Export a CDX file to SVG format
///
/// # Arguments
/// * `cdx_file` - The CDX file to render
/// * `path` - Output file path
/// * `options` - Export options
///
/// # Returns
/// Result indicating success or error message
pub fn export_to_svg(
    cdx_file: &CdxFile,
    path: &Path,
    options: &RenderExportOptions,
) -> Result<(), String> {
    let svg_content = render_to_svg(cdx_file, options)?;
    std::fs::write(path, svg_content).map_err(|e| e.to_string())
}

/// Export a CDX file to PNG format
///
/// # Arguments
/// * `cdx_file` - The CDX file to render
/// * `path` - Output file path
/// * `options` - Export options
///
/// # Returns
/// Result indicating success or error message
pub fn export_to_png(
    cdx_file: &CdxFile,
    path: &Path,
    options: &RenderExportOptions,
) -> Result<(), String> {
    // Calculate dimensions with scale
    let width = (options.width as f32 * options.scale) as u32;
    let height = (options.height as f32 * options.scale) as u32;

    let backend = ImagePngBackend::new(width, height, options.background_color);

    render_to_backend(cdx_file, &backend, options)?;

    backend.save_png(path)
}

/// Render a CDX file to SVG string
///
/// # Arguments
/// * `cdx_file` - The CDX file to render
/// * `options` - Export options
///
/// # Returns
/// SVG content as a string or error message
pub fn render_to_svg(cdx_file: &CdxFile, options: &RenderExportOptions) -> Result<String, String> {
    let clip_rect = crate::renderer::backend::Rect {
        min: crate::renderer::backend::Point2d { x: 0.0, y: 0.0 },
        max: crate::renderer::backend::Point2d {
            x: options.width as f32,
            y: options.height as f32,
        },
    };

    let backend = SvgBackend::new(clip_rect);

    // Add background rectangle
    if options.background_color.a > 0 {
        use crate::renderer::backend::AbstractPainter;
        backend.rect_filled(clip_rect, 0.0, options.background_color);
    }

    render_to_backend(cdx_file, &backend, options)?;

    Ok(backend.to_svg_document(options.width as f32, options.height as f32))
}

/// Internal function to render CDX file to any AbstractPainter backend
fn render_to_backend<P: crate::renderer::backend::AbstractPainter>(
    cdx_file: &CdxFile,
    backend: &P,
    options: &RenderExportOptions,
) -> Result<(), String> {
    // Get document
    let document = cdx_file
        .get_document()
        .map_err(|e| format!("Failed to get document: {}", e))?;

    // Collect node positions
    let mut node_positions: HashMap<u32, CdxPoint2d> = HashMap::new();
    let tree = &cdx_file.tree;
    let root = tree.root();
    collect_node_positions(root.clone(), &mut node_positions);

    // Calculate bounds and scaling
    let (auto_scale, center_offset) = calculate_auto_scale(
        &node_positions,
        options.width as f32,
        options.height as f32,
        options.margin,
    );

    // Create render context
    let ctx = RenderContext::new(
        backend,
        Pos2 {
            x: center_offset.0,
            y: center_offset.1,
        },
        &document,
        node_positions,
        1.0, // zoom
        auto_scale,
    );

    // Render the tree
    render_tree(root, &ctx);

    Ok(())
}

/// Recursively collect node positions from the tree
fn collect_node_positions(root: Node<NodePayload>, node_positions: &mut HashMap<u32, CdxPoint2d>) {
    let data = root.borrow_data();

    if let NodePayload::Node(node_obj) = &*data {
        if let Some(pos) = &node_obj.position_2d {
            node_positions.insert(node_obj.id, pos.clone());
        } else if let Some(pos3d) = &node_obj.position_3d {
            // Try to use 3D position if 2D is not available
            let pos_2d = CdxPoint2d {
                x: pos3d.x,
                y: pos3d.y,
            };
            node_positions.insert(node_obj.id, pos_2d);
        }
    }

    for child in root.children() {
        collect_node_positions(child, node_positions);
    }
}

/// Calculate auto-scale factor to fit document bounds within window
fn calculate_auto_scale(
    node_positions: &HashMap<u32, CdxPoint2d>,
    width: f32,
    height: f32,
    margin: f32,
) -> (f32, (f32, f32)) {
    if node_positions.is_empty() {
        return (1.0, (width / 2.0, height / 2.0));
    }

    // Find bounding box of all nodes
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
        return (1.0, (width / 2.0, height / 2.0));
    }

    // Calculate scale to fit in window with some padding
    let available_width = width - margin * 2.0;
    let available_height = height - margin * 2.0;

    let scale_x = available_width / doc_width as f32;
    let scale_y = available_height / doc_height as f32;
    let auto_scale = scale_x.min(scale_y);

    // Calculate center offset
    let doc_center_x = ((min_x + max_x) / 2.0) as f32;
    let doc_center_y = ((min_y + max_y) / 2.0) as f32;
    let window_center_x = width / 2.0;
    let window_center_y = height / 2.0;

    let center_offset_x = window_center_x - doc_center_x * auto_scale;
    let center_offset_y = window_center_y - doc_center_y * auto_scale; // CDX Y increases downward (same as screen)

    (auto_scale, (center_offset_x, center_offset_y))
}

/// Recursively render the tree
fn render_tree<P: crate::renderer::backend::AbstractPainter>(
    root: Node<NodePayload>,
    ctx: &RenderContext<P>,
) {
    let data = root.borrow_data();

    // Draw the current object
    data.draw(ctx);

    // Check if this object defines a coordinate offset for its children
    let child_ctx;
    let ctx_ref: &RenderContext<P> = if let NodePayload::Page(page) = &*data {
        if let Some(bounds) = &page.bounds_in_parent {
            // Create a child context with offset from the parent's top-left corner
            let offset = CdxPoint2d {
                x: bounds.left,
                y: bounds.top,
            };
            child_ctx = ctx.with_offset(&offset);
            &child_ctx
        } else {
            ctx
        }
    } else {
        ctx
    };

    // Render children with potentially modified context
    for child in root.children() {
        render_tree(child, ctx_ref);
    }
}
