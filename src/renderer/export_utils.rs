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

/// Load system Arial font data if available, or fall back to the embedded font.
fn load_png_backend(width: u32, height: u32, background: Color) -> ImagePngBackend {
    let candidate_paths: &[&str] = &[
        r"C:\Windows\Fonts\arial.ttf",
        r"C:\Windows\Fonts\Arial.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/Library/Fonts/Arial.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
    ];

    for path in candidate_paths {
        if let Ok(data) = std::fs::read(path) {
            if let Some(backend) = ImagePngBackend::with_font(width, height, background, data) {
                return backend;
            }
        }
    }

    ImagePngBackend::new(width, height, background)
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
    // Calculate actual pixel dimensions
    let width = (options.width as f32 * options.scale) as u32;
    let height = (options.height as f32 * options.scale) as u32;

    let backend = load_png_backend(width, height, options.background_color);

    // Use actual pixel dimensions so auto_scale fills the real canvas
    let mut scaled_opts = options.clone();
    scaled_opts.width = width;
    scaled_opts.height = height;
    scaled_opts.margin = options.margin * options.scale;
    scaled_opts.scale = 1.0;

    render_to_backend(cdx_file, &backend, &scaled_opts)?;

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

    // Collect node positions (for bond rendering)
    let mut node_positions: HashMap<u32, CdxPoint2d> = HashMap::new();
    let tree = &cdx_file.tree;
    let root = tree.root();
    collect_node_positions(root.clone(), &mut node_positions);

    // Collect all visible object positions for bounds calculation (atoms + text + graphics)
    let mut all_bounds: Vec<(f64, f64)> = node_positions
        .values()
        .map(|p| (p.x / 65536.0, p.y / 65536.0))
        .collect();
    collect_all_bounds(root.clone(), &mut all_bounds);

    // Calculate bounds and scaling using all-object positions
    let (auto_scale, center_offset) = calculate_auto_scale_from_points(
        &all_bounds,
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

/// Collect positions from TextObjects and Graphics for bounds calculation
fn collect_all_bounds(root: Node<NodePayload>, points: &mut Vec<(f64, f64)>) {
    {
        let data = root.borrow_data();
        match &*data {
            NodePayload::TextObject(t) => {
                if let Some(false) = t.visible {
                    // skip invisible
                } else if let Some(pos) = &t.position_2d {
                    // position_2d is raw CDX fixed-point; divide by 65536 → CDX pts
                    let px = pos.x / 65536.0;
                    let py = pos.y / 65536.0;
                    points.push((px, py));
                    if let Some(bb) = &t.bounding_box {
                        points.push((bb.left / 65536.0, bb.top / 65536.0));
                        points.push((bb.right / 65536.0, bb.bottom / 65536.0));
                    } else if let Some(cdx_str) = &t.text {
                        // No bounding box: estimate extent from text content + font size.
                        // CDX font_size is in 20ths-of-a-point; CDX coordinate unit = 1 typographic point,
                        // so font_size_pt (in CDX pts) equals font_size_raw / 20.
                        let font_size_pt = cdx_str.style_runs.first()
                            .map(|r| r.font_size as f64 / 20.0)
                            .or_else(|| t.caption_size.map(|s| s as f64 / 20.0))
                            .or_else(|| t.label_size.map(|s| s as f64 / 20.0))
                            .unwrap_or(10.0);
                        let max_line_chars = cdx_str.text
                            .split(|c: char| c == '\r' || c == '\n')
                            .map(|l| l.chars().count())
                            .max()
                            .unwrap_or(0) as f64;
                        let num_lines = cdx_str.text
                            .split(|c: char| c == '\r' || c == '\n')
                            .count() as f64;
                        // ~0.55 pts per char is a reasonable approximation for proportional fonts
                        let est_width = max_line_chars * font_size_pt * 0.55;
                        let est_height = num_lines * font_size_pt * 1.2;
                        if est_width > 0.0 { points.push((px + est_width, py)); }
                        if est_height > 0.0 { points.push((px, py + est_height)); }
                    }
                }
            }
            NodePayload::Graphic(g) => {
                if let Some(bb) = &g.bounding_box {
                    points.push((bb.left / 65536.0, bb.top / 65536.0));
                    points.push((bb.right / 65536.0, bb.bottom / 65536.0));
                }
                if let (Some(h), Some(t)) = (&g.head_3d, &g.tail_3d) {
                    // head/tail_3d are already in CDX pts
                    points.push((h.x, h.y));
                    points.push((t.x, t.y));
                }
            }
            NodePayload::Arrow(a) => {
                if let Some(bb) = &a.bounding_box {
                    points.push((bb.left / 65536.0, bb.top / 65536.0));
                    points.push((bb.right / 65536.0, bb.bottom / 65536.0));
                }
                if let Some(h) = &a.head_3d {
                    points.push((h.x, h.y));
                }
                if let Some(t) = &a.tail_3d {
                    points.push((t.x, t.y));
                }
            }
            _ => {}
        }
    }
    for child in root.children() {
        collect_all_bounds(child, points);
    }
}

/// Calculate auto-scale from a flat list of (x,y) CDX-pt positions
fn calculate_auto_scale_from_points(
    points: &[(f64, f64)],
    width: f32,
    height: f32,
    margin: f32,
) -> (f32, (f32, f32)) {
    if points.is_empty() {
        return (1.0, (width / 2.0, height / 2.0));
    }

    let min_x = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let doc_width = max_x - min_x;
    let doc_height = max_y - min_y;

    if doc_width <= 0.0 || doc_height <= 0.0 {
        return (1.0, (width / 2.0, height / 2.0));
    }

    let available_width = width - margin * 2.0;
    let available_height = height - margin * 2.0;

    let scale_x = available_width / doc_width as f32;
    let scale_y = available_height / doc_height as f32;
    let auto_scale = scale_x.min(scale_y);

    let doc_center_x = ((min_x + max_x) / 2.0) as f32;
    let doc_center_y = ((min_y + max_y) / 2.0) as f32;

    let center_offset_x = width / 2.0 - doc_center_x * auto_scale;
    let center_offset_y = height / 2.0 - doc_center_y * auto_scale;

    (auto_scale, (center_offset_x, center_offset_y))
}

/// Export a CDX file to PNG with debug bounding boxes overlaid (matches App's Debug Mode)
pub fn export_to_png_debug(
    cdx_file: &CdxFile,
    path: &Path,
    options: &RenderExportOptions,
) -> Result<(), String> {
    let width = (options.width as f32 * options.scale) as u32;
    let height = (options.height as f32 * options.scale) as u32;
    let backend = load_png_backend(width, height, options.background_color);
    render_to_backend(cdx_file, &backend, options)?;

    let debug_stroke = crate::renderer::backend::Stroke::new(
        1.0,
        crate::renderer::backend::Color::from_rgb(192, 192, 192),
    );
    let document = cdx_file
        .get_document()
        .map_err(|e| format!("Failed to get document: {}", e))?;
    let mut node_positions: HashMap<u32, CdxPoint2d> = HashMap::new();
    let tree = &cdx_file.tree;
    let root = tree.root();
    collect_node_positions(root.clone(), &mut node_positions);
    let mut all_bounds: Vec<(f64, f64)> = node_positions
        .values()
        .map(|p| (p.x / 65536.0, p.y / 65536.0))
        .collect();
    collect_all_bounds(root.clone(), &mut all_bounds);
    let (auto_scale, center_offset) = calculate_auto_scale_from_points(
        &all_bounds,
        options.width as f32,
        options.height as f32,
        options.margin,
    );
    let ctx = RenderContext::new(
        &backend,
        Pos2 { x: center_offset.0, y: center_offset.1 },
        &document,
        node_positions,
        1.0,
        auto_scale,
    );
    draw_debug_boxes(root, &ctx, &debug_stroke);
    backend.save_png(path)
}

/// Recursively draw light gray bounding boxes for all objects (replicates App's drawbox())
fn draw_debug_boxes<P: crate::renderer::backend::AbstractPainter>(
    root: Node<NodePayload>,
    ctx: &RenderContext<P>,
    stroke: &crate::renderer::backend::Stroke,
) {
    for child in root.children() {
        if let Some(bbox) = child.borrow_data().get_bounding_box() {
            let rb = bbox.to_backend_rect();
            let screen_rect = ctx.cdx_rect_to_screen(&rb);
            ctx.painter.rect_stroke(screen_rect, 0.0, *stroke);
        }
        draw_debug_boxes(child, ctx, stroke);
    }
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

/// Recursively render the tree
fn render_tree<P: crate::renderer::backend::AbstractPainter>(
    root: Node<NodePayload>,
    ctx: &RenderContext<P>,
) {
    let data = root.borrow_data();

    // SegComponent draws its column border via draw_with_node (needs children for bbox).
    // All other types use draw().
    match &*data {
        NodePayload::SegComponent(_) => {
            data.draw_with_node(ctx, &root);
        }
        _ => {
            data.draw(ctx);
        }
    }

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
