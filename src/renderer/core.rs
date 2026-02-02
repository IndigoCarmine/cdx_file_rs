use crate::cdx::color_table::RGBColor;
use crate::cdx::document::Document;
use crate::cdx::file::CdxFile;
use crate::cdx::file::NodePayload;
use crate::cdx::values::Point2d;
use dendron::Node;
use eframe::egui::{self, Color32, Painter, Pos2};
use std::collections::HashMap;
/// Common trait for rendering chemical model objects in egui.
///
/// This trait defines the minimal interface required for drawing
/// domain objects (e.g., Bond, Node, Fragment) onto a 2D egui canvas.
/// Implementations receive a RenderContext with access to the Painter,
/// coordinate origin, and Document defaults (font, color, etc.).
pub trait Drawable {
    fn draw(&self, ctx: &RenderContext);
}

#[macro_export]
macro_rules! define_node_renderer {
    (
             $( $ty:ident ),* $(,)?
    ) => {
        impl NodePayload {
            pub fn draw(&self, ctx: &RenderContext) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw(ctx),
                    )*

                }
            }
        }
    };
}

define_node_renderer!(
    Arrow,
    Bond,
    Border,
    Constraint,
    Document,
    Fragment,
    Geometry,
    Graphic,
    Group,
    Node,
    ObjectTag,
    Page,
    ReactionScheme,
    ReactionStep,
    Table,
    TextObject,
    TlcLane,
    TLCPlate,
    UnknownObject802B,
);

/// High-level CDX renderer with zoom, offset
pub struct CdxRenderer<'a> {
    pub zoom: f32,
    pub offset: egui::Vec2,
    pub center_offset: egui::Vec2, // Auto-calculated center offset
    pub auto_scale: f32,           // Auto-calculated scale factor
    pub cdx_file: &'a CdxFile,
    pub window_size: egui::Vec2,
}

impl<'a> CdxRenderer<'a> {
    /// Create a new CDX renderer with specified zoom and offset
    pub fn new(
        zoom: f32,
        offset: egui::Vec2,
        cdx_file: &'a CdxFile,
        window_size: egui::Vec2,
    ) -> Self {
        CdxRenderer {
            zoom,
            offset,
            center_offset: egui::Vec2::ZERO,
            auto_scale: 1.0,
            cdx_file,
            window_size,
        }
    }

    /// Create a new CDX renderer with center offset and auto_scale calculated
    pub fn with_center_offset(
        zoom: f32,
        offset: egui::Vec2,
        center_offset: egui::Vec2,
        auto_scale: f32,
        cdx_file: &'a CdxFile,
        window_size: egui::Vec2,
    ) -> Self {
        CdxRenderer {
            zoom,
            offset,
            center_offset,
            auto_scale,
            cdx_file,
            window_size,
        }
    }
}

impl RGBColor {
    pub fn to_color32(&self) -> Color32 {
        Color32::from_rgb(
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
    }
}
impl Document {
    pub fn get_color_table(&self) -> Option<&Vec<RGBColor>> {
        self.color_table.as_ref().map(|ct| &ct.colors)
    }
}
impl<'a> CdxRenderer<'a> {
    /// Get color from color table by index
    pub fn get_color(&self, index: usize) -> Color32 {
        let root = self.cdx_file.tree.root();
        let root_node = root.borrow_data();
        if let NodePayload::Document(doc) = &*root_node {
            if let Some(table) = &doc.color_table {
                if index < table.colors.len() {
                    table.colors[index].to_color32()
                } else {
                    Color32::BLACK
                }
            } else {
                Color32::BLACK
            }
        } else {
            Color32::BLACK
        }
    }

    /// Get background color
    pub fn background_color(&self) -> Color32 {
        // self.get_color(0)\
        Color32::WHITE
    }

    /// Get foreground color
    pub fn foreground_color(&self) -> Color32 {
        self.get_color(1)
    }

    /// Render all objects from a CdxFile
    ///
    /// This function traverses the CdxFile tree and renders all visual elements
    /// with the current zoom, offset, and color settings.
    pub fn render_all(&self, painter: &Painter, cdx_file: &crate::cdx::file::CdxFile) {
        // For now, just set background and render a simple placeholder
        let bg_color = self.background_color();
        let document = match cdx_file.get_document() {
            Ok(doc) => doc,
            Err(_) => return, // or handle error appropriately
        };
        let mut node_positions: HashMap<u32, Point2d> = HashMap::new();
        let tree = &cdx_file.tree;
        let root = tree.root();
        self.collect_node_positions(root, &mut node_positions);

        // Use the pre-calculated center_offset instead of recalculating it
        let ctx = RenderContext::new(
            painter,
            Pos2 {
                x: self.center_offset.x + self.offset.x,
                y: self.center_offset.y + self.offset.y,
            },
            &document,
            node_positions,
            self.zoom,
            self.auto_scale, // Use stored auto_scale
        );
        // Fill background
        let rect = painter.clip_rect();
        painter.rect_filled(rect, 0.0, bg_color);

        let root = tree.root();
        self.render(root, &ctx);
    }

