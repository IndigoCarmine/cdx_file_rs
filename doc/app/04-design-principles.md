# Design Principles and Philosophy

## Core Values

cdx_file_rs is built on six fundamental principles that guide all architectural decisions and implementation choices. Understanding these principles is essential for contributing to or extending the library.

## Principle 1: Separation of Concerns

### Statement
> **Each layer has a single, well-defined responsibility with minimal coupling to other layers.**

### Manifestation

**Binary Format Layer**:
- Responsibility: Read/write bytes, nothing more
- Knows: CDX binary encoding rules
- Doesn't know: What tags mean, how to validate, how to render

**Domain Model Layer**:
- Responsibility: Business logic and structure
- Knows: Chemical semantics, relationships, validation rules
- Doesn't know: Binary encoding, rendering details

**Rendering Layer**:
- Responsibility: Visual representation
- Knows: Drawing primitives, layouts, styles
- Doesn't know: File I/O, binary formats, editing logic

### Benefits
- **Independent Evolution**: Change rendering without affecting parsing
- **Testing**: Each layer testable in isolation
- **Reusability**: Domain model usable in non-GUI applications
- **Clarity**: Clear boundaries prevent "spaghetti code"

### Counter-Example (What We Avoid)
```rust
// ❌ BAD: Mixing concerns
pub struct Node {
    pub element: Option<i16>,
    
    // BAD: Rendering logic in domain model
    pub fn draw(&self, painter: &egui::Painter) {
        painter.circle(...);
    }
    
    // BAD: Binary encoding in domain model
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.write_u16(0x8004);
        // ...
    }
}
```

```rust
// ✅ GOOD: Separation via traits
pub struct Node {
    pub element: Option<i16>,
    // Pure data, no behavior
}

// Separate parsing implementation
impl TaggedObject for Node { /* ... */ }

// Separate rendering implementation
impl Drawable for Node { /* ... */ }
```

### Key Insight
> **Data structures should represent "what is," not "how to process."**

## Principle 2: Type Safety Over Flexibility

### Statement
> **Use Rust's type system to make invalid states unrepresentable and errors detectable at compile time.**

### Manifestation

**Typed Properties**:
```rust
// ✅ Type-safe
pub struct Node {
    pub element: Option<i16>,      // Can only be integer
    pub charge: Option<i8>,        // Can only be -128 to 127
    pub position_2d: Option<Point2d>,  // Enforces 2D structure
}

// ❌ Stringly-typed alternative (what we avoid)
pub struct GenericNode {
    properties: HashMap<String, String>,  // Any string key/value
}
```

**Required vs. Optional**:
```rust
pub struct Node {
    pub id: u32,                    // Always present (not Option)
    pub element: Option<i16>,       // May be absent
}

// Impossible to create invalid state
let node = Node {
    id: None,  // ❌ Compile error: expected u32, found Option
    element: Some(6),
};
```

**Enum for Variants**:
```rust
pub enum NodePayload {
    Node(Node),
    Bond(Bond),
    Text(TextObject),
    // ... exhaustive list
}

// Pattern matching is exhaustive
match payload {
    NodePayload::Node(n) => { /* ... */ }
    NodePayload::Bond(b) => { /* ... */ }
    // Compiler warns if we forget a case
}
```

### Benefits
- **Early Error Detection**: Bugs caught at compile time
- **Refactoring Safety**: Compiler guides changes across codebase
- **Documentation**: Types document valid states
- **IDE Support**: Autocomplete knows exact fields available

### Trade-offs
- **Verbosity**: More types to define upfront
- **Learning Curve**: Developers must understand type system
- **Flexibility Loss**: Can't store arbitrary data without extensions

### When to Relax Type Safety
The **raw binary layer** uses flexible types (`Vec<u8>`, generic tags) because:
- Unknown future tags must be preserved
- Roundtrip fidelity requires lossless storage
- Different CDX versions may vary

This is acceptable because it's **isolated** to the lowest layer.

### Key Insight
> **Strong typing at the API surface; flexibility at the file format boundary.**

## Principle 3: Roundtrip Fidelity

### Statement
> **Reading a file and writing it back must produce byte-identical output, even for unrecognized or unimplemented features.**

### Manifestation

