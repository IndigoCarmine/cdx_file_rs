# Data Model: Two-Layer Abstraction

## Overview

cdx_file_rs employs a **dual-layer data representation** strategy that balances perfect binary fidelity with developer ergonomics. This design is fundamental to achieving the library's core goals of roundtrip accuracy and type safety.

## The Two Layers

```
┌─────────────────────────────────────────────────┐
│        Typed Domain Layer (High-Level)          │
│  • Strongly-typed structs (Node, Bond, etc.)    │
│  • Named fields with semantic meaning           │
│  • Option<T> for optional properties            │
│  • Developer-friendly API                        │
│  Location: src/cdx/*.rs                         │
└─────────────────────────────────────────────────┘
                     ↕ (via TaggedObject trait)
┌─────────────────────────────────────────────────┐
│         Raw Binary Layer (Low-Level)            │
│  • Generic containers (RawCdxObject)            │
│  • Tag-based property storage                   │
│  • Binary value representation                   │
│  • Format-agnostic, preserves unknowns          │
│  Location: src/cdx_parse_impl/raw_*.rs          │
└─────────────────────────────────────────────────┘
```

## Layer 1: Raw Binary Representation

### Purpose
The raw layer provides a **1:1 mapping** to the CDX file format, treating all objects and properties generically.

### Data Structures

```rust
/// Generic representation of any CDX object
pub struct RawCdxObject {
    pub tag: u16,                    // Object type (e.g., 0x8004 for Node)
    pub id: u32,                     // Unique identifier for cross-references
    pub properties: Vec<RawCdxProperty>,
    pub children: Vec<RawCdxObject>, // Nested objects
}

/// Generic representation of any property
pub struct RawCdxProperty {
    pub tag: u16,                    // Property type (e.g., 0x0402 for element)
    pub value: Vec<u8>,              // Raw binary data
}
```

### Design Rationale

**1. Perfect Roundtrip Fidelity**
- Preserves all data, even unrecognized tags
- Read → Write produces identical bytes
- Critical for forward compatibility

**Example**: If a future version of ChemDraw adds `CDXPROP_ATOM_ORBITAL_TYPE`, this library will:
- Parse it into `RawCdxProperty { tag: 0xXXXX, value: [...] }`
- Preserve it during modifications
- Write it back unchanged

**2. Format-Agnostic Parsing**
- Parser doesn't need to know what every tag means
- New object types require zero parser changes
- Reduces coupling between layers

**3. Error Resilience**
- Malformed properties can be skipped without aborting
- Unknown tags don't cause failures
- Partial file recovery possible

### Limitations
- No type safety: values are raw bytes
- No semantic validation
- Awkward for developers to use directly

## Layer 2: Typed Domain Model

### Purpose
The typed layer provides **developer-friendly structs** with strong typing, named fields, and semantic meaning.

### Example: Node (Atom) Object

```rust
/// Represents a chemical atom or node
pub struct Node {
    pub id: u32,
    
    // Position
    pub position_2d: Option<Point2d>,
    pub position_3d: Option<Point3d>,
    
    // Chemical properties
    pub element: Option<i16>,        // Atomic number (6 = Carbon)
    pub charge: Option<i8>,          // Formal charge
    pub isotope: Option<i16>,        // Mass number
    pub radical: Option<u8>,         // Radical type
    
    // Visual properties  
    pub node_type: Option<i16>,      // Display style
    pub label_display: Option<u8>,   // Show/hide label
    
    // Geometry
    pub geometry: Option<i16>,       // Hybridization
    pub num_hydrogens: Option<u16>,  // Implicit H count
    
    // 50+ more properties...
}
```

### Design Rationale

**1. Type Safety**
```rust
// Compile-time type checking
node.element = Some(6);        // ✅ OK: i16
node.element = Some("C");      // ❌ Compile error: expected i16

// Optional vs. required distinction
node.position_2d               // Option<Point2d> - may be None
node.id                        // u32 - always present
```

**2. Semantic Clarity**
```rust
// Typed domain model (readable)
if let Some(charge) = node.charge {
    println!("Charged atom: {}", charge);
}

// Raw binary layer (obscure)
if let Some(prop) = raw.get_property(0x0423) {
    if let Ok(charge) = i8::decode(&prop.value) {
        println!("Charged atom: {}", charge);
    }
}
```

**3. Documentation via Types**
- Field names document purpose (`isotope` vs. tag `0x0425`)
- Types document valid ranges (`i8` for charge = -128 to 127)
- Optional fields document required vs. optional

**4. Default Values**
```rust
impl Default for Node {
    fn default() -> Self {
        Node {
            id: 0,
            position_2d: None,
            element: Some(6),  // Default to carbon
            charge: None,
            // ...
        }
    }
}
```

