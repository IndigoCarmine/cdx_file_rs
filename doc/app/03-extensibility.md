# Extensibility: Extension Points and Patterns

## Overview

cdx_file_rs is designed for **extensibility at multiple levels**. This document describes the primary extension points and patterns for adding new functionality without modifying core library code.

## Extension Point 1: Rendering Backends

### Current State
The library currently renders using **egui** (an immediate-mode GUI library). To support other outputs (SVG, PDF, raster images), we're implementing the **AbstractPainter trait**.

### AbstractPainter Trait

**Location**: `src/renderer/backend.rs`

**Purpose**: Define a backend-agnostic interface for 2D drawing operations

```rust
pub trait AbstractPainter {
    // Lines and shapes
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke);
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color);
    fn circle_stroke(&self, center: Point2d, radius: f32, stroke: Stroke);
    fn rect_filled(&self, rect: Rect, rounding: f32, color: Color);
    fn rect_stroke(&self, rect: Rect, rounding: f32, stroke: Stroke);
    fn rect(&self, rect: Rect, rounding: f32, fill: Color, stroke: Stroke);
    
    // Complex shapes
    fn polyline(&self, points: &[Point2d], stroke: Stroke);
    fn polyline_closed(&self, points: &[Point2d], stroke: Stroke);
    fn convex_polygon(&self, points: &[Point2d], fill: Color);
    
    // Text
    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color);
    fn layout_no_wrap(&self, text: String, font: FontId, color: Color) -> Galley;
    
    // Context
    fn clip_rect(&self) -> Rect;
}
```

### Backend Types

Backend-agnostic types are defined to replace egui-specific types:

```rust
// Backend-agnostic color (RGBA)
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// Backend-agnostic 2D point
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

// Stroke style
pub struct Stroke {
    pub width: f32,
    pub color: Color,
}

// Text alignment
pub enum Align { Left, Center, Right }
pub struct Align2 { pub x: Align, pub y: VerticalAlign }

// Rectangle
pub struct Rect {
    pub min: Point2d,
    pub max: Point2d,
}

// Font specification
pub struct FontId {
    pub size: f32,
    pub family: FontFamily,
}
```

### Implementing a New Backend

**Example: SVG Backend**

```rust
use svg::Document;
use crate::renderer::backend::*;

pub struct SvgBackend {
    document: Document,
    width: f32,
    height: f32,
}

impl AbstractPainter for SvgBackend {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke) {
        let line = svg::node::element::Line::new()
            .set("x1", start.x)
            .set("y1", start.y)
            .set("x2", end.x)
            .set("y2", end.y)
            .set("stroke", format!("rgb({},{},{})", stroke.color.r, stroke.color.g, stroke.color.b))
            .set("stroke-width", stroke.width);
        
        self.document = self.document.clone().add(line);
    }
    
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color) {
        let circle = svg::node::element::Circle::new()
            .set("cx", center.x)
            .set("cy", center.y)
            .set("r", radius)
            .set("fill", format!("rgb({},{},{})", color.r, color.g, color.b));
        
        self.document = self.document.clone().add(circle);
    }
    
    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color) {
        let anchor = match align.x {
            Align::Left => "start",
            Align::Center => "middle",
            Align::Right => "end",
        };
        
        let text_elem = svg::node::element::Text::new()
            .set("x", pos.x)
            .set("y", pos.y)
            .set("text-anchor", anchor)
            .set("font-size", font.size)
            .set("fill", format!("rgb({},{},{})", color.r, color.g, color.b))
            .add(svg::node::Text::new(text));
        
        self.document = self.document.clone().add(text_elem);
    }
    
    // ... implement remaining methods
}

// Usage
pub fn export_to_svg(file: &CdxFile, path: &str) -> Result<(), CdxError> {
    let backend = SvgBackend::new(800.0, 600.0);
    let ctx = RenderContext::new(file, &backend, 1.0, Point2d::new(0.0, 0.0));
    
    // Render all objects
    for node in file.tree.descendants(&file.tree.root()) {
        node.borrow_data().draw(&ctx);
    }
    
    svg::save(path, &backend.document)?;
    Ok(())
}
```

### Migration Status

**✅ Completed**:
- AbstractPainter trait defined
- Backend-agnostic types created
- EguiBackend adapter implemented
- RenderContext generic over painter type

**🚧 In Progress**:
- Converting object renderers to use backend types exclusively
- Removing direct egui dependencies from renderer implementations

**📋 TODO**:
- Implement SVG backend
- Implement PDF backend (via printpdf crate)
- Implement raster backend (via image crate)

## Extension Point 2: Interactive Modes