**Preserve Unknown Tags**:
```rust
pub struct RawCdxObject {
    pub properties: Vec<RawCdxProperty>,  // Includes unknown properties
    // ...
}

impl TaggedObject for Node {
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract known properties
        let element = raw.get_property(CDXPROP_ATOM_ELEMENT)
            .and_then(|v| i16::decode(&v.value).ok());
        
        // Unknown properties remain in raw.properties
        // They'll be preserved in to_raw()
    }
}
```

**Two-Layer Design**:
- **Raw layer**: Stores everything literally
- **Typed layer**: Interprets known subset
- Conversion preserves unknown data

**Test Requirement**:
```rust
#[test]
fn test_roundtrip() {
    let original_bytes = std::fs::read("test.cdx").unwrap();
    
    // Parse
    let file = CdxFile::from_bytes(&original_bytes).unwrap();
    
    // Modify (only known properties)
    // ...
    
    // Write
    let output_bytes = file.to_bytes().unwrap();
    
    // Assert identical
    assert_eq!(original_bytes, output_bytes);
}
```

### Benefits
- **Forward Compatibility**: Works with newer CDX versions
- **Data Preservation**: No information loss during edit
- **Trustworthy**: Scientists can rely on it for important data
- **Debugging**: Can verify parsing correctness empirically

### Trade-offs
- **Memory Overhead**: Must keep raw representation
- **Complexity**: Two-layer conversion adds code
- **Performance**: Encoding/decoding has overhead

### Why This Matters
ChemDraw files often contain years of research data. Losing any information—even unrecognized metadata—would be unacceptable.

### Key Insight
> **Correctness over cleverness. Preservation over optimization.**

## Principle 4: Extensibility Through Abstraction

### Statement
> **Design for extension via traits and abstractions rather than modification of core code.**

### Manifestation

**Drawable Trait**:
```rust
// Core defines the interface
pub trait Drawable {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>);
}

// New objects implement the trait
impl Drawable for MyCustomObject {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>) {
        // Implementation
    }
}

// No changes to core rendering loop needed
for node in file.tree.descendants(&root) {
    node.borrow_data().draw(&ctx);  // Polymorphic dispatch
}
```

**Backend Abstraction**:
```rust
// Core defines the painter interface
pub trait AbstractPainter {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke);
    // ...
}

// Multiple backends implement it
impl AbstractPainter for EguiBackend { /* ... */ }
impl AbstractPainter for SvgBackend { /* ... */ }
impl AbstractPainter for PdfBackend { /* ... */ }

// Rendering code works with any backend
fn render<P: AbstractPainter>(painter: &P) {
    painter.line_segment(...);
}
```

**Mode Handler System**:
```rust
pub trait ModeHandler {
    fn handle_click(&mut self, ctx: &mut ModeContext);
    // ...
}

// Add new modes without modifying app core
pub struct CustomMode { /* ... */ }
impl ModeHandler for CustomMode { /* ... */ }
```

### Benefits
- **Open/Closed Principle**: Open for extension, closed for modification
- **Plugin Architecture**: Third-party extensions possible
- **Testability**: Mock implementations for testing
- **Decoupling**: Core doesn't depend on specifics

### Contrast: Enum-Based Dispatch (What We Avoid)
```rust
// ❌ BAD: Requires core modification for new types
enum ObjectType {
    Node,
    Bond,
    Text,
    // Adding new type requires editing this enum
}

fn render(obj_type: ObjectType, painter: &Painter) {
    match obj_type {
        ObjectType::Node => { /* ... */ }
        ObjectType::Bond => { /* ... */ }
        ObjectType::Text => { /* ... */ }
        // Adding new type requires editing this match
    }
}
```

### Key Insight
> **Composition over inheritance. Traits over enums for extensibility.**

## Principle 5: Progressive Enhancement

### Statement
> **Implement core functionality first, then layer advanced features. Maintain backward compatibility as features evolve.**

### Manifestation

**Feature Layers**:
1. **Foundation**: Parse/write CDX files (roundtrip fidelity)
2. **Basic Rendering**: Display atoms, bonds, text
3. **Interactivity**: Selection, basic editing
4. **Advanced Rendering**: Z-order, gradients, effects
5. **Full Editor**: Complete drawing tool suite

**Versioning Strategy**:
```rust
// Version 0.1: Basic
pub struct Node {
    pub id: u32,
    pub element: Option<i16>,
}

// Version 0.2: Add field (backward compatible)
pub struct Node {
    pub id: u32,
    pub element: Option<i16>,
    pub isotope: Option<i16>,  // New field, optional
}

// Old code still works
let node = Node {
    id: 1,
    element: Some(6),
    isotope: None,  // Explicit None, or use ..Default::default()
};
```

