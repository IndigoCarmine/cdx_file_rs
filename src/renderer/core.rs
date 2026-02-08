use super::backend::{
    AbstractPainter, Align2 as BackendAlign2, Color as BackendColor, Point2d as BackendPoint2d,
};
use super::egui_backend::EguiBackend;
use crate::cdx::color_table::RGBColor;
use crate::cdx::document::Document;
use crate::cdx::file::CdxFile;
use crate::cdx::file::NodePayload;
use crate::cdx::values::{Point2d as CdxPoint2d, Rectangle};

use dendron::Node;
use eframe::egui::{self, Color32, Painter, Pos2};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RenderStyle {
    pub default_atom_radius: f32,
    pub charge_label_size: f32,
    pub charge_label_offset: f32,
    pub screen_dpi: f32,
    pub arrowhead_size_default: f32,
    pub arrowhead_side_ratio: f32,
    pub arrowhead_min_length: f32,
    pub arrowhead_max_screen: f32,
    pub bracket_lip_ratio: f32,
    pub arc_segment_degrees: f32,
}

impl Default for RenderStyle {
    fn default() -> Self {
        RenderStyle {
            default_atom_radius: 10.0,
            charge_label_size: 8.0,
            charge_label_offset: 8.0,
            screen_dpi: 96.0,
            arrowhead_size_default: 10.0,
            arrowhead_side_ratio: 0.4,
            arrowhead_min_length: 0.01,
            arrowhead_max_screen: 20.0,
            bracket_lip_ratio: 0.05,
            arc_segment_degrees: 5.0,
        }
    }
}
/// Common trait for rendering chemical model objects.
///
/// This trait defines the minimal interface required for drawing
/// domain objects (e.g., Bond, Node, Fragment) onto a 2D canvas.
/// Implementations receive a RenderContext with access to the Painter,
/// coordinate origin, and Document defaults (font, color, etc.).
pub trait Drawable {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>);

    /// Draw with access to the tree node (for objects that need child access)
    /// Default implementation calls draw() for backward compatibility
    fn draw_with_node<P: AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        _node: &Node<NodePayload>,
    ) {
        self.draw(ctx);
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        None
    }
}