### ModeHandler Trait

**Location**: `src/mode_handlers/mod.rs`

**Purpose**: Define pluggable interaction handlers for different editing tools

```rust
pub trait ModeHandler {
    /// Handle mouse click
    fn handle_click(&mut self, ctx: &mut ModeContext);
    
    /// Handle mouse drag
    fn handle_drag(&mut self, ctx: &mut ModeContext);
    
    /// Handle hover (for preview rendering)
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter);
    
    /// Handle keyboard input
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool;
}

pub struct ModeContext<'a> {
    pub file: &'a mut CdxFile,
    pub mouse_pos_screen: egui::Pos2,
    pub mouse_pos_cdx: Point2d,
    pub zoom: f32,
    pub offset: Point2d,
    pub is_dragging: bool,
    // ... more context
}
```

### Existing Modes

**ViewMode**: Pan and zoom navigation
- Click and drag: pan view
- Scroll: zoom in/out
- Double-click: reset view

**SelectMode**: Object selection and manipulation
- Click: select object
- Drag: move selected objects
- Shift+click: multi-select
- Delete key: delete selected

**BondMode**: Draw bonds between atoms
- Click atom 1: start bond
- Click atom 2: create bond
- ESC: cancel

**EraserMode**: Delete objects
- Click: delete object under cursor
- Drag: delete multiple objects

### Creating a New Mode

**Example: TextMode for adding text annotations**

```rust
// src/mode_handlers/text_mode.rs
use crate::mode_handlers::{ModeHandler, ModeContext};
use crate::cdx::text::TextObject;
use crate::cdx::values::Point2d;

pub struct TextMode {
    editing_text: Option<String>,
    position: Option<Point2d>,
}

impl TextMode {
    pub fn new() -> Self {
        TextMode {
            editing_text: None,
            position: None,
        }
    }
}

impl ModeHandler for TextMode {
    fn handle_click(&mut self, ctx: &mut ModeContext) {
        // Start text entry at click position
        self.position = Some(ctx.mouse_pos_cdx);
        self.editing_text = Some(String::new());
    }
    
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool {
        if let Some(ref mut text) = self.editing_text {
            match key {
                egui::Key::Enter => {
                    // Finalize text object
                    if let Some(pos) = self.position {
                        let text_obj = TextObject {
                            id: ctx.file.generate_id(),
                            text: text.clone(),
                            position_2d: Some(pos),
                            ..Default::default()
                        };
                        
                        ctx.file.add_text_to_current_fragment(text_obj);
                    }
                    
                    // Reset state
                    self.editing_text = None;
                    self.position = None;
                    true
                }
                egui::Key::Escape => {
                    // Cancel
                    self.editing_text = None;
                    self.position = None;
                    true
                }
                _ => {
                    // Handle character input (done via egui text input)
                    false
                }
            }
        } else {
            false
        }
    }
    
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter) {
        if self.editing_text.is_some() {
            // Draw cursor or preview
            if let Some(pos) = self.position {
                let screen_pos = ctx.cdx_to_screen(&pos);
                painter.circle_filled(
                    screen_pos,
                    3.0,
                    egui::Color32::from_rgb(255, 0, 0),
                );
            }
        }
    }
    
    fn handle_drag(&mut self, _ctx: &mut ModeContext) {
        // Text mode doesn't handle drag
    }
}
```

**Registration in main.rs**:
```rust
enum AppMode {
    View(ViewMode),
    Select(SelectMode),
    Bond(BondMode),
    Eraser(EraserMode),
    Text(TextMode),  // Add new mode
}

// In event handler
if ui.button("Text").clicked() {
    self.mode = AppMode::Text(TextMode::new());
}
```

## Extension Point 3: New CDX Object Types

### Adding a New Object Type

CDX files can contain many object types. Here's how to add support for a new one.

**Example: Adding Support for `Spectrum` Object**

#### Step 1: Define the Struct (`src/cdx/spectrum.rs`)

```rust
use crate::cdx::values::*;

/// Represents a spectrum object (NMR, IR, etc.)
#[derive(Debug, Clone)]
pub struct Spectrum {
    pub id: u32,
    
    // Spectrum properties
    pub spectrum_type: Option<i16>,  // 0=NMR, 1=IR, 2=Mass, etc.
    pub data_points: Option<Vec<f64>>,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
    pub bounding_box: Option<Rectangle>,
    
    // Visual properties
    pub foreground_color: Option<u16>,
    pub background_color: Option<i16>,
}

impl Default for Spectrum {
    fn default() -> Self {
        Spectrum {
            id: 0,
            spectrum_type: None,
            data_points: None,
            x_axis_label: None,
            y_axis_label: None,
            bounding_box: None,
            foreground_color: None,
            background_color: None,
        }
    }
}
```

