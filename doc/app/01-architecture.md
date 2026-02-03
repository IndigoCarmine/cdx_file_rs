# Architecture: Layered Design

## Overview

cdx_file_rs follows a **strict layered architecture** where each layer has clear responsibilities and dependencies flow downward. Higher layers depend on lower layers, but not vice versa.

## Layer Diagram

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 6: Application Layer (main.rs)                       │
│  ─────────────────────────────────────────────────────────  │
│  • GUI application using eframe/egui                         │
│  • Event handling (mouse, keyboard)                          │
│  • Application state management                              │
│  • Mode coordination (view/select/bond/eraser)               │
│  Dependencies: Layer 5 (mode handlers), Layer 4 (renderer)  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 5: Mode Handler Layer (src/mode_handlers/)           │
│  ─────────────────────────────────────────────────────────  │
│  • ModeHandler trait for pluggable interactions             │
│  • SelectMode: object selection and manipulation            │
│  • BondMode: bond drawing between atoms                     │
│  • EraserMode: object deletion                              │
│  • DebugMode: development tools                             │
│  Dependencies: Layer 3 (domain model), Layer 4 (renderer)   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 4: Rendering Layer (src/renderer/)                   │
│  ─────────────────────────────────────────────────────────  │
│  • Drawable trait (object → visual representation)          │
│  • AbstractPainter trait (backend abstraction)              │
│  • RenderContext (zoom, offset, styles, color resolution)   │
│  • EguiBackend implementation                               │
│  • Object-specific renderers (node.rs, bond.rs, etc.)       │
│  Dependencies: Layer 3 (domain model), backend interface    │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 3: Domain Model Layer (src/cdx/)                     │
│  ─────────────────────────────────────────────────────────  │
│  • CdxFile: high-level API for file operations              │
│  • Tree<NodePayload>: hierarchical document structure       │
│  • Typed structs: Node, Bond, Text, Arrow, Page, etc.       │
│  • Business logic: querying, validation, ID generation       │
│  • Coordinate transformations and inheritance                │
│  Dependencies: Layer 2 (parsing), dendron (tree library)    │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 2: Parsing/Serialization (src/cdx_parse_impl/)       │
│  ─────────────────────────────────────────────────────────  │
│  • TaggedObject trait: from_raw() / to_raw()                │
│  • BinaryCodec trait: encode() / decode() for primitives    │
│  • Type conversions: RawCdxObject ↔ typed structs           │
│  • Property extraction and validation                        │
│  • Error handling for malformed data                         │
│  Dependencies: Layer 1 (raw objects), Layer 0 (tags)        │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Binary Format Layer (RawCdxObject)                │
│  ─────────────────────────────────────────────────────────  │
│  • RawCdxParser: reads binary file → RawCdxObject tree      │
│  • CdxWriter: writes RawCdxObject tree → binary file        │
│  • RawCdxObject: generic container (tag, id, props, kids)   │
│  • RawCdxProperty: binary property (tag, value bytes)       │
│  • Little-endian encoding/decoding                           │
│  Dependencies: Layer 0 (tags), byteorder crate              │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 0: Tag Definitions (src/cdx_tags/)                   │
│  ─────────────────────────────────────────────────────────  │
│  • Object tag constants (CDXOBJ_NODE, CDXOBJ_BOND, ...)     │
│  • Property tag constants (CDXPROP_ATOM_ELEMENT, ...)       │
│  • Organized by category (node_tags, bond_tags, etc.)       │
│  Dependencies: None (foundational constants)                 │
└─────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### Layer 0: Tag Definitions
**Purpose**: Define the vocabulary of the CDX format

**Files**: `src/cdx_tags/*.rs`

**Responsibilities**:
- Declare tag constants matching CDX specification
- Group related tags by object/property type
- Provide documentation for tag meanings

**Key Characteristics**:
- No dependencies on other layers
- Pure constants (no logic)
- Single source of truth for tag values

**Example**:
```rust
// Object tags (bit 15 = 1)
pub const CDXOBJ_NODE: u16 = 0x8004;
pub const CDXOBJ_BOND: u16 = 0x8005;

// Property tags (bit 15 = 0)
pub const CDXPROP_ATOM_ELEMENT: u16 = 0x0402;
pub const CDXPROP_ATOM_CHARGE: u16 = 0x0423;
```

### Layer 1: Binary Format Layer
**Purpose**: Raw binary I/O with no semantic interpretation

**Files**: `src/cdx_parse_impl/raw_cdx_parser.rs`, `src/cdx_parse_impl/raw_cdx_writer.rs`

**Responsibilities**:
- Read CDX file bytes into generic `RawCdxObject` tree
- Write `RawCdxObject` tree to CDX file bytes
- Handle binary encoding (little-endian, variable-length properties)
- Preserve unknown/unrecognized tags

