# CDX File Format Specification

## Overview

The CDX (Chemical Drawing Exchange) format is a proprietary binary file format developed by CambridgeSoft for storing chemical structures, reactions, and related data. This document provides an overview of the format as implemented in cdx_file_rs.

## Format Fundamentals

### Binary Structure
CDX is a **binary, tag-based, hierarchical** format:
- **Binary**: Data stored as bytes (little-endian)
- **Tag-based**: Each element identified by a 16-bit tag ID
- **Hierarchical**: Objects can contain child objects (tree structure)

### Design Goals (per CDX Spec)
1. **Compactness**: Binary encoding for smaller file sizes
2. **Extensibility**: Tag-based system allows new features
3. **Completeness**: Represent all aspects of chemical drawings
4. **Roundtrip Fidelity**: Preserve all information exactly

## File Structure

### High-Level Layout
```
[File Header: 22 bytes fixed]
[Object Tree: variable length]
    [Document Object]
        [Page Object]
            [Fragment Object]
                [Node Objects (atoms)]
                [Bond Objects]
                [Text Objects]
            [Graphic Objects]
        [Color Table]
    [End markers]
```

### File Header (22 bytes)

```
Offset | Size | Description
-------|------|------------
0x00   | 8    | Magic string: "VjCD0100" (identifies file as CDX)
0x08   | 4    | Reserved (usually 0x04040004)
0x0C   | 4    | Reserved (usually 0x00000000)
0x10   | 2    | Reserved (usually 0x0000)
0x12   | 4    | Reserved (usually 0x00000000)
```

**Implementation**:
```rust
pub const CDX_MAGIC: &[u8; 8] = b"VjCD0100";

pub fn parse_header(reader: &mut impl Read) -> Result<(), CdxError> {
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic)?;
    
    if &magic != CDX_MAGIC {
        return Err(CdxError::InvalidMagic);
    }
    
    // Skip reserved fields
    let mut reserved = [0u8; 14];
    reader.read_exact(&mut reserved)?;
    
    Ok(())
}
```

## Tag System

### Tag Structure
Tags are **16-bit unsigned integers** with special meaning for bit 15:
- **Bit 15 = 0**: Property tag (e.g., 0x0402 = atom element)
- **Bit 15 = 1**: Object tag (e.g., 0x8004 = node object)

### Tag Ranges

| Range | Purpose |
|-------|---------|
| 0x0000 - 0x7FFF | Properties (bit 15 = 0) |
| 0x8000 - 0xFFFF | Objects (bit 15 = 1) |

### Common Object Tags

```rust
// Document structure
pub const CDXOBJ_DOCUMENT: u16 = 0x8000;
pub const CDXOBJ_PAGE: u16 = 0x8001;
pub const CDXOBJ_FRAGMENT: u16 = 0x8005;
pub const CDXOBJ_GROUP: u16 = 0x8002;

// Chemical objects
pub const CDXOBJ_NODE: u16 = 0x8004;      // Atom
pub const CDXOBJ_BOND: u16 = 0x8005;      // Chemical bond
pub const CDXOBJ_TEXT: u16 = 0x8010;      // Text label
pub const CDXOBJ_ARROW: u16 = 0x800F;     // Arrow graphic

// Visual objects
pub const CDXOBJ_GRAPHIC: u16 = 0x800E;
pub const CDXOBJ_BORDER: u16 = 0x8021;
pub const CDXOBJ_BRACKETED_GROUP: u16 = 0x8019;

// Supporting objects
pub const CDXOBJ_COLOR_TABLE: u16 = 0x0300;
pub const CDXOBJ_FONT_TABLE: u16 = 0x0100;
pub const CDXOBJ_STYLE_TABLE: u16 = 0x0200;
```

### Common Property Tags