**Feature Flags** (future):
```toml
[features]
default = ["basic_rendering"]
advanced_rendering = ["z_order", "effects"]
export = ["svg", "pdf"]
scripting = ["pyo3"]
```

### Benefits
- **Incremental Development**: Ship useful features early
- **Stable Foundation**: Core doesn't churn while adding features
- **User Choice**: Enable only needed features
- **Compatibility**: Old code works with new library versions

### Trade-offs
- **Technical Debt**: Early decisions may constrain later features
- **Refactoring**: Sometimes core changes needed (see backend abstraction)

### Current Status
We're in a **refactoring phase**: Abstracting egui dependencies to enable SVG/PDF export. This is acceptable because:
- Core data model remains stable
- Only renderer layer affected
- Public API (CdxFile) unchanged

### Key Insight
> **Make it work, make it right, make it fast—in that order.**

## Principle 6: Documentation as Code

### Statement
> **Design decisions, specifications, and constraints should be discoverable through code structure, types, and inline documentation.**

### Manifestation

**Self-Documenting Types**:
```rust
// Name and type convey meaning
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

// Not this:
pub struct CDXVal {
    pub v1: i64,
    pub v2: i64,
}
```

**Doc Comments**:
```rust
/// Represents a chemical atom or node in a molecular structure.
/// 
/// # Fields
/// - `element`: Atomic number (e.g., 6 for Carbon, 1 for Hydrogen)
/// - `charge`: Formal charge (e.g., +1, -1, 0)
/// - `position_2d`: 2D coordinates in CDX units (65536 units = 1 inch)
pub struct Node {
    // ...
}
```

**Spec Compliance Documentation**:
```rust
// src/cdx_tags/property_tags.rs
/// Atom element property (atomic number)
/// 
/// CDX Spec: Section 4.2.1
/// Type: INT16
/// Range: 1-118 (periodic table)
pub const CDXPROP_ATOM_ELEMENT: u16 = 0x0402;
```

**Verification Reports**:
- `TAG_VERIFICATION_REPORT.md`: Which tags implemented
- `IMPLEMENTATION_GUIDE.md`: How to add features
- `Z_ORDER_IMPLEMENTATION_GUIDE.md`: Planned features

### Benefits
- **Onboarding**: New developers understand design quickly
- **Maintenance**: Future maintainers know "why" not just "what"
- **Compliance**: Clear mapping to CDX specification
- **Quality**: Documentation reviewed alongside code

### Key Insight
> **Code that requires extensive external documentation is poorly designed. Types and names should tell the story.**

## Design Trade-offs and Rationale

### Trade-off 1: Performance vs. Correctness
**Choice**: Prioritize correctness and fidelity over performance

**Rationale**:
- Chemical data is precious; losing information unacceptable
- Files are typically small (< 10 MB)
- Rust's zero-cost abstractions make both achievable

**Example**: We maintain two representations (raw + typed) for roundtrip fidelity, even though it uses more memory.

### Trade-off 2: Type Safety vs. Flexibility
**Choice**: Strong typing in domain layer, flexibility in raw layer

**Rationale**:
- Developer ergonomics matter
- Compiler catches bugs early
- Flexibility isolated to boundary layer

### Trade-off 3: Simplicity vs. Features
**Choice**: Simple core, features via composition

**Rationale**:
- Easy to understand = easy to extend
- Traits enable features without core complexity
- Incremental adoption of advanced features

### Trade-off 4: Immediate Mode (egui) vs. Retained Mode
**Choice**: egui for initial GUI implementation

**Rationale**:
- Faster development for MVP
- Simpler mental model
- Now abstracting to support retained-mode backends (SVG, PDF)

## Technology Choices and Alternatives

### dendron (Tree Library)
**Choice**: Use dendron for tree structure

**Alternatives Considered**:
- Custom tree implementation
- petgraph (graph library)
- id_tree

**Rationale**:
- Rust-native, safe parent/child navigation
- Ownership model enforced
- Minimal dependencies

### eframe/egui (GUI Framework)
**Choice**: egui for viewer application

**Alternatives Considered**:
- iced (elm-style)
- druid (data-first)
- gtk-rs (native widgets)