    fn render(&self, root: Node<crate::cdx::file::NodePayload>, ctx: &RenderContext) {
        //render
        let data = root.borrow_data();
        
        // Special handling for Table objects to draw grid lines
        if let NodePayload::Table(table) = &*data {
            table.draw(ctx);
            // Draw grid lines based on child Page bounds
            self.draw_table_grid(table, &root, ctx);
        } else {
            data.draw(ctx);
        }
        
        for child in root.children() {
            self.render(child, ctx);
        }
    }

    fn collect_node_positions(
        &self,
        root: Node<crate::cdx::file::NodePayload>,
        node_positions: &mut HashMap<u32, Point2d>,
    ) {
        let data = root.borrow_data();

        if let NodePayload::Node(node_obj) = &*data {
            if let Some(pos) = &node_obj.position_2d {
                node_positions.insert(node_obj.id, pos.clone());
            } else if let Some(pos3d) = &node_obj.position_3d {
                // Try to use 3D position if 2D is not available
                let pos_2d = Point2d {
                    x: pos3d.x,
                    y: pos3d.y,
                };
                node_positions.insert(node_obj.id, pos_2d);
            }
        }

        for child in root.children() {
            self.collect_node_positions(child, node_positions);
        }
    }

    /// Draw grid lines for a Table object based on child Page bounds
    fn draw_table_grid(
        &self,
        table: &crate::cdx::table::Table,
        table_node: &Node<crate::cdx::file::NodePayload>,
        ctx: &RenderContext,
    ) {
        use crate::cdx::values::{Point2d, Rectangle};
        use std::collections::HashSet;

        // Collect bounds_in_parent from all child Page objects
        let mut cell_bounds = Vec::new();
        for child in table_node.children() {
            let child_data = child.borrow_data();
            if let NodePayload::Page(page) = &*child_data {
                if let Some(bounds) = &page.bounds_in_parent {
                    cell_bounds.push(bounds.clone());
                }
            }
        }

        if cell_bounds.is_empty() {
            // No cells with bounds, nothing to draw
            return;
        }

        // Get the line width and color
        let line_width = table
            .line_width
            .unwrap_or_else(|| ctx.default_line_width());
        let scale = ctx.zoom * ctx.auto_scale;
        let stroke_width = (line_width * scale as f64) as f32;

        let color = if let Some(color_idx) = table.foreground_color {
            ctx.document
                .get_color_table()
                .and_then(|ct| ct.get(color_idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(egui::Color32::BLACK)
        } else {
            ctx.default_label_color()
        };

        let stroke = egui::Stroke::new(stroke_width, color);

        // Collect unique x and y coordinates for grid lines
        let mut x_coords = HashSet::new();
        let mut y_coords = HashSet::new();

        for bounds in &cell_bounds {
            x_coords.insert(bounds.left.to_bits());
            x_coords.insert(bounds.right.to_bits());
            y_coords.insert(bounds.top.to_bits());
            y_coords.insert(bounds.bottom.to_bits());
        }

        // Convert back to f64 and sort
        let mut x_sorted: Vec<f64> = x_coords.iter().map(|&bits| f64::from_bits(bits)).collect();
        let mut y_sorted: Vec<f64> = y_coords.iter().map(|&bits| f64::from_bits(bits)).collect();
        x_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if x_sorted.is_empty() || y_sorted.is_empty() {
            return;
        }

        let y_min = *y_sorted.first().unwrap();
        let y_max = *y_sorted.last().unwrap();
        let x_min = *x_sorted.first().unwrap();
        let x_max = *x_sorted.last().unwrap();

        // Draw vertical lines
        for &x in &x_sorted {
            let top = ctx.cdx_to_screen(&Point2d { x, y: y_min });
            let bottom = ctx.cdx_to_screen(&Point2d { x, y: y_max });
            ctx.painter.line_segment([top, bottom], stroke);
        }

        // Draw horizontal lines
        for &y in &y_sorted {
            let left = ctx.cdx_to_screen(&Point2d { x: x_min, y });
            let right = ctx.cdx_to_screen(&Point2d { x: x_max, y });
            ctx.painter.line_segment([left, right], stroke);
        }
    }

    /// Calculate auto-scale factor to fit document bounds within window
    /// Returns (scale, center_offset)
    fn calculate_auto_scale(&self, node_positions: &HashMap<u32, Point2d>) -> (f32, egui::Vec2) {
        if node_positions.is_empty() {
            return (1.0, egui::Vec2::ZERO);
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
            return (1.0, egui::Vec2::ZERO);
        }

        // Calculate scale to fit in window with some padding
        let padding = 50.0;
        let available_width = self.window_size.x - padding * 2.0;
        let available_height = self.window_size.y - padding * 2.0;

        let scale_x = available_width / doc_width as f32;
        let scale_y = available_height / doc_height as f32;
        let auto_scale = scale_x.min(scale_y);

        // Calculate center offset
        let doc_center_x = ((min_x + max_x) / 2.0) as f32;
        let doc_center_y = ((min_y + max_y) / 2.0) as f32;
        let window_center_x = self.window_size.x / 2.0;
        let window_center_y = self.window_size.y / 2.0;

        let center_offset = egui::Vec2::new(
            window_center_x - doc_center_x * auto_scale,
            window_center_y + doc_center_y * auto_scale, // + because Y is inverted
        );

        (auto_scale, center_offset)
    }
}
/// Rendering context that holds the painter, origin, and document defaults
///
/// The RenderContext provides access to:
/// - painter: The egui Painter for drawing
/// - origin: The origin point for coordinate transformation
/// - document: Reference to the Document object containing default settings
///   (font, color, bond length, line width, etc.)
pub struct RenderContext<'a> {
    pub painter: &'a Painter,
    pub origin: Pos2,
    pub document: &'a Document,
    pub node_positions: HashMap<u32, Point2d>,
    pub zoom: f32,
    pub auto_scale: f32,
}

impl<'a> RenderContext<'a> {
    /// Create a new rendering context
    pub fn new(
        painter: &'a Painter,
        origin: Pos2,
        document: &'a Document,
        node_positions: HashMap<u32, Point2d>,
        zoom: f32,
        auto_scale: f32,
    ) -> Self {
        RenderContext {
            painter,
            origin,
            document,
            node_positions,
            zoom,
            auto_scale,
        }
    }