## The Bridge: TaggedObject Trait

### Purpose
Convert between raw binary and typed representations bidirectionally.

### Trait Definition

```rust
pub trait TaggedObject: Sized {
    /// Object tag constant (e.g., CDXOBJ_NODE = 0x8004)
    const TAG: u16;
    
    /// Parse from raw binary representation
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError>;
    
    /// Serialize to raw binary representation
    fn to_raw(&self) -> Result<RawCdxObject, CdxError>;
}
```

### Implementation Pattern

**Parsing (from_raw)**:
```rust
impl TaggedObject for Node {
    const TAG: u16 = CDXOBJ_NODE;
    
    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Helper to extract and decode properties
        let position_2d = raw.get_property(CDXPROP_2D_POSITION)
            .and_then(|prop| Point2d::decode(&prop.value).ok());
            
        let element = raw.get_property(CDXPROP_ATOM_ELEMENT)
            .and_then(|prop| i16::decode(&prop.value).ok());
            
        let charge = raw.get_property(CDXPROP_ATOM_CHARGE)
            .and_then(|prop| i8::decode(&prop.value).ok());
        
        // Validation
        if raw.tag != Self::TAG {
            return Err(CdxError::TagMismatch { 
                expected: Self::TAG, 
                found: raw.tag 
            });
        }
        
        Ok(Node {
            id: raw.id,
            position_2d,
            element,
            charge,
            // ... 50+ more fields
        })
    }
    
    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();
        
        // Encode each present property
        if let Some(pos) = self.position_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_POSITION,
                value: pos.encode()?,
            });
        }
        
        if let Some(elem) = self.element {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ELEMENT,
                value: elem.encode()?,
            });
        }
        
        if let Some(chg) = self.charge {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_CHARGE,
                value: chg.encode()?,
            });
        }
        
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),  // Nodes don't have children
        })
    }
}
```

### Binary Codec Trait

For encoding/decoding primitive types:

```rust
pub trait BinaryCodec: Sized {
    fn encode(&self) -> Result<Vec<u8>, CdxError>;
    fn decode(data: &[u8]) -> Result<Self, CdxError>;
}

// Example implementations
impl BinaryCodec for i16 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        buf.write_i16::<LittleEndian>(*self)?;
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        Ok(cursor.read_i16::<LittleEndian>()?)
    }
}

// Custom types: Point2d uses fixed-point encoding
impl BinaryCodec for Point2d {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        // CDX stores coordinates as fixed-point (16 fractional bits)
        let x_fixed = (self.x * 65536.0) as i64;
        let y_fixed = (self.y * 65536.0) as i64;
        
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(x_fixed)?;
        buf.write_i64::<LittleEndian>(y_fixed)?;
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        let x_fixed = cursor.read_i64::<LittleEndian>()?;
        let y_fixed = cursor.read_i64::<LittleEndian>()?;
        
        Ok(Point2d {
            x: (x_fixed as f64) / 65536.0,
            y: (y_fixed as f64) / 65536.0,
        })
    }
}
```

## Tree Structure: Hierarchical Document Model

### Why a Tree?
CDX documents are **inherently hierarchical**:
- Document contains Pages
- Pages contain Fragments
- Fragments contain Nodes and Bonds
- Nodes can contain labels (Text objects)

### dendron Library
We use the `dendron` crate for tree management:

```rust
use dendron::{Tree, Node};

pub struct CdxFile {
    pub tree: Tree<NodePayload>,
}

pub enum NodePayload {
    Document(Document),
    Page(Page),
    Fragment(Fragment),
    Node(Node),
    Bond(Bond),
    Text(TextObject),
    Arrow(Arrow),
    // ... 27 object types total
}
```

### Tree Benefits

**1. Natural Relationships**
```rust
// Parent/child navigation
let page_node = file.tree.root();
for fragment in page_node.children() {
    let frag_data = fragment.borrow_data();
    // Process fragment
}

// Ancestor queries
let atom_node = file.find_node_by_id(42)?;
let fragment = atom_node.parent().unwrap();
let page = fragment.parent().unwrap();
```

**2. Coordinate Propagation**
Child coordinates are relative to parent offsets:
```rust
// Page has BoundsInParent: (100, 200, 300, 400)
// Fragment has position (10, 20)
// Actual screen position = (110, 220)

impl RenderContext {
    pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> BackendPoint2d {
        // Accumulate parent offsets
        let offset = self.calculate_inherited_offset(current_node);
        let x = (cdx_pos.x + offset.x) * self.zoom;
        let y = (cdx_pos.y + offset.y) * self.zoom;
        BackendPoint2d::new(x, y)
    }
}
```