#### Step 2: Define Tags (`src/cdx_tags/spectrum_tags.rs`)

```rust
// Object tag
pub const CDXOBJ_SPECTRUM: u16 = 0x8024;  // Check CDX spec for actual value

// Property tags
pub const CDXPROP_SPECTRUM_TYPE: u16 = 0x0A01;
pub const CDXPROP_SPECTRUM_DATA: u16 = 0x0A02;
pub const CDXPROP_SPECTRUM_X_LABEL: u16 = 0x0A03;
pub const CDXPROP_SPECTRUM_Y_LABEL: u16 = 0x0A04;
```

#### Step 3: Implement TaggedObject (`src/cdx_parse_impl/spectrum.rs`)

```rust
use crate::cdx::spectrum::Spectrum;
use crate::cdx_parse_impl::{TaggedObject, BinaryCodec, RawCdxObject};
use crate::cdx_tags::spectrum_tags::*;
use crate::error::CdxError;

impl TaggedObject for Spectrum {
    const TAG: u16 = CDXOBJ_SPECTRUM;
    
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let spectrum_type = raw.get_property(CDXPROP_SPECTRUM_TYPE)
            .and_then(|v| i16::decode(&v.value).ok());
        
        let x_axis_label = raw.get_property(CDXPROP_SPECTRUM_X_LABEL)
            .and_then(|v| String::decode(&v.value).ok());
        
        let y_axis_label = raw.get_property(CDXPROP_SPECTRUM_Y_LABEL)
            .and_then(|v| String::decode(&v.value).ok());
        
        // ... decode other properties
        
        Ok(Spectrum {
            id: raw.id,
            spectrum_type,
            x_axis_label,
            y_axis_label,
            // ...
        })
    }
    
    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        
        if let Some(val) = self.spectrum_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_TYPE,
                value: val.encode()?,
            });
        }
        
        if let Some(ref label) = self.x_axis_label {
            properties.push(RawCdxProperty {
                tag: CDXPROP_SPECTRUM_X_LABEL,
                value: label.encode()?,
            });
        }
        
        // ... encode other properties
        
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
```

#### Step 4: Add to NodePayload (`src/cdx/file.rs`)

```rust
define_node_payload!(
    Arrow,
    Bond,
    // ... existing types
    Spectrum,  // Add here
    // ...
);
```

This macro automatically generates:
- `NodePayload::Spectrum(Spectrum)` variant
- `from_raw()` dispatch for `CDXOBJ_SPECTRUM` tag
- `to_raw()` serialization

#### Step 5: Implement Drawable (`src/renderer/spectrum.rs`)

```rust
use crate::renderer::{Drawable, RenderContext};
use crate::renderer::backend::AbstractPainter;
use crate::cdx::spectrum::Spectrum;

impl Drawable for Spectrum {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>) {
        // Render spectrum graph
        if let Some(ref bbox) = self.bounding_box {
            let top_left = ctx.cdx_to_screen(&Point2d { x: bbox.left, y: bbox.top });
            let bottom_right = ctx.cdx_to_screen(&Point2d { x: bbox.right, y: bbox.bottom });
            
            // Draw bounding box
            let rect = Rect::from_min_max(top_left, bottom_right);
            let color = ctx.resolve_color(self.foreground_color, Color::BLACK);
            let stroke = Stroke::new(1.0, color);
            ctx.painter.rect_stroke(rect, 0.0, stroke);
            
            // Draw data points (if present)
            if let Some(ref data) = self.data_points {
                // Convert data points to screen coordinates
                // Draw as polyline
                let points: Vec<Point2d> = data.iter().enumerate()
                    .map(|(i, &y)| {
                        let x = bbox.left + (i as f64) / (data.len() as f64) * (bbox.right - bbox.left);
                        ctx.cdx_to_screen(&Point2d { x, y })
                    })
                    .collect();
                
                ctx.painter.polyline(&points, stroke);
            }
            
            // Draw axis labels
            if let Some(ref x_label) = self.x_axis_label {
                let label_pos = ctx.cdx_to_screen(&Point2d { 
                    x: (bbox.left + bbox.right) / 2.0, 
                    y: bbox.bottom + 10.0 
                });
                ctx.painter.text(
                    label_pos,
                    Align2::CENTER_TOP,
                    x_label,
                    FontId::new(12.0, FontFamily::Proportional),
                    color,
                );
            }
        }
    }
}
```

#### Step 6: Add Renderer to Module (`src/renderer/mod.rs`)