```rust
// Position
pub const CDXPROP_2D_POSITION: u16 = 0x0200;   // Point2d
pub const CDXPROP_3D_POSITION: u16 = 0x0201;   // Point3d

// Atom properties
pub const CDXPROP_ATOM_ELEMENT: u16 = 0x0402;      // INT16 (atomic number)
pub const CDXPROP_ATOM_CHARGE: u16 = 0x0423;       // INT8 (formal charge)
pub const CDXPROP_ATOM_ISOTOPE: u16 = 0x0420;      // INT16 (mass number)
pub const CDXPROP_ATOM_RADICAL: u16 = 0x0421;      // UINT8
pub const CDXPROP_ATOM_NUM_HYDROGENS: u16 = 0x0424;  // UINT16

// Bond properties
pub const CDXPROP_BOND_BEGIN: u16 = 0x0500;        // CDXOBJECTID (start atom)
pub const CDXPROP_BOND_END: u16 = 0x0501;          // CDXOBJECTID (end atom)
pub const CDXPROP_BOND_ORDER: u16 = 0x0502;        // INT16 (1=single, 2=double, etc.)
pub const CDXPROP_BOND_DISPLAY: u16 = 0x0503;      // INT16 (visual style)

// Visual properties
pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204;      // Rectangle
pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0700;  // UINT16 (color table index)
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0701;  // INT16
```

## Object Encoding

### Object Structure
```
[Object Tag: u16]
[Object ID: u32]
[Properties and Children]
[End Object Marker: 0x0000]
```

**Diagram**:
```
Bytes: [04 80] [01 00 00 00] [properties...] [00 00]
        │       │              │              └─ End marker
        │       │              └─ Properties/children
        │       └─ ID = 1
        └─ Tag = 0x8004 (Node)
```

**Implementation**:
```rust
pub struct RawCdxObject {
    pub tag: u16,       // Object type identifier
    pub id: u32,        // Unique ID for cross-references
    pub properties: Vec<RawCdxProperty>,
    pub children: Vec<RawCdxObject>,
}

pub fn parse_object(reader: &mut impl Read) -> Result<RawCdxObject, CdxError> {
    let tag = reader.read_u16::<LittleEndian>()?;
    
    // Check if it's actually an object (bit 15 = 1)
    if tag & 0x8000 == 0 {
        return Err(CdxError::ExpectedObject { found: tag });
    }
    
    let id = reader.read_u32::<LittleEndian>()?;
    
    let mut properties = Vec::new();
    let mut children = Vec::new();
    
    loop {
        let item_tag = reader.read_u16::<LittleEndian>()?;
        
        if item_tag == 0x0000 {
            // End of object
            break;
        } else if item_tag & 0x8000 != 0 {
            // Child object (bit 15 = 1)
            let child = parse_object_with_tag(reader, item_tag)?;
            children.push(child);
        } else {
            // Property (bit 15 = 0)
            let property = parse_property_with_tag(reader, item_tag)?;
            properties.push(property);
        }
    }
    
    Ok(RawCdxObject { tag, id, properties, children })
}
```

## Property Encoding

### Property Structure
```
[Property Tag: u16]
[Length: u16]
[Data: variable bytes]
```

**Special Case**: If `Length == 0xFFFF`, the next 4 bytes are a `u32` containing the actual length (for properties > 65534 bytes).

**Implementation**:
```rust
pub struct RawCdxProperty {
    pub tag: u16,
    pub value: Vec<u8>,
}

pub fn parse_property(reader: &mut impl Read, tag: u16) -> Result<RawCdxProperty, CdxError> {
    let length = reader.read_u16::<LittleEndian>()?;
    
    let actual_length = if length == 0xFFFF {
        // Extended length format
        reader.read_u32::<LittleEndian>()? as usize
    } else {
        length as usize
    };
    
    let mut value = vec![0u8; actual_length];
    reader.read_exact(&mut value)?;
    
    Ok(RawCdxProperty { tag, value })
}
```

## Data Type Encodings

### Primitive Types