**3. Safe Mutation**
```rust
// Rust's borrow checker enforced by dendron
let mut node_data = node.borrow_data_mut();
node_data.charge = Some(1);  // ✅ OK: exclusive borrow

// Can't access while mutably borrowed
let parent = node.parent();  // ❌ Compile error: already borrowed
```

**4. Tree Operations**
```rust
// Create new node
let new_node = parent.create_as_last_child(grant, NodePayload::Node(node));

// Move node
source.detach(grant);
target.prepend_child(grant, source);

// Delete subtree
node.remove_subtree(grant);
```

## NodePayload Enum: Type-Safe Union

### Purpose
The `NodePayload` enum allows the tree to store **any CDX object type** while maintaining type safety.

### Pattern Matching
```rust
for node in file.tree.descendants(&root) {
    match &*node.borrow_data() {
        NodePayload::Node(atom) => {
            println!("Atom: element={:?}", atom.element);
        }
        NodePayload::Bond(bond) => {
            println!("Bond: {} → {}", bond.begin, bond.end);
        }
        NodePayload::Text(text) => {
            println!("Text: {}", text.text);
        }
        _ => {}  // Other types
    }
}
```

### Macro-Generated Code
To avoid boilerplate, we use macros:

```rust
// In src/cdx/file.rs
define_node_payload!(
    Arrow, Bond, Border, BracketAttachment, BracketedGroup,
    ChemicalProperty, ColorTable, Constraint, /* ... 27 types */
);

// Macro generates:
pub enum NodePayload {
    Arrow(Arrow),
    Bond(Bond),
    Border(Border),
    // ...
}

impl NodePayload {
    pub fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        match raw.tag {
            Arrow::TAG => Ok(NodePayload::Arrow(Arrow::from_raw(raw)?)),
            Bond::TAG => Ok(NodePayload::Bond(Bond::from_raw(raw)?)),
            // ...
        }
    }
    
    pub fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        match self {
            NodePayload::Arrow(obj) => obj.to_raw(),
            NodePayload::Bond(obj) => obj.to_raw(),
            // ...
        }
    }
}
```

## Data Flow Example: Reading a Node

```
1. File bytes: [0x04, 0x80, 0x01, 0x00, 0x00, 0x00, ...]
                 ↓
2. RawCdxParser: Parse tag 0x8004 (CDXOBJ_NODE)
                 ↓
3. RawCdxObject {
     tag: 0x8004,
     id: 1,
     properties: [
       RawCdxProperty { tag: 0x0402, value: [0x06, 0x00] },  // Element = 6
       RawCdxProperty { tag: 0x0423, value: [0x01] },        // Charge = +1
     ]
   }
                 ↓
4. TaggedObject::from_raw()
   - Decode 0x0402 → i16::decode([0x06, 0x00]) → 6
   - Decode 0x0423 → i8::decode([0x01]) → 1
                 ↓
5. Node {
     id: 1,
     element: Some(6),    // Carbon
     charge: Some(1),     // +1 cation
     ...
   }
                 ↓
6. NodePayload::Node(Node { ... })
                 ↓
7. Tree<NodePayload> stored in CdxFile
```

## Advantages of This Design

### 1. Roundtrip Fidelity
- Raw layer preserves unknown tags
- Typed layer doesn't lose data during conversion
- Binary identity maintained: `read(file) |> write() == file`

### 2. Type Safety
- Compile-time checks on property types
- No runtime tag lookups in application code
- Impossible states unrepresentable (`element: Some("C")` won't compile)

### 3. Extensibility
- New properties: Add field to typed struct + update `TaggedObject` impl
- New objects: Add struct + `TaggedObject` impl + `NodePayload` variant
- No changes to parser or tree structure

### 4. Testability
- Raw layer testable independently (binary I/O)
- TaggedObject testable via roundtrip: `obj == from_raw(to_raw(obj))`
- Typed layer testable without file I/O

### 5. Performance
- Zero-copy where possible (property values referenced from file buffer)
- Lazy parsing: Can parse raw without decoding all properties
- Efficient tree navigation via dendron

## Trade-offs and Limitations

### Complexity
- Two representations require maintenance
- TaggedObject implementations repetitive (mitigated by macros/codegen)

### Memory
- Both layers in memory simultaneously during parsing
- Could optimize by streaming (parse on-demand)

### Learning Curve
- Developers must understand both layers
- When to use raw vs. typed? (Answer: almost always typed)

## Best Practices

### When to Use Raw Layer
- Implementing new `TaggedObject`
- Debugging binary format issues
- Preserving unrecognized data

### When to Use Typed Layer
- All application code
- Querying documents
- Rendering
- Validation logic

### Adding New Properties
1. Add field to typed struct
2. Update `Default` implementation
3. Update `from_raw()` to parse property
4. Update `to_raw()` to encode property
5. Write roundtrip test