```rust
mod spectrum;  // Add module
```

#### Step 7: Register in Macro (`src/renderer/mod.rs`)

```rust
define_node_renderer!(
    Arrow,
    Bond,
    // ...
    Spectrum,  // Add here
    // ...
);
```

### Testing the New Object

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spectrum_roundtrip() {
        let spectrum = Spectrum {
            id: 1,
            spectrum_type: Some(0),  // NMR
            x_axis_label: Some("Chemical Shift (ppm)".to_string()),
            ..Default::default()
        };
        
        // Serialize
        let raw = spectrum.to_raw().unwrap();
        assert_eq!(raw.tag, CDXOBJ_SPECTRUM);
        
        // Deserialize
        let spectrum2 = Spectrum::from_raw(raw).unwrap();
        assert_eq!(spectrum.spectrum_type, spectrum2.spectrum_type);
        assert_eq!(spectrum.x_axis_label, spectrum2.x_axis_label);
    }
}
```

## Extension Point 4: Custom Property Codecs

### BinaryCodec for Complex Types

If your new object has complex property types, implement `BinaryCodec`:

```rust
// Example: Array of floating-point values
impl BinaryCodec for Vec<f64> {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(self.len() as u32)?;
        for &val in self {
            buf.write_f64::<LittleEndian>(val)?;
        }
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        let len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(cursor.read_f64::<LittleEndian>()?);
        }
        Ok(vec)
    }
}
```

## Extension Point 5: Export Formats

### Adding a New Export Format

Export formats can be added by implementing conversions from `CdxFile`:

**Example: SMILES Export**

```rust
// src/export/smiles.rs
use crate::CdxFile;
use crate::cdx::file::NodePayload;

pub fn export_to_smiles(file: &CdxFile) -> Result<String, CdxError> {
    let mut smiles = String::new();
    
    // Find fragments
    for fragment_node in file.iter_fragments() {
        let fragment = fragment_node.borrow_data();
        
        // Collect atoms
        let atoms: Vec<_> = fragment_node.children()
            .filter_map(|child| {
                match &*child.borrow_data() {
                    NodePayload::Node(atom) => Some(atom),
                    _ => None,
                }
            })
            .collect();
        
        // Collect bonds
        let bonds: Vec<_> = fragment_node.children()
            .filter_map(|child| {
                match &*child.borrow_data() {
                    NodePayload::Bond(bond) => Some(bond),
                    _ => None,
                }
            })
            .collect();
        
        // Convert to SMILES (simplified)
        smiles.push_str(&generate_smiles(&atoms, &bonds)?);
    }
    
    Ok(smiles)
}

fn generate_smiles(atoms: &[&Node], bonds: &[&Bond]) -> Result<String, CdxError> {
    // SMILES generation algorithm
    // ...
}
```

## Best Practices for Extensions

### 1. Follow Existing Patterns
- Study similar existing implementations
- Use the same macro patterns
- Maintain consistent naming conventions

### 2. Add Tests
- Roundtrip tests for `TaggedObject`
- Rendering tests (visual snapshots)
- Integration tests for modes

### 3. Document New Features
- Add doc comments to public APIs
- Update relevant markdown documentation
- Include usage examples

### 4. Maintain Backward Compatibility
- Don't break existing APIs
- Use `Option<T>` for new fields
- Deprecate rather than remove

### 5. Consider Performance
- Avoid unnecessary allocations
- Use references where possible
- Profile rendering code

## Future Extensibility Plans

### Plugin System
**Goal**: Load modes and backends dynamically

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn init(&mut self, ctx: &mut PluginContext);
}

// Dynamic loading
let plugin = load_plugin("libcustom_mode.so")?;
app.register_plugin(plugin);
```

### Custom Object Registry
**Goal**: Register custom object types without modifying core

```rust
pub trait CustomObjectType: TaggedObject + Drawable {
    fn type_name(&self) -> &str;
}

registry.register_custom_type::<MyCustomObject>();
```

### Script Integration
**Goal**: Expose API to scripting languages (Python, Lua)

```rust
use pyo3::prelude::*;

#[pyfunction]
fn load_cdx_file(path: &str) -> PyResult<CdxFile> {
    CdxFile::from_file(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}
```

## Summary

cdx_file_rs provides extensibility through:
1. **AbstractPainter**: Backend-agnostic rendering
2. **ModeHandler**: Pluggable interaction tools
3. **TaggedObject**: Easy addition of CDX object types
4. **BinaryCodec**: Custom property encoding
5. **Export APIs**: Multiple output formats

All extension points follow trait-based patterns, allowing compile-time polymorphism and type safety.
