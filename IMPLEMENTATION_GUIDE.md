# CDX Object Implementation Guide

This guide provides step-by-step instructions for implementing any CDX object struct with the `TaggedObject` trait.

## Overview

Each CDX object requires:
1. A Rust struct with all properties (required and optional)
2. Implementation of the `TaggedObject` trait
3. A `from_raw()` method to parse `RawCdxObject` into the typed struct
4. Module registration in `mod.rs`

## Step-by-Step Instructions

### Step 1: Gather Object Specification

1. Locate the object specification document in `doc/[object_name].txt`
2. Identify:
   - Object CDX Constant Name (e.g., `kCDXObj_Bond`)
   - CDX Constant Value (e.g., `0x8005`)
   - Required properties (marked as "Required")
   - Optional properties with their property IDs and data types

### Step 2: Review Tag Definitions

1. Open `src/cdx_tags/[object_name]_tags.rs` (or similar naming)
2. Verify all property constants are defined:
   - Object tag: `CDXOBJ_[OBJECTNAME]`
   - Property tags: `CDXPROP_[PROPERTY_NAME]`
3. Note the exact constant names for use in the struct

### Step 3: Understand Binary Encoding with BinaryCodec

RawCdxProperty values are stored as raw binary `Vec<u8>`. Use the `BinaryCodec` trait to encode/decode:

```rust
use crate::cdx::binary_codec::BinaryCodec;

// Decoding (from_raw):
let value = i16::decode(binary_data)?;

// Encoding (to_raw):
let binary = value.encode()?;
```

Supported types:
- `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `f64` - primitive types
- `bool` - boolean values
- `String` - UTF-8 text (usually from `CdxString.text`)
- `Vec<u8>` - raw binary data (pass-through)
- `Point2d` - 2D points (x, y as f64)
- `Rectangle` - rectangles (left, top, right, bottom as f64)
- `Vec<u32>` - object ID arrays (via `encode_u32_array()` / `decode_u32_array()`)

Complex types like `CdxString` and `ObjectIDArray` require custom handling - see property-specific examples below.

### Step 4: Create the Struct File

Create `src/cdx/[object_name].rs` with:

```rust
use crate::cdx::raw_nodes::RawCdxObject;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx_tags::[object_name]_tags::*;
use crate::CdxError;
use serde::{Deserialize, Serialize};
use super::node::TaggedObject;

/// [Object Description from specification]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct [ObjectName] {
    pub id: u32,
    // Required properties first
    pub [required_prop1]: [Type],
    pub [required_prop2]: [Type],
    
    // Optional properties (wrapped in Option<>)
    pub [optional_prop1]: Option<[Type]>,
    pub [optional_prop2]: Option<[Type]>,
    // ... more properties
}

impl [ObjectName] {
    /// Create a new [ObjectName] with required properties
    pub fn new(id: u32, [required_params]) -> Self {
        [ObjectName] {
            id,
            [required_props],
            [all_optional_props_as_None],
        }
    }
}

impl TaggedObject for [ObjectName] {
    const TAG: u16 = CDXOBJ_[OBJECTNAME];

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Extract required properties - decode from binary
        let [required_prop] = raw
            .get_property(CDXPROP_[REQUIRED_PROP])
            .ok_or_else(|| {
                CdxError::DecodeError("[ObjectName] missing required property".to_string())
            })
            .and_then(|v| [Type]::decode(v))?;

        // Extract optional properties - use .and_then() for optional decoding
        let [optional_prop] = raw
            .get_property(CDXPROP_[OPTIONAL_PROP])
            .and_then(|v| [Type]::decode(v).ok());