#[macro_export]
macro_rules! define_node_renderer {
    (
             $( $ty:ident ),* $(,)?
    ) => {
        impl NodePayload {
            pub fn draw<P: $crate::renderer::backend::AbstractPainter>(&self, ctx: &$crate::renderer::RenderContext<P>) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw(ctx),
                    )*

                }
            }

            pub fn draw_with_node<P: $crate::renderer::backend::AbstractPainter>(&self, ctx: &$crate::renderer::RenderContext<P>, node: &dendron::Node<NodePayload>) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw_with_node(ctx, node),
                    )*
                }
            }

            pub fn get_bounding_box(&self) -> Option<Rectangle> {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.get_bounding_box(),
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
    BracketAttachment,
    BracketedGroup,
    ChemicalProperty,
    ColorTable,
    Constraint,
    CrossReference,
    CrossingBond,
    Curve,
    Document,
    EmbeddedObject,
    Fragment,
    Geometry,
    Graphic,
    Group,
    NamedAlternativeGroup,
    Node,
    ObjectTag,
    Page,
    ReactionScheme,
    ReactionStep,
    RegistryNumber,
    Sequence,
    Spectrum,
    Splitter,
    Table,
    TemplateGrid,
    TextObject,
    TlcLane,
    TLCPlate,
    TLCSpot,
    Annotation,
    UnknownObject802B,
    UnknownObject801D,
    UnknownObject801E,
    UnknownObject801F,
    StoichiometryGrid,
    SegComponent,
    SegDatum,
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

    pub fn to_backend_color(&self) -> BackendColor {
        BackendColor::from_rgb(
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
        let mut node_positions: HashMap<u32, CdxPoint2d> = HashMap::new();
        let tree = &cdx_file.tree;
        let root = tree.root();
        self.collect_node_positions(root, &mut node_positions);

        // Use the pre-calculated center_offset instead of recalculating it
        let egui_backend = EguiBackend::new(painter);
        let ctx = RenderContext::new(
            &egui_backend,
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

    fn render<P: AbstractPainter>(
        &self,
        root: Node<crate::cdx::file::NodePayload>,
        ctx: &RenderContext<P>,
    ) {
        let data = root.borrow_data();

        // Draw the current object with its parent's context and node reference
        // Using draw_with_node allows objects like Table to access their children for grid rendering
        data.draw_with_node(ctx, &root);

        // // Check if this object defines a coordinate offset for its children
        // // Currently only Page objects with BoundsInParent need this
        let child_ctx; // Declare outside to avoid lifetime issues
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
            self.render(child, ctx_ref);
        }
    }

    fn collect_node_positions(
        &self,
        root: Node<crate::cdx::file::NodePayload>,
        node_positions: &mut HashMap<u32, CdxPoint2d>,
    ) {
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
            self.collect_node_positions(child, node_positions);
        }
    }

    /// Calculate auto-scale factor to fit document bounds within window
    /// Returns (scale, center_offset)
    fn calculate_auto_scale(&self, node_positions: &HashMap<u32, CdxPoint2d>) -> (f32, egui::Vec2) {
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
            window_center_y - doc_center_y * auto_scale, // CDX Y increases downward (same as screen)
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
/// - parent_offset: Cumulative offset from parent containers (for relative positioning)
///
/// ## Scaling Structure
/// The RenderContext uses a **two-level scaling system** to transform CDX coordinates to screen coordinates:
///
/// 1. **auto_scale**: Automatically calculated scale factor that fits the entire document
///    within the window while maintaining aspect ratio.
///    - Calculated based on document bounding box and window size
///    - Ensures all content is visible (scale = min(available_width / doc_width, available_height / doc_height))
///    - Applied uniformly to all coordinates
///
/// 2. **zoom**: User-controlled zoom factor for interactive magnification/reduction
///    - Allows users to zoom in/out around the auto-scaled content
///    - Multiplied by auto_scale for final screen scaling
///
/// ## Final Screen Transformation
/// CDX coordinate → Screen coordinate transformation:
/// ```text
/// scale = zoom * auto_scale
/// screen_pos = origin + (cdx_pos + parent_offset) * scale
/// ```
///
/// ## Parent Offset Handling
/// - parent_offset: Cumulative offset from parent objects (in CDX coordinates)
/// - Applied BEFORE scaling to maintain relative positioning within parent containers
/// - Used for Pages with BoundsInParent to establish coordinate system for children
///
/// ## Example Scaling Flow
/// 1. Document has nodes at positions 0-500 in CDX units
/// 2. Window is 800x600 pixels
/// 3. auto_scale calculated as ~1.0 (fits with padding)
/// 4. User zooms to 1.5x: zoom = 1.5
/// 5. Final scale = 1.5 * 1.0 = 1.5
/// 6. Node at CDX 100 → Screen 800/2 + 100*1.5 = 550px
pub struct RenderContext<'a, P: AbstractPainter> {
    pub painter: &'a P,
    pub origin: Pos2,
    pub document: &'a Document,
    pub node_positions: HashMap<u32, CdxPoint2d>,
    /// User-controlled zoom factor for interactive scaling
    /// Multiplied with auto_scale to achieve final screen scale
    pub zoom: f32,
    /// Automatically calculated scale to fit document bounds in window
    /// Computed from: min(available_width / doc_width, available_height / doc_height)
    pub auto_scale: f32,
    /// Cumulative offset from parent objects (in CDX coordinates)
    /// Applied before scaling to maintain relative positioning in parent containers
    pub parent_offset: CdxPoint2d,
    pub style: RenderStyle,
}

impl<'a, P: AbstractPainter> RenderContext<'a, P> {
    /// Create a new rendering context
    pub fn new(
        painter: &'a P,
        origin: Pos2,
        document: &'a Document,
        node_positions: HashMap<u32, CdxPoint2d>,
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
            parent_offset: CdxPoint2d { x: 0.0, y: 0.0 },
            style: RenderStyle::default(),
        }
    }

    pub fn with_style(mut self, style: RenderStyle) -> Self {
        self.style = style;
        self
    }

    /// Create a child rendering context with an additional offset
    ///
    /// This method creates a new RenderContext for child objects with accumulated parent offsets.
    /// Used for parent-child coordinate propagation in container objects (e.g., BoundsInParent for Pages).
    ///
    /// ## Offset Accumulation
    /// - The new parent_offset = current parent_offset + provided offset
    /// - This accumulation ensures that nested containers properly transform their children
    /// - Offsets are in CDX coordinates and applied BEFORE scaling
    ///
    /// ## Usage Pattern
    /// - Page with BoundsInParent creates child context with top-left offset
    /// - Group containers create child context with their local origin
    /// - Nested containers accumulate offsets through the hierarchy
    ///
    /// ## Example with Multiple Levels
    /// ```text
    /// Root context: parent_offset = (0, 0)
    /// Page1 (bounds at 100, 100):
    ///   - Child context: parent_offset = (0, 0) + (100, 100) = (100, 100)
    ///   - Group1 (local origin at 50, 50):
    ///     - Grandchild context: parent_offset = (100, 100) + (50, 50) = (150, 150)
    ///     - Node at CDX(10, 10) renders at CDX(160, 160) in root coordinates
    /// ```
    ///
    /// ## Scaling Note
    /// All scaling (zoom * auto_scale) remains unchanged and is applied uniformly.
    /// The offset accumulation is purely additive in CDX coordinate space.
    pub fn with_offset(&self, offset: &CdxPoint2d) -> Self {
        RenderContext {
            painter: self.painter,
            origin: self.origin,
            document: self.document,
            node_positions: self.node_positions.clone(),
            zoom: self.zoom,
            auto_scale: self.auto_scale,
            parent_offset: CdxPoint2d {
                x: self.parent_offset.x + offset.x,
                y: self.parent_offset.y + offset.y,
            },
            style: self.style.clone(),
        }
    }

    /// Convert CDX coordinates to screen coordinates
    ///
    /// This method performs a complete coordinate transformation using the two-level scaling system:
    ///
    /// ## Transformation Steps
    /// 1. Apply parent offset to CDX position (for relative positioning in parent containers)
    ///    - adjusted_pos = cdx_pos + parent_offset
    /// 2. Scale by combined zoom factor
    ///    - scale = zoom * auto_scale
    /// 3. Translate by origin (window-relative positioning)
    ///
    /// Note: CDX uses the same Y-axis direction as screen coordinates (Y increases downward).
    ///
    /// ## Formula
    /// ```text
    /// adjusted_x = cdx_pos.x + parent_offset.x
    /// adjusted_y = cdx_pos.y + parent_offset.y
    /// scale = zoom * auto_scale
    /// screen_x = origin.x + adjusted_x * scale
    /// screen_y = origin.y + adjusted_y * scale
    /// ```
    ///
    /// ## Examples
    /// - With auto_scale=1.0, zoom=1.0, origin=(400,300):
    ///   CDX(100, 100) → Screen(500, 400)
    /// - With zoom=2.0 (user zoomed in):
    ///   CDX(100, 100) → Screen(600, 500) [twice as far from origin]
    /// - With parent_offset=(50, 50):
    ///   CDX(100, 100) → Screen(550, 450) [offset applied before scaling]
    pub fn cdx_to_screen(&self, cdx_pos: &CdxPoint2d) -> BackendPoint2d {
        let scale = self.zoom * self.auto_scale;
        // Apply parent offset to the CDX position
        let adjusted_x = cdx_pos.x + self.parent_offset.x;
        let adjusted_y = cdx_pos.y + self.parent_offset.y;
        BackendPoint2d::new(
            self.origin.x + (adjusted_x as f32 * scale),
            self.origin.y + (adjusted_y as f32 * scale), // CDX Y increases downward (same as screen)
        )
    }

    pub fn accumulate_children_bounding_box(&self, node: &Node<NodePayload>) -> Option<Rectangle> {
        let mut bbox: Option<Rectangle> = None;

        for child in node.children() {
            let child_bbox = child.borrow_data().get_bounding_box();
            if let Some(child_bbox_inner) = child_bbox {
                if let Some(bbox_inner) = bbox {
                    bbox = Some(bbox_inner.union(&child_bbox_inner));
                } else {
                    bbox = Some(child_bbox_inner);
                }
            }

            let inner_bbox = self.accumulate_children_bounding_box(&child);
            if let Some(inner_bbox_inner) = inner_bbox {
                if let Some(bbox_inner) = bbox {
                    bbox = Some(bbox_inner.union(&inner_bbox_inner));
                } else {
                    bbox = Some(inner_bbox_inner);
                }
            }
        }
        bbox
    }

    /// Convert CDX Rectangle to screen Rect
    pub fn cdx_rect_to_screen(&self, rect: &Rectangle) -> crate::renderer::backend::Rect {
        let top_left = CdxPoint2d {
            x: rect.left,
            y: rect.top,
        };
        let bottom_right = CdxPoint2d {
            x: rect.right,
            y: rect.bottom,
        };
        crate::renderer::backend::Rect::from_two_pos(
            self.cdx_to_screen(&top_left),
            self.cdx_to_screen(&bottom_right),
        )
    }

    /// Convert CDX length to screen length (pixels)
    pub fn cdx_length_to_screen(&self, cdx_length: f64) -> f32 {
        (cdx_length as f32) * self.zoom * self.auto_scale
    }

    /// Convert CDX offset to screen offset (pixels)
    pub fn cdx_offset_to_screen(&self, dx: f64, dy: f64) -> (f32, f32) {
        let scale = self.zoom * self.auto_scale;
        (dx as f32 * scale, dy as f32 * scale)
    }

    /// Get a node position by node id
    pub fn node_position(&self, node_id: u32) -> Option<&CdxPoint2d> {
        self.node_positions.get(&node_id)
    }

    /// Draw text at specified position
    pub fn draw_text(&self, text: &str, pos: BackendPoint2d, color: BackendColor, size: f32) {
        use super::backend::TextSpan;
        let scale = self.zoom * self.auto_scale;
        let scaled_size = size * scale;
        let span = TextSpan::new(text.to_string(), scaled_size, color);
        self.painter
            .rich_text(pos, BackendAlign2::CENTER_CENTER, &[span]);
    }

    /// Draw text at specified position with custom alignment
    pub fn draw_text_with_align(
        &self,
        text: &str,
        pos: BackendPoint2d,
        align: BackendAlign2,
        color: BackendColor,
        size: f32,
    ) {
        use super::backend::TextSpan;
        let scale = self.zoom * self.auto_scale;
        let scaled_size = size * scale;
        let span = TextSpan::new(text.to_string(), scaled_size, color);
        self.painter.rich_text(pos, align, &[span]);
    }

    pub fn default_bond_length(&self) -> f64 {
        self.document.bond_length.unwrap_or(30.0)
    }

    /// Resolve a color index using document color table and fallback
    pub fn resolve_color(&self, color_index: Option<u16>, default: BackendColor) -> BackendColor {
        match color_index {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or(default),
            None => default,
        }
    }

    /// Resolve a possibly signed color index; negatives fallback
    pub fn resolve_color_i16(
        &self,
        color_index: Option<i16>,
        default: BackendColor,
    ) -> BackendColor {
        match color_index {
            Some(idx) if idx >= 0 => self.resolve_color(Some(idx as u16), default),
            _ => default,
        }
    }

    /// Get default foreground color from document or use fallback
    pub fn default_foreground_color(&self) -> BackendColor {
        //color index 3 is default foreground
        match self.document.label_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or(BackendColor::BLACK),
            None => BackendColor::BLACK,
        }
    }
    pub fn default_background_color(&self) -> BackendColor {
        //color index 2 is default background
        match self.document.label_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or(BackendColor::WHITE),
            None => BackendColor::WHITE,
        }
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
    pub fn default_label_color(&self) -> BackendColor {
        match self.document.label_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or(BackendColor::BLACK),
            None => BackendColor::BLACK,
        }
    }

    /// Get default caption size from document or use fallback
    pub fn default_caption_size(&self) -> f32 {
        self.document.caption_size.unwrap_or(10) as f32
    }

    /// Get default caption color from document or use fallback
    pub fn default_caption_color(&self) -> BackendColor {
        match self.document.caption_color {
            Some(idx) => self
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_backend_color())
                .unwrap_or(BackendColor::BLACK),
            None => BackendColor::BLACK,
        }
    }

    pub fn default_stroke(&self) -> super::backend::Stroke {
        use super::backend::Stroke;
        let width = self.cdx_length_to_screen(self.default_line_width());
        Stroke::new(width, self.default_foreground_color())
    }

    pub fn default_align(&self) -> super::backend::Align2 {
        super::backend::Align2::CENTER_CENTER
    }

    pub fn default_font(&self) -> super::backend::FontId {
        use super::backend::{FontFamily, FontId};
        FontId::new(
            self.default_label_size() * self.zoom * self.auto_scale,
            FontFamily::Proportional,
        )
    }

    pub fn default_color(&self) -> super::backend::Color {
        self.default_foreground_color()
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