| Type | Size (bytes) | Encoding |
|------|--------------|----------|
| INT8 | 1 | Signed 8-bit integer |
| UINT8 | 1 | Unsigned 8-bit integer |
| INT16 | 2 | Signed 16-bit integer (little-endian) |
| UINT16 | 2 | Unsigned 16-bit integer (little-endian) |
| INT32 | 4 | Signed 32-bit integer (little-endian) |
| UINT32 | 4 | Unsigned 32-bit integer (little-endian) |
| FLOAT64 | 8 | IEEE 754 double-precision float |

**Implementation**:
```rust
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
```

### Coordinate Encoding (Fixed-Point)

CDX stores coordinates as **fixed-point integers** with 16 fractional bits:

**Formula**: `cdx_value = float_value * 65536`

**Example**: Position (1.5, 2.25) inches
- x: 1.5 * 65536 = 98304 (stored as i64)
- y: 2.25 * 65536 = 147456 (stored as i64)

**Implementation**:
```rust
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl BinaryCodec for Point2d {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let x_fixed = (self.x * 65536.0) as i64;
        let y_fixed = (self.y * 65536.0) as i64;
        
        let mut buf = Vec::new();
        buf.write_i64::<LittleEndian>(x_fixed)?;
        buf.write_i64::<LittleEndian>(y_fixed)?;
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() != 16 {
            return Err(CdxError::InvalidDataLength { 
                expected: 16, 
                found: data.len() 
            });
        }
        
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

### String Encoding

Strings use **UTF-16 little-endian** with **null terminator**:

```
[Length: u16 (number of UTF-16 code units)]
[UTF-16LE characters]
[0x0000 null terminator]
```

**Implementation**:
```rust
impl BinaryCodec for String {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let utf16: Vec<u16> = self.encode_utf16().collect();
        
        let mut buf = Vec::new();
        for code_unit in utf16 {
            buf.write_u16::<LittleEndian>(code_unit)?;
        }
        buf.write_u16::<LittleEndian>(0)?;  // Null terminator
        
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() % 2 != 0 {
            return Err(CdxError::InvalidStringEncoding);
        }
        
        let mut cursor = Cursor::new(data);
        let mut utf16 = Vec::new();
        
        loop {
            let code_unit = cursor.read_u16::<LittleEndian>()?;
            if code_unit == 0 {
                break;  // Null terminator
            }
            utf16.push(code_unit);
        }
        
        String::from_utf16(&utf16)
            .map_err(|_| CdxError::InvalidStringEncoding)
    }
}
```

### Color Encoding (RGB)

Colors stored as 3 unsigned 16-bit integers (red, green, blue):

```rust
pub struct RGBColor {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl BinaryCodec for RGBColor {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>(self.red)?;
        buf.write_u16::<LittleEndian>(self.green)?;
        buf.write_u16::<LittleEndian>(self.blue)?;
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() != 6 {
            return Err(CdxError::InvalidDataLength { expected: 6, found: data.len() });
        }
        