**Rationale**:
- Immediate mode simplifies state management
- Pure Rust, cross-platform
- Active development
- Now being abstracted for backend independence

### byteorder (Binary I/O)
**Choice**: byteorder crate for endianness

**Alternatives Considered**:
- std::io read/write primitives
- bincode serialization
- serde binary formats

**Rationale**:
- CDX format is specific (little-endian, variable-length)
- Need fine-grained control
- Lightweight dependency

## Anti-Patterns We Avoid

### Anti-Pattern 1: God Objects
**Avoid**: Single object knowing too much
```rust
// ❌ BAD
pub struct EverythingManager {
    pub file_io: FileIO,
    pub parser: Parser,
    pub renderer: Renderer,
    pub validator: Validator,
    // Too many responsibilities
}
```

**Instead**: Separate concerns via modules and traits

### Anti-Pattern 2: Primitive Obsession
**Avoid**: Using primitives for domain concepts
```rust
// ❌ BAD
pub fn set_position(x: f64, y: f64) { /* ... */ }

// ✅ GOOD
pub fn set_position(pos: Point2d) { /* ... */ }
```

### Anti-Pattern 3: Leaky Abstractions
**Avoid**: Implementation details escaping layer boundaries
```rust
// ❌ BAD: Domain model exposing raw bytes
pub struct Node {
    pub raw_data: Vec<u8>,  // Leaks binary format
}

// ✅ GOOD: Clean abstraction
pub struct Node {
    pub element: Option<i16>,
}
```

### Anti-Pattern 4: Stringly-Typed APIs
**Avoid**: Strings where enums/structs appropriate
```rust
// ❌ BAD
pub fn set_style(style: &str) { /* "bold", "italic", ??? */ }

// ✅ GOOD
pub enum FontStyle {
    Bold,
    Italic,
    Underline,
}
pub fn set_style(style: FontStyle) { /* ... */ }
```

## Consistency Guidelines

### Naming Conventions
- **Modules**: lowercase, underscore-separated (`cdx_parse_impl`)
- **Types**: PascalCase (`Node`, `RawCdxObject`)
- **Functions**: snake_case (`from_raw`, `to_raw`)
- **Constants**: SCREAMING_SNAKE_CASE (`CDXOBJ_NODE`)

### Error Handling
- Use `Result<T, CdxError>` for fallible operations
- Provide context in error messages
- Don't panic in library code

### Testing
- Unit tests in same file as implementation
- Integration tests in `tests/` directory
- Roundtrip tests for all `TaggedObject` implementations

### Documentation
- Public APIs must have doc comments
- Include examples in doc comments
- Link to CDX specification where applicable

## Evolution of Design Principles

### Original Vision (v0.1)
- Parse CDX files
- Basic rendering
- egui-based viewer

### Current Reality (v0.2)
- Robust parsing with roundtrip fidelity
- Functional rendering (egui-dependent)
- Interactive editing modes
- **In progress**: Backend abstraction

### Future Direction (v1.0)
- Complete backend abstraction
- Multiple export formats (SVG, PDF)
- Advanced rendering (Z-order, effects)
- Comprehensive editing tools
- Stable public API

### Lessons Learned
1. **Early abstraction pays off**: Drawable trait made renderer iteration easy
2. **Late abstraction is expensive**: egui coupling harder to remove later
3. **Two-layer model essential**: Enabled perfect roundtrip fidelity
4. **Type safety catches bugs**: Prevented many runtime errors
5. **Documentation matters**: Onboarding new contributors much easier

## Philosophical Stance

### On Complexity
> **Complexity is inherent in the domain (chemistry + binary formats). Our job is to tame it through clear abstractions, not add accidental complexity.**

### On Correctness
> **Chemical research data is irreplaceable. Correctness trumps all other concerns.**

### On User Experience
> **Library users are developers. The API should feel natural, predictable, and type-safe.**

### On Open Source
> **Design for contributors. Clear architecture and documentation lower the barrier to participation.**

## Summary: The cdx_file_rs Way

1. **Separate concerns** rigorously across layers
2. **Trust the type system** to catch errors early
3. **Preserve data** perfectly, even when unknown
4. **Extend via traits**, not by modifying core
5. **Build incrementally**, maintain compatibility
6. **Document through code**, supplement with prose

These principles guide every design decision, from high-level architecture to variable naming. When in doubt, refer back to these core values.