    /// Convert CDX coordinates to screen coordinates
    pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> Pos2 {
        let scale = self.zoom * self.auto_scale;
        Pos2 {
            x: self.origin.x + (cdx_pos.x as f32 * scale),
            y: self.origin.y - (cdx_pos.y as f32 * scale), // CDX uses inverted Y-axis
        }
    }

    /// Get a node position by node id
    pub fn node_position(&self, node_id: u32) -> Option<&Point2d> {
        self.node_positions.get(&node_id)
    }

    /// Draw text at specified position
    pub fn draw_text(&self, text: &str, pos: Pos2, color: Color32, size: f32) {
        let scale = self.zoom * self.auto_scale;
        let scaled_size = size * scale;
        let font_id = egui::FontId::new(scaled_size, egui::FontFamily::Monospace);
        self.painter
            .text(pos, egui::Align2::CENTER_CENTER, text, font_id, color);
    }

    /// Draw text at specified position with custom alignment
    pub fn draw_text_with_align(
        &self,
        text: &str,
        pos: Pos2,
        align: egui::Align2,
        color: Color32,
        size: f32,
    ) {
        let scale = self.zoom * self.auto_scale;
        let scaled_size = size * scale;
        let font_id = egui::FontId::new(scaled_size, egui::FontFamily::Proportional);
        self.painter.text(pos, align, text, font_id, color);
    }

    /// Get default bond length from document or use fallback
    pub fn default_bond_length(&self) -> f64 {
        self.document.bond_length.unwrap_or(30.0)
    }

    /// Get default line width from document or use fallback
    pub fn default_line_width(&self) -> f64 {
        self.document.line_width.unwrap_or(1.0)
    }

    /// Get default bold width from document or use fallback
    pub fn default_bold_width(&self) -> f64 {
        self.document.bold_width.unwrap_or(2.0)
    }

    /// Get default bond spacing from document or use fallback
    pub fn default_bond_spacing(&self) -> i16 {
        self.document.bond_spacing.unwrap_or(18)
    }

    /// Get default label font size from document or use fallback
    pub fn default_label_size(&self) -> f32 {
        self.document.label_size.unwrap_or(10) as f32
    }

    /// Get default label color from document or use fallback
    pub fn default_label_color(&self) -> Color32 {
        match self.document.label_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(Color32::BLACK),
            None => Color32::BLACK,
        }
    }

    /// Get default caption size from document or use fallback
    pub fn default_caption_size(&self) -> f32 {
        self.document.caption_size.unwrap_or(10) as f32
    }

    /// Get default caption color from document or use fallback
    pub fn default_caption_color(&self) -> Color32 {
        match self.document.caption_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(Color32::BLACK),
            None => Color32::BLACK,
        }
    }
}

/// Convert CDX element number to chemical symbol
pub fn element_to_symbol(element: i16) -> String {
    match element {
        1 => "H".to_string(),
        6 => "C".to_string(),
        7 => "N".to_string(),
        8 => "O".to_string(),
        9 => "F".to_string(),
        15 => "P".to_string(),
        16 => "S".to_string(),
        17 => "Cl".to_string(),
        35 => "Br".to_string(),
        53 => "I".to_string(),
        _ => format!("{}", element),
    }
}