        let mut cursor = Cursor::new(data);
        Ok(RGBColor {
            red: cursor.read_u16::<LittleEndian>()?,
            green: cursor.read_u16::<LittleEndian>()?,
            blue: cursor.read_u16::<LittleEndian>()?,
        })
    }
}
```

### Object ID Arrays

Some properties reference multiple objects (e.g., bond's endpoints):

```
[Count: u32]
[ID1: u32]
[ID2: u32]
...
```

**Implementation**:
```rust
impl BinaryCodec for Vec<u32> {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(self.len() as u32)?;
        for &id in self {
            buf.write_u32::<LittleEndian>(id)?;
        }
        Ok(buf)
    }
    
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        let count = cursor.read_u32::<LittleEndian>()? as usize;
        
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(cursor.read_u32::<LittleEndian>()?);
        }
        
        Ok(vec)
    }
}
```

## Object Hierarchy

### Typical Document Structure

```
Document (0x8000)
├─ ColorTable (0x0300)
│  └─ [RGB color entries]
├─ Page (0x8001)
│  ├─ Fragment (0x8005)
│  │  ├─ Node (0x8004) - Atom 1
│  │  ├─ Node (0x8004) - Atom 2
│  │  └─ Bond (0x8006) - Connecting atoms 1 & 2
│  ├─ Fragment (0x8005)
│  │  ├─ Node (0x8004) - Atom 3
│  │  └─ Text (0x8010) - Label
│  └─ Graphic (0x800E) - Arrow
└─ Page (0x8001) - Second page
```

### Object Containment Rules

**Document** can contain:
- Pages
- Color tables
- Font tables
- Style tables

**Page** can contain:
- Fragments
- Groups
- Graphics
- Text
- Arrows
- TLC plates

**Fragment** (molecular structure) can contain:
- Nodes (atoms)
- Bonds
- Text (labels)
- Bracketed groups

**Node** can contain:
- Text (atom label)
- Geometry information

## Coordinate Systems

### CDX Units
- **Base unit**: 1/65536 of a point (typography unit)
- **1 point** = 1/72 inch = 65536 CDX units
- **1 inch** = 72 points = 4718592 CDX units

### Coordinate Origin
- **Origin**: Top-left corner of page
- **X-axis**: Increases to the right
- **Y-axis**: Increases downward

### Coordinate Inheritance
Child objects' positions are **relative to parent offsets**:

```rust
// Page has BoundsInParent: (left=100, top=200, right=500, bottom=600)
// Fragment in page has position (10, 20)
// Actual screen position = (110, 220)

pub fn calculate_screen_position(
    child_pos: Point2d,
    parent_offset: Point2d,
    zoom: f32
) -> Point2d {
    Point2d {
        x: (child_pos.x + parent_offset.x) * zoom,
        y: (child_pos.y + parent_offset.y) * zoom,
    }
}
```

## Special Object Types

### Color Table

The color table defines custom colors used throughout the document:

```
ColorTable (0x0300)
├─ Property: Color count (implicit from array size)
└─ Property: RGB color array
   ├─ [R=65535, G=0, B=0]      // Index 0: Red
   ├─ [R=0, G=65535, B=0]      // Index 1: Green
   └─ [R=0, G=0, B=65535]      // Index 2: Blue
```

Other objects reference colors by **index** into this table:
```rust
pub struct Node {
    pub foreground_color: Option<u16>,  // Index into color table
    // ...
}
```

### Bond Order Values

| Value | Meaning |
|-------|---------|
| 1 | Single bond |
| 2 | Double bond |
| 3 | Triple bond |
| 4 | Quadruple bond |
| 0.5 | Half bond (for resonance) |
| 1.5 | One-and-a-half bond (aromatic) |

### Atom Element Numbers

Standard periodic table atomic numbers:
- 1 = Hydrogen
- 6 = Carbon
- 7 = Nitrogen
- 8 = Oxygen
- 15 = Phosphorus
- 16 = Sulfur
- 17 = Chlorine

**Special values**:
- 0 = No specific element (e.g., "R" group)

## CDXML: XML Variant

### Relationship to CDX
CDXML is an **XML representation** of the same data model:
- Same object types
- Same properties
- Same semantics
- Human-readable text format

### Example Comparison

**CDX (binary)**:
```
04 80 01 00 00 00 02 04 06 00 00 00
```

**CDXML (XML)**:
```xml
<n id="1" Element="6"/>
```

Both represent: Node with ID=1, Element=6 (Carbon)

### Current Implementation Status
- **CDX**: ✅ Fully implemented (read/write)
- **CDXML**: 🚧 Not yet implemented (planned)

## Parsing Strategy in cdx_file_rs

### Two-Phase Parsing

**Phase 1: Raw Parsing**
```
Binary bytes → RawCdxObject tree
```
- Generic containers
- No semantic interpretation
- Preserves unknown tags

**Phase 2: Typed Conversion**
```
RawCdxObject tree → Tree<NodePayload> (typed structs)
```
- Type-specific extraction
- Validation
- Default values

### Benefits
- **Roundtrip fidelity**: Raw layer preserves everything
- **Type safety**: Typed layer provides ergonomics
- **Extensibility**: New objects don't affect raw parser

## Validation and Error Handling

### Common Errors

```rust
pub enum CdxError {
    /// File doesn't start with "VjCD0100"
    InvalidMagic,
    