        Ok([ObjectName] {
            id: raw.id,
            [required_prop],
            [optional_prop],
            // ... include all extracted properties
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx::raw_nodes::RawCdxProperty;
        
        let mut properties = Vec::new();
        
        // Encode required properties
        properties.push(RawCdxProperty {
            tag: CDXPROP_[REQUIRED_PROP],
            value: self.[required_prop].encode()?,
        });
        
        // Encode optional properties
        if let Some(val) = self.[optional_prop] {
            properties.push(RawCdxProperty {
                tag: CDXPROP_[OPTIONAL_PROP],
                value: val.encode()?,
            });
        }
        
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}
```

#### Special Cases for String Properties:

For string properties (stored as `CdxString` in CDX, but binary in RawCdxProperty):
```rust
// from_raw: Decode binary to String
let text = raw
    .get_property(CDXPROP_[PROPERTY])
    .and_then(|v| String::decode(v).ok());

// to_raw: Encode String to binary
if let Some(ref text) = self.[prop_name] {
    properties.push(RawCdxProperty {
        tag: CDXPROP_[PROPERTY],
        value: text.encode()?,
    });
}
```

#### Special Cases for Array Properties:

For object ID arrays:
```rust
use crate::cdx::binary_codec::{encode_u32_array, decode_u32_array};

// from_raw: Decode binary to Vec<u32>
let ids = raw
    .get_property(CDXPROP_[PROPERTY])
    .and_then(|v| decode_u32_array(v).ok());

// to_raw: Encode Vec<u32> to binary
if let Some(ref ids) = self.[prop_name] {
    properties.push(RawCdxProperty {
        tag: CDXPROP_[PROPERTY],
        value: encode_u32_array(ids)?,
    });
}
```

### Step 5: Update Module Exports

1. Open `src/cdx/mod.rs`
2. Add to the module declarations:
   ```rust
   pub mod [object_name];
   ```
3. Add to the re-exports:
   ```rust
   pub use [object_name]::*;
   ```

### Step 6: Verify the Implementation

1. Run `cargo check` to verify compilation
2. Check for any unused properties or incorrect type mappings
3. Ensure all required properties have `.ok_or_else()` error handling
4. Ensure optional properties do NOT have `.ok_or_else()` (they should be `Option<T>`)

## Common Pitfalls

1. **Pattern matching on CdxValue**: Don't use `if let CdxValue::Type` - use `BinaryCodec::decode()` instead
2. **Forgetting to use BinaryCodec**: All primitive and complex types must go through encoding/decoding
3. **Wrong error handling for optional properties**: Use `.and_then(|v| Type::decode(v).ok())` for optional
4. **Wrong error handling for required properties**: Use `.ok_or_else()` then `.and_then()` for required
5. **Missing module registration**: New modules must be added to `src/cdx/mod.rs`
6. **Trait import issues**: Always use `use crate::cdx::binary_codec::BinaryCodec;` in from_raw/to_raw
7. **Property constant typos**: Verify constant names match exactly in `[object]_tags.rs`

## Property Organization Pattern

Always organize properties in this order:
1. `id: u32` (always present)
2. Required properties (1-10 typically)
3. Optional common properties (visibility, color, warnings, etc.)
4. Optional object-specific properties
5. Optional geometry/style properties (spacing, width, font, etc.)

## Testing Your Implementation

After implementation:

```bash
# Check compilation
cargo check

# Build the project
cargo build

# Run tests (if any)
cargo test
```

## Example File Structure

For a complete example, see `src/cdx/bond.rs` which implements:
- Bond struct with all properties
- `TaggedObject` trait implementation
- Property extraction with proper error handling
- Both required and optional properties

## Object Types Reference

Common CDX objects to implement:
- Bond (`kCDXObj_Bond`, 0x8005)
- Node (`kCDXObj_Node`, 0x8001)
- Fragment (`kCDXObj_Fragment`, 0x8004)
- Group (`kCDXObj_Group`, 0x8002)
- Page (`kCDXObj_Page`, 0x8000)
- Document (`kCDXObj_Document`, 0x8003)
- Text (`kCDXObj_Text`, 0x8006)
- Geometry objects (Point, Line, Polygon, etc.)

Each follows the same implementation pattern described above.