**Key Characteristics**:
- Format-agnostic: doesn't interpret tag meanings
- Lossless: preserves all data, even unknown tags
- Error handling for corrupted files

**Data Structures**:
```rust
pub struct RawCdxObject {
    pub tag: u16,                    // Object type identifier
    pub id: u32,                     // Unique ID for cross-references
    pub properties: Vec<RawCdxProperty>,
    pub children: Vec<RawCdxObject>,
}

pub struct RawCdxProperty {
    pub tag: u16,                    // Property type identifier
    pub value: Vec<u8>,              // Raw binary data
}
```

### Layer 2: Parsing/Serialization Layer
**Purpose**: Convert between raw binary and typed domain objects

**Files**: `src/cdx_parse_impl/*.rs` (parallel to `src/cdx/*.rs`)

**Responsibilities**:
- Implement `TaggedObject` trait for each domain type
- Decode binary property values using `BinaryCodec`
- Validate data (e.g., required properties present)
- Handle optional vs. required fields
- Provide sensible defaults

**Key Characteristics**:
- Bidirectional: `from_raw()` for parsing, `to_raw()` for serialization
- Type-safe: leverages Rust's type system
- Validation layer: checks semantic constraints

**Pattern**:
```rust
impl TaggedObject for Node {
    const TAG: u16 = CDXOBJ_NODE;
    
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let position_2d = raw.get_property(CDXPROP_2D_POSITION)
            .and_then(|v| Point2d::decode(v).ok());
        let element = raw.get_property(CDXPROP_ATOM_ELEMENT)
            .and_then(|v| i16::decode(v).ok());
        // ... extract all properties
        Ok(Node { position_2d, element, ... })
    }
    
    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        if let Some(val) = self.position_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_POSITION,
                value: val.encode()?,
            });
        }
        // ... encode all properties
        Ok(RawCdxObject { tag: Self::TAG, id: self.id, properties, children })
    }
}
```

### Layer 3: Domain Model Layer
**Purpose**: Business logic and high-level API

**Files**: `src/cdx/*.rs`, `src/cdx/file.rs`

**Responsibilities**:
- Define typed structs for all CDX objects
- Maintain document tree structure using `dendron`
- Provide query methods (find nodes, bonds, etc.)
- Generate unique IDs
- Handle coordinate transformations
- Validate relationships (e.g., bond endpoints exist)
- Offer convenience methods for common operations

**Key Characteristics**:
- Strong typing: `Option<T>` for optional properties
- Tree-based: hierarchical parent/child relationships
- API-focused: designed for ergonomic use
- Business rules: enforces CDX semantics

**Core Type**:
```rust
pub struct CdxFile {
    pub tree: Tree<NodePayload>,
}

pub enum NodePayload {
    Document(Document),
    Page(Page),
    Fragment(Fragment),
    Node(Node),
    Bond(Bond),
    // ... 27 object types
}

impl CdxFile {
    pub fn from_file(path: &str) -> Result<Self, CdxError>;
    pub fn write_to_file(&self, path: &str) -> Result<(), CdxError>;
    pub fn get_document(&self) -> Result<&Document, CdxError>;
    pub fn iter_nodes(&self) -> impl Iterator<Item = &Node>;
    pub fn find_node_by_id(&self, id: u32) -> Option<&Node>;
    // ... many more methods
}
```

### Layer 4: Rendering Layer
**Purpose**: Visual representation of domain objects

**Files**: `src/renderer/*.rs`

**Responsibilities**:
- Define `Drawable` trait for renderable objects
- Implement `draw()` methods for each object type
- Manage `RenderContext` (zoom, offset, styles)
- Abstract backend via `AbstractPainter` trait
- Provide color resolution and coordinate conversion
- Handle text layout and formatting

**Key Characteristics**:
- Decoupled from domain: domain objects don't know about rendering
- Backend-agnostic (in progress): supports multiple rendering targets
- Context-based: rendering decisions use shared context
- Extensible: new objects implement `Drawable`

**Traits**:
```rust
pub trait Drawable {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>);
    
    // Optional: for objects needing tree access
    fn draw_with_node<P: AbstractPainter>(
        &self, 
        ctx: &RenderContext<P>, 
        node: &Node<NodePayload>
    ) {
        self.draw(ctx);
    }
}

pub trait AbstractPainter {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke);
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color);
    fn text(&self, pos: Point2d, align: Align2, text: &str, font: FontId, color: Color);
    // ... more drawing primitives
}
```

### Layer 5: Mode Handler Layer
**Purpose**: Interactive editing behaviors

**Files**: `src/mode_handlers/*.rs`

**Responsibilities**:
- Define `ModeHandler` trait for tool implementations
- Handle user input (click, drag, hover, keyboard)
- Modify document based on interactions
- Provide visual feedback during operations
- Maintain mode-specific state

