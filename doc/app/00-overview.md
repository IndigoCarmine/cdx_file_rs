# cdx_file_rs: Overview

## Purpose

**cdx_file_rs** is a pure Rust library for reading, writing, and rendering ChemDraw CDX (Chemical Drawing Exchange) binary files. It provides both low-level file format handling and high-level APIs for building chemical drawing applications.

## What Problem Does It Solve?

The CDX file format is a proprietary binary format used by ChemDraw and related chemistry software to store chemical structures, reactions, and annotations. This library enables:

1. **File Format Access**: Parse and generate CDX files without ChemDraw dependencies
2. **Chemical Structure Manipulation**: Programmatically read, modify, and create molecular structures
3. **Cross-Platform Visualization**: Render chemical drawings on any platform supporting Rust
4. **Data Preservation**: Maintain perfect roundtrip fidelity (read → modify → write preserves all data)
5. **Application Foundation**: Serve as the core for chemistry software applications

## Who Should Use This Library?

### As a Library (Parsing/Generation)
- Developers building chemistry software that needs CDX compatibility
- Scientists automating chemical structure processing
- Data pipeline developers converting between chemical file formats
- Tools that need to extract information from ChemDraw files

### As an Application (Interactive Viewer/Editor)
- Chemistry educators needing a cross-platform drawing tool
- Researchers reviewing and annotating chemical structures
- Developers prototyping chemical drawing features

## High-Level Capabilities

### ✅ Fully Implemented
- **File I/O**: Read and write CDX binary files with perfect roundtrip fidelity
- **Core Objects**: Document, Page, Fragment, Node (atom), Bond, Text, Arrow
- **Rendering**: Visual display of molecules, bonds, text annotations, arrows
- **Tree Structure**: Hierarchical document model with parent/child relationships
- **Color Tables**: Full support for custom color palettes
- **Coordinate Systems**: 2D/3D positioning with inherited transformations
- **Interactive Viewer**: GUI application with zoom, pan, selection

### ⚠️ Partially Implemented
- **Advanced Graphics**: Basic line/rectangle/oval rendering (some properties missing)
- **TLC (Thin Layer Chromatography)**: Parsing complete, rendering basic
- **Bracketed Groups**: Structure parsing works, visual rendering incomplete
- **Editing Tools**: Selection and bond drawing modes functional, others in progress

### 🚧 Planned Features
- **Backend Abstraction**: Currently egui-specific, abstracting to support SVG/PDF export
- **Z-Order/Layering**: Proper rendering order for overlapping objects
- **Extended Editing**: Full suite of drawing tools (text, shapes, arrows)
- **CDXML Support**: XML variant of CDX format

## Quick Start: Understanding the Codebase

### Directory Structure
```
src/
├── cdx/                    # Domain model (typed structs for CDX objects)
│   ├── node.rs            # Atom/Node definition
│   ├── bond.rs            # Chemical bond
│   ├── text.rs            # Text annotations
│   └── ...                # ~27 object types
├── cdx_parse_impl/        # Binary parsing (RawCdxObject ↔ typed structs)
│   ├── node.rs            # Node parsing/serialization
│   └── ...                # Parallel to cdx/ files
├── cdx_tags/              # Tag constant definitions
│   ├── object_tags.rs     # Object type IDs
│   └── property_tags.rs   # Property type IDs
├── renderer/              # Rendering implementation
│   ├── backend.rs         # Backend abstraction traits
│   ├── egui_backend.rs    # egui-specific implementation
│   ├── node.rs            # Atom rendering
│   └── ...                # Parallel to cdx/ files
├── mode_handlers/         # Interactive editing modes
│   ├── select_mode.rs     # Object selection
│   ├── bond_mode.rs       # Bond drawing
│   └── ...                # Additional tools
├── lib.rs                 # Public API surface
└── main.rs                # GUI application entry point
```

### Key Modules

| Module | Purpose | Read First |
|--------|---------|------------|
| `cdx/` | Domain model definitions | `node.rs`, `bond.rs` |
| `cdx_parse_impl/` | Binary encoding/decoding | `codec.rs`, `tagged_object.rs` |
| `renderer/` | Visual display | `core.rs`, `backend.rs` |
| `mode_handlers/` | Interactive editing | `select_mode.rs` |

### Conceptual Model

```
User Application
       ↓
  CdxFile API (lib.rs)
       ↓
  Tree<NodePayload>        ← Document hierarchy
       ↓
  Domain Objects (cdx/)    ← Typed structs
       ↓
  TaggedObject Trait       ← Parsing layer
       ↓
  RawCdxObject             ← Generic binary representation
       ↓
  Binary File (CDX)
```

### Typical Usage Patterns

**Reading a file:**
```rust
use cdx_file_rs::CdxFile;

let file = CdxFile::from_file("molecule.cdx")?;
let document = file.get_document()?;
println!("Title: {:?}", document.title);

// Query atoms
for node in file.iter_nodes() {
    println!("Atom: element={:?} position={:?}", 
             node.element, node.position_2d);
}
```

**Creating a file:**
```rust
let mut file = CdxFile::new();
let doc_node = file.create_document();
let page_node = file.create_page_in_document(&doc_node);
let fragment_node = file.create_fragment_in_page(&page_node);

// Add atoms
let atom1 = file.create_node_in_fragment(&fragment_node, Node {
    element: Some(6), // Carbon
    position_2d: Some(Point2d { x: 0.0, y: 0.0 }),
    ..Default::default()
});

file.write_to_file("output.cdx")?;
```

**Rendering:**
```rust
// Via GUI application
cargo run -- view molecule.cdx

// Programmatically (future: backend abstraction)
let render_ctx = RenderContext::new(document, painter, zoom, offset);
for node in file.tree.descendants(&page_node) {
    node.borrow_data().draw(&render_ctx);
}
```

## Design Philosophy in One Sentence

> **cdx_file_rs separates concerns into layers (binary format → typed domain model → rendering) with strong type safety and extensibility, prioritizing roundtrip fidelity and clean abstractions.**

## Next Steps

- **For Architecture**: Read [01-architecture.md](01-architecture.md) for layer responsibilities
- **For Data Model**: Read [02-data-model.md](02-data-model.md) for type system design
- **For Extension**: Read [03-extensibility.md](03-extensibility.md) for adding features
- **For Philosophy**: Read [04-design-principles.md](04-design-principles.md) for core values
- **For Format Details**: Read [05-file-format.md](05-file-format.md) for CDX specification
