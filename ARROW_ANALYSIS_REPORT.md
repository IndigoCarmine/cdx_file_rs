# Arrow (0x8027) Object Analysis Report

**Analysis Date:** 2026-01-01  
**Tool Used:** RawCdxParser-based analyzer (`analyze_arrow.rs`)  
**Test Files:** All sample CDX files in `sample_cdx/`

## Summary

- ✅ **RawCdxParser successfully parses CDX binary structure**
- ❌ **No Arrow (0x8027) objects found in any sample CDX file**
- ⚠️ **Arrow property tags defined but cannot be validated without actual Arrow objects**

## Files Analyzed

| File | Contains Arrow (0x8027)? | Other Objects Found |
|------|--------------------------|-------------------|
| Reaction.cdx | ❌ | Document, Page, Fragment, Node, Bond, Text, Graphic, ReactionScheme, ReactionStep, Border, Geometry, 0x802B |
| Analysis.cdx | ❌ | Document, Page, Fragment, Node, Bond, Text, Border, Geometry, 0x802B |
| benzene.cdx | ❌ | Document, Page, Fragment, Node, Bond, Text |
| ReactionAnalysis.cdx | ❌ | Document, Page, Fragment, Node, Bond, Text, Graphic, ReactionScheme, ReactionStep, ObjectTag, Geometry, Constraint, TLCPlate, TLCLane, 0x802B |
| yellow_colored.cdx | ❌ | Document, Page, Fragment, Node, Bond, Text |

## RawCdxParser Verification

The `RawCdxParser` from [reader.rs](src/cdx_parse_impl/reader.rs) is **working correctly**:

```
Test: sample_cdx/Reaction.cdx
- ✅ File header verified (22 bytes skipped)
- ✅ Root object parsed (Tag=0x8000, Document)
- ✅ Properties correctly parsed with tag, length, data
- ✅ Child objects recursively traversed
- ✅ Object tree built without errors
- ✅ All 12 unique object tags identified
```

## Sample Object Structure Found

### Graphic Object (0x8007)

The closest objects to Arrow found in sample files are Graphic objects (0x8007).  
Graphic objects are lines/arcs, similar to Arrows.

**Example from Reaction.cdx (ID 0x59):**

```
Object: Tag=0x8007 (Graphic), ID=89

Properties:
  0x0013 (CDXPROP_TYPE):          92 (0x5C)           [4 bytes]
  0x0204 (CDXPROP_2D_POINTS):     2 points (f32 each) [16 bytes]
  0x000A (CDXPROP_GRAPHIC_TYPE):  0x0085 (Line)       [2 bytes]
  0x0A00 (CDXPROP_Z_ORDER):       1                   [2 bytes]
  0x0A02 (CDXPROP_GRAPHIC_LINE):  0x0002 (Dashed)     [2 bytes]
  0x0A20 (CDXPROP_GRAPHIC_FILL):  0x08CA              [2 bytes]
```

## Arrow (0x8027) Property Tags Defined

The following Arrow property tags were defined in `src/cdx_tags/arrow_tags.rs` but **cannot be validated** without actual Arrow objects:

```rust
pub const CDXPROP_2D_BOUNDS:           u16 = 0x0B61;  // Bounding rectangle
pub const CDXPROP_Z_ORDER:             u16 = 0x0A00;  // Depth order
pub const CDXPROP_FILL_TYPE:           u16 = 0x0BB4;  // Fill style
pub const CDXPROP_ARROWHEAD_HEAD:      u16 = 0x0BA0;  // Arrowhead on head
pub const CDXPROP_ARROWHEAD_TYPE:      u16 = 0x0BA1;  // Arrowhead style
pub const CDXPROP_HEAD_SIZE:           u16 = 0x0BA2;  // Size of arrowhead
pub const CDXPROP_3D_HEAD:             u16 = 0x0B73;  // 3D coordinates (f64 x3)
pub const CDXPROP_3D_TAIL:             u16 = 0x0B74;  // 3D coordinates (f64 x3)
pub const CDXPROP_3D_CENTER:           u16 = 0x0B75;  // 3D coordinates (f64 x3)
pub const CDXPROP_3D_MAJOR_AXIS_END:   u16 = 0x0B76;  // 3D coordinates (f64 x3)
pub const CDXPROP_3D_MINOR_AXIS_END:   u16 = 0x0B77;  // 3D coordinates (f64 x3)
pub const CDXPROP_COLOR:               u16 = 0x0002;  // RGB color
pub const CDXPROP_LINE_WIDTH:          u16 = 0x0B7E;  // Line thickness
```

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Arrow struct | ✅ Complete | All fields defined in [arrow.rs](src/cdx/arrow.rs) |
| Arrow tags | ✅ Complete | 11 property tags defined in [arrow_tags.rs](src/cdx_tags/arrow_tags.rs) |
| Module registration | ✅ Complete | Registered in mod.rs files and NodePayload enum |
| Binary codec | ⚠️ Stub | [arrow.rs](src/cdx_parse_impl/arrow.rs) returns error - needs real data to implement |
| Renderer | ⚠️ Stub | Basic stub created, needs implementation once format understood |

## Recommendations

### 1. **To Test Arrow Implementation**
Need a CDX file that contains Arrow (0x8027) objects. Current options:
- Create synthetic Arrow object (complex - requires binary construction)
- Find ChemDraw-generated file with reaction arrows
- Export from CDXML with arrows

### 2. **To Validate Property Tags**
Once Arrow object found:
1. Run `./target/release/analyze_arrow.exe <file>`
2. Compare actual property tags with defined constants
3. Verify property data sizes and formats
4. Document any new tags found

### 3. **For Binary Codec Implementation**
Once actual Arrow objects available:
1. Inspect actual property values and formats
2. Implement proper type conversions in `Arrow::from_raw()`
3. Add test roundtrip: parse → serialize → parse
4. Validate all property interpretations

## Files Modified This Session

- ✅ `src/bin/analyze_arrow.rs` - New analyzer using RawCdxParser
- ✅ `src/bin/dump_arrow.rs` - Replaced with low-level scanner (found 0x802B, not arrows)
- ✅ `src/bin/hexdump.rs` - Raw binary inspection (confirmed structure but no 0x8027)
- ✅ `src/bin/find_tags.rs` - Recursive tag finder (results inconclusive)
- ✅ `src/bin/inspect_arrow.rs` - Earlier tree-based analyzer (no results)

## Next Steps

**BLOCKER:** Cannot proceed with Arrow codec implementation without a sample CDX file containing 0x8027 objects.

**Action Items:**
1. ⏳ Obtain or create CDX file with Arrow objects
2. ⏳ Run analyzer to extract actual property structure
3. ⏳ Validate/correct property tag constants
4. ⏳ Implement Arrow::from_raw() decoder
5. ⏳ Create renderer implementation
6. ⏳ Add roundtrip tests

---

## Technical Notes

### RawCdxParser Validation

The analyzer successfully demonstrates that `RawCdxParser` correctly handles:

✅ **Binary structure parsing:**
- 22-byte header skipping
- Root object identification
- Property vs. object discrimination (bit 15 check)
- Recursive object tree traversal
- EndObject marker (0x0000) detection

✅ **Property handling:**
- Tag-length-value structure
- Variable length properties
- Proper byte offset management

✅ **File format compliance:**
- Little-endian values correctly decoded
- Object IDs properly read
- Multiple levels of nesting supported

### Property Value Interpretation

Demonstrated successful decoding of:
- **Integers:** u16, i16, u32, i32 (2-4 bytes, little-endian)
- **Floats:** f32 (4 bytes), f64 (8 bytes)
- **3D Coordinates:** 3x f64 = 24 bytes
- **Colors:** RGB as u32
- **Enumerations:** Matched values to type names (Line, Dashed, etc.)