**Key Characteristics**:
- Pluggable: modes are swappable at runtime
- Stateful: each mode tracks its own state
- Event-driven: responds to user actions
- Renderer-aware: can draw overlays

**Pattern**:
```rust
pub trait ModeHandler {
    fn handle_click(&mut self, ctx: &mut ModeContext);
    fn handle_drag(&mut self, ctx: &mut ModeContext);
    fn handle_hover(&self, ctx: &ModeContext, painter: &egui::Painter);
    fn handle_key(&mut self, ctx: &mut ModeContext, key: egui::Key) -> bool;
}

pub struct BondMode {
    start_node: Option<u32>,  // ID of first atom
    temp_position: Option<Point2d>,
}
```

### Layer 6: Application Layer
**Purpose**: GUI application and event loop

**Files**: `src/main.rs`

**Responsibilities**:
- Initialize egui application
- Manage application state
- Coordinate mode handlers
- Handle file I/O (open, save)
- Provide UI controls (zoom, mode selection)
- Render document via Layer 4

**Key Characteristics**:
- Framework-specific: uses eframe/egui
- Top-level orchestration: ties all layers together
- UI logic: menus, panels, dialogs
- Can be replaced: library usable without GUI

## Data Flow Examples

### Reading a File
```
User: Open "molecule.cdx"
   → Application Layer: File dialog
   → Layer 1: RawCdxParser reads bytes → RawCdxObject tree
   → Layer 2: TaggedObject::from_raw() → typed domain objects
   → Layer 3: CdxFile::from_raw() → Tree<NodePayload>
   → Application Layer: Store in app state
   → Layer 4: Drawable::draw() → rendered on screen
```

### User Interaction (Bond Drawing)
```
User: Click atom A
   → Application Layer: egui event
   → Layer 5: BondMode::handle_click()
   → Layer 3: CdxFile::find_node_at_position()
   → BondMode: Store atom A ID

User: Click atom B
   → Layer 5: BondMode::handle_click()
   → Layer 3: CdxFile::create_bond(A, B)
   → BondMode: Clear state
   → Layer 4: Redraw with new bond
```

### Writing a File
```
User: Save file
   → Application Layer: File dialog
   → Layer 3: CdxFile (Tree<NodePayload>)
   → Layer 2: TaggedObject::to_raw() → RawCdxObject tree
   → Layer 1: CdxWriter writes RawCdxObject → bytes
   → File system: Write "output.cdx"
```

## Dependency Rules

### Strict Rules (Enforced by Architecture)
1. **Downward Dependencies Only**: Layer N may depend on Layer N-1, N-2, etc., but never Layer N+1
2. **No Circular Dependencies**: Rendering doesn't depend on modes, domain doesn't depend on rendering
3. **Interface Boundaries**: Layers communicate through well-defined traits/APIs

### Rationale
- **Testability**: Lower layers can be tested without higher layers
- **Replaceability**: GUI can be swapped without changing domain logic
- **Clarity**: Data flow is predictable and unidirectional
- **Maintenance**: Changes in one layer have limited blast radius

## Key Architectural Decisions

### Decision 1: Two-Layer Data Representation (Raw + Typed)
**Rationale**: 
- Raw layer enables perfect roundtrip fidelity
- Typed layer provides developer ergonomics
- Separation allows parsing unknown/future tags

### Decision 2: Separate Rendering Layer
**Rationale**:
- Domain objects remain pure data (no rendering logic)
- Enables multiple backends (GUI, SVG, PDF)
- Rendering can evolve independently

### Decision 3: Tree Structure via Dendron
**Rationale**:
- CDX documents are inherently hierarchical
- Dendron provides safe parent/child navigation
- Rust's ownership model enforced by library

### Decision 4: Trait-Based Extensibility
**Rationale**:
- `Drawable`, `TaggedObject`, `ModeHandler` enable plugins
- New features don't require core changes
- Compile-time polymorphism (zero-cost abstraction)

## Evolution and Future Directions

### Current Limitations
- **Backend Abstraction Incomplete**: Some rendering code still egui-specific
- **No Render Passes**: Z-order/layering not implemented
- **Limited Interaction Traits**: `Drawable` missing hit-testing methods

### Planned Improvements
- **Complete AbstractPainter**: Fully backend-agnostic rendering
- **Render Pipeline**: Multi-pass rendering (background → objects → selection → overlays)
- **Extended Drawable**: Add `bounding_box()`, `hit_test()`, `draw_selected()`
- **Plugin System**: Dynamic mode handler loading

### Stability Guarantees
- **Layer 0-3**: Stable public API (semantic versioning)
- **Layer 4**: API stabilizing (backend abstraction in progress)
- **Layer 5-6**: Application-specific (may change frequently)