    /// Tag value doesn't match expected object type
    TagMismatch { expected: u16, found: u16 },
    
    /// Property data has wrong length
    InvalidDataLength { expected: usize, found: usize },
    
    /// Referenced object ID not found
    ObjectNotFound { id: u32 },
    
    /// Bond references non-existent atoms
    InvalidBondEndpoints { bond_id: u32 },
    
    /// UTF-16 string decoding failed
    InvalidStringEncoding,
    
    // ... more errors
}
```

### Validation Rules

1. **Required IDs**: All objects (except ColorTable) must have unique IDs
2. **Bond endpoints**: Must reference existing Node objects
3. **Parent/child**: Objects must be in allowed hierarchy
4. **Color references**: Indices must be within color table bounds

## Format Quirks and Edge Cases

### ID 0 is Special
- Some objects use `id = 0` to mean "not set"
- ID 0 should not be used for real objects
- When generating IDs, start from 1

### ColorTable Has No ID
- Unlike other objects, ColorTable doesn't have an `id` field
- Only one color table per document
- Special case in parsing/serialization

### Property Ordering Matters (Sometimes)
- Generally, property order doesn't matter
- But some properties override earlier ones
- Preserve order during roundtrip for safety

### Unknown Tags
- Must be preserved unchanged
- Include in `RawCdxObject` even if not understood
- Critical for forward compatibility

## Tools and Debugging

### Hex Dump Interpretation

Example CDX bytes:
```
56 6A 43 44 30 31 30 30   Magic: "VjCD0100"
04 04 00 04 00 00 00 00   Reserved
00 00 00 00 00 00         Reserved
00 80 01 00 00 00         Object: tag=0x8000 (Document), id=1
00 00                     End of Document object
```

### Debugging Tips

1. **Verify magic string** first (offset 0x00)
2. **Check tag bit 15** (object vs. property)
3. **Follow end markers** (0x0000) to verify structure
4. **Decode coordinates** using fixed-point formula
5. **Compare with CDXML** (if available) to understand structure

### Useful Commands

```bash
# Hex dump of CDX file
xxd molecule.cdx | head -20

# Search for specific tag (e.g., 0x8004 = Node)
xxd molecule.cdx | grep "04 80"

# Count objects
xxd molecule.cdx | grep -c "80 00 00"  # Document objects
```

## References

### Official Specification
- **ChemDraw CDX Specification** (CambridgeSoft documentation)
- Located in: `doc/html/` (HTML version converted to markdown)

### Implementation Files
- `src/cdx_tags/`: Tag constant definitions
- `src/cdx_parse_impl/raw_cdx_parser.rs`: Binary parser
- `src/cdx_parse_impl/codec.rs`: Type encodings

### Verification Reports
- `TAG_VERIFICATION_REPORT.md`: Which tags are implemented
- `CDX_IMPLEMENTATION_VERIFICATION.md`: Parser correctness checks

## Summary

The CDX format is:
- **Binary**: Compact, requires careful byte-level handling
- **Tag-based**: Extensible via 16-bit identifiers
- **Hierarchical**: Tree structure of objects
- **Fixed-point coordinates**: 65536 units per point
- **Little-endian**: All multi-byte integers
- **UTF-16 strings**: With null terminators

cdx_file_rs implements:
- Full binary parsing (raw layer)
- Type-safe domain model (typed layer)
- Perfect roundtrip fidelity
- Unknown tag preservation
- Comprehensive validation

Understanding this format is essential for working with chemical drawing data and extending the library's capabilities.
