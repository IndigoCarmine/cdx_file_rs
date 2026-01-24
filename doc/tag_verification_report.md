# CDX Tags Verification Report (Updated)

## 検証日時
2026-01-25 (Updated)

## 検証対象
- ドキュメント: doc/*.txt
- コード: src/cdx/tags.rs, src/cdx/values.rs

## 修正完了事項

### ✅ 修正されたタグ定義

#### オブジェクトタグ
- `ARROW` (0x8021 → **0x8027**) ✅ 修正完了
- `STOICHIOMETRY_GRID` (0x8022) → **CONSTRAINT** に変更（エイリアスとして保持）
- `SG_COMPONENT` (0x8023) → **TLC_PLATE** に変更（エイリアスとして保持）
- `SG_DATUM` (0x8024) → **TLC_LANE** に変更（エイリアスとして保持）

新規追加：
- `GEOMETRY` (0x8021) - kCDXObj_Geometry
- `CONSTRAINT` (0x8022) - kCDXObj_Constraint
- `TLC_PLATE` (0x8023) - kCDXObj_TLCPlate
- `TLC_LANE` (0x8024) - kCDXObj_TLCLane

#### プロパティタグ
- `FG_COLOR` (0x0301) - kCDXProp_ForegroundColor (UINT16) ✅ 修正完了
- `BG_COLOR` (0x0302) - kCDXProp_BackgroundColor (INT16) ✅ 修正完了
- `BOND_DOUBLE_POSITION` (0x0601 → **0x0603**) ✅ 修正完了

### 📋 現在の正しいタグ定義

#### オブジェクトタグ (0x8000番台)
```rust
pub const DOCUMENT: u16 = 0x8000;  // kCDXObj_Document
pub const PAGE: u16 = 0x8001;      // kCDXObj_Page
pub const GROUP: u16 = 0x8002;     // kCDXObj_Group
pub const FRAGMENT: u16 = 0x8003;  // kCDXObj_Fragment
pub const NODE: u16 = 0x8004;      // kCDXObj_Node
pub const BOND: u16 = 0x8005;      // kCDXObj_Bond
pub const TEXT: u16 = 0x8006;      // kCDXObj_Text
pub const GRAPHIC: u16 = 0x8007;   // kCDXObj_Graphic
pub const OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag
pub const GEOMETRY: u16 = 0x8021;  // kCDXObj_Geometry
pub const CONSTRAINT: u16 = 0x8022; // kCDXObj_Constraint
pub const TLC_PLATE: u16 = 0x8023; // kCDXObj_TLCPlate
pub const TLC_LANE: u16 = 0x8024;  // kCDXObj_TLCLane
pub const ARROW: u16 = 0x8027;     // kCDXObj_Arrow
```

#### プロパティタグ (0x0000番台)
```rust
// Position/Geometry
pub const POSITION: u16 = 0x0200;       // kCDXProp_2DPosition
pub const EXTENT: u16 = 0x0202;         // (extent/size)
pub const BOUNDING_BOX: u16 = 0x0204;   // kCDXProp_BoundingBox
pub const HEAD_3D: u16 = 0x0207;        // kCDXProp_3DHead
pub const TAIL_3D: u16 = 0x0208;        // kCDXProp_3DTail

// Color
pub const COLOR_TABLE: u16 = 0x0300;    // kCDXProp_ColorTable
pub const FG_COLOR: u16 = 0x0301;       // kCDXProp_ForegroundColor (UINT16)
pub const BG_COLOR: u16 = 0x0302;       // kCDXProp_BackgroundColor (INT16)

// Node/Atom
pub const ELEMENT: u16 = 0x0402;        // kCDXProp_Node_Element (INT16)

// Bond
pub const BOND_ORDER: u16 = 0x0600;     // kCDXProp_Bond_Order (INT16)
pub const BOND_DOUBLE_POSITION: u16 = 0x0603; // kCDXProp_Bond_DoublePosition (INT16)
pub const BOND_BEGIN: u16 = 0x0604;     // kCDXProp_Bond_Begin (UINT32)
pub const BOND_END: u16 = 0x0605;       // kCDXProp_Bond_End (UINT32)

// Text
pub const TEXT_STRING: u16 = 0x0700;    // kCDXProp_Text
pub const TEXT_STRING_ALT: u16 = 0x0709; // Alternative text

// Font
pub const FONT_TABLE: u16 = 0x0100;     // kCDXProp_FontTable
```

### 🔧 values.rsの型定義

すべて正しく定義されています：
- `FG_COLOR` (0x0301): **Uint16** ✅
- `BG_COLOR` (0x0302): **Int16** ✅
- `ELEMENT` (0x0402): **Int16** ✅
- `BOND_ORDER` (0x0600): **Int16** ✅
- `BOND_BEGIN` (0x0604): **Uint32** ✅
- `BOND_END` (0x0605): **Uint32** ✅

### 📝 レガシータグの扱い

以下のタグは公式ドキュメントに記載がありませんが、既存コードとの互換性のためエイリアスとして保持：

```rust
// Legacy/Undocumented tags (not found in official documentation)
pub const STOICHIOMETRY_GRID: u16 = 0x8022;  // Alias for CONSTRAINT
pub const SG_COMPONENT: u16 = 0x8023;        // Alias for TLC_PLATE
pub const SG_DATUM: u16 = 0x8024;            // Alias for TLC_LANE
```

これらは render.rs と export.rs で使用されています。

## 検証結果

✅ **すべてのタグ定義がCDX仕様書と一致しています**
✅ **cargo check が成功しました**
✅ **既存コードとの互換性を維持しています**

## 今後の推奨事項

1. **追加タグの定義**: よく使われる以下のタグを追加することを検討
   - `0x000A` - kCDXProp_ZOrder (INT16)
   - `0x0011` - kCDXProp_Visible (CDXBoolean)
   - `0x0400` - kCDXProp_Node_Type (INT16)
   - `0x0421` - kCDXProp_Atom_Charge (INT8)
   - `0x042B` - kCDXProp_Atom_NumHydrogens (UINT16)

2. **レガシータグの調査**: STOICHIOMETRY_GRID関連のタグが実際にどのように使われているか、実際のCDXファイルで確認

3. **包括的なタグ定義**: すべてのCDXタグを網羅的に定義したファイルの作成を検討
