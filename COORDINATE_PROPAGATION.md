# 座標伝搬の実装 (Coordinate Propagation Implementation)

## 概要 (Overview)

このドキュメントは、親オブジェクトから子オブジェクトへの座標伝搬機能の実装について説明します。

This document explains the implementation of coordinate propagation from parent to child objects in the CDX renderer.

## 背景 (Background)

CDXファイル形式では、一部のオブジェクト（特にPage）が`BoundsInParent`プロパティを持ちます。このプロパティは、親の座標空間内でのオブジェクトの位置を定義します。

The CDX file format allows certain objects (particularly Page objects) to have a `BoundsInParent` property. This property defines the position of an object within its parent's coordinate space.

### 使用例 (Use Cases)

- **Tableオブジェクト**: 各セルは個別のPageオブジェクトとして格納され、BoundsInParentでセルの位置を指定
- **入れ子のページ**: 複数のページを持つドキュメントで、各ページが独自の座標空間を持つ
- **グループ化されたオブジェクト**: 相対的な位置関係を維持したまま移動可能

- **Table objects**: Each cell is stored as a Page object with BoundsInParent specifying cell position
- **Nested pages**: Multi-page documents where each page has its own coordinate space
- **Grouped objects**: Objects that maintain relative positions when moved together

## 実装 (Implementation)

### 1. RenderContextの拡張

`RenderContext`構造体に`parent_offset`フィールドを追加しました。

```rust
pub struct RenderContext<'a> {
    pub painter: &'a Painter,
    pub origin: Pos2,
    pub document: &'a Document,
    pub node_positions: HashMap<u32, Point2d>,
    pub zoom: f32,
    pub auto_scale: f32,
    /// Cumulative offset from parent objects (in CDX coordinates)
    pub parent_offset: Point2d,
}
```

### 2. オフセット付きコンテキストの作成

`with_offset()`メソッドで、親のオフセットを継承した新しいコンテキストを作成できます。

```rust
pub fn with_offset(&self, offset: &Point2d) -> Self {
    RenderContext {
        painter: self.painter,
        origin: self.origin,
        document: self.document,
        node_positions: self.node_positions.clone(),
        zoom: self.zoom,
        auto_scale: self.auto_scale,
        parent_offset: Point2d {
            x: self.parent_offset.x + offset.x,
            y: self.parent_offset.y + offset.y,
        },
    }
}
```

### 3. 座標変換の修正

`cdx_to_screen()`メソッドで、parent_offsetを考慮した座標変換を行います。

```rust
pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> Pos2 {
    let scale = self.zoom * self.auto_scale;
    // Apply parent offset to the CDX position
    let adjusted_x = cdx_pos.x + self.parent_offset.x;
    let adjusted_y = cdx_pos.y + self.parent_offset.y;
    Pos2 {
        x: self.origin.x + (adjusted_x as f32 * scale),
        y: self.origin.y - (adjusted_y as f32 * scale), // CDX uses inverted Y-axis
    }
}
```

### 4. 描画トラバーサルでの適用

`CdxRenderer::render()`メソッドで、PageのBoundsInParentをチェックし、子オブジェクトに適切なオフセットを適用します。

```rust
fn render(&self, root: Node<crate::cdx::file::NodePayload>, ctx: &RenderContext) {
    let data = root.borrow_data();
    
    // Check if this object defines a coordinate offset for its children
    let child_ctx = if let NodePayload::Page(page) = &*data {
        if let Some(bounds) = &page.bounds_in_parent {
            // Create a child context with offset from the parent's top-left corner
            let offset = Point2d {
                x: bounds.left,
                y: bounds.top,
            };
            ctx.with_offset(&offset)
        } else {
            ctx.clone()
        }
    } else {
        ctx.clone()
    };
    
    // Draw the current object and its children with the appropriate context
    data.draw(&child_ctx);
    for child in root.children() {
        self.render(child, &child_ctx);
    }
}
```

## 座標系 (Coordinate System)

### CDX座標系
- 原点: 通常は左上
- X軸: 右方向が正
- Y軸: 下方向が正

### 画面座標系 (egui)
- 原点: 左上
- X軸: 右方向が正
- Y軸: 下方向が正

### 変換式

```
screen_x = origin.x + (cdx_x + parent_offset.x) * scale
screen_y = origin.y - (cdx_y + parent_offset.y) * scale  // Y軸反転
```

## 後方互換性 (Backward Compatibility)

- BoundsInParentプロパティがないオブジェクトでは、parent_offsetは0のまま
- 既存のCDXファイルは変更なく動作
- 座標変換ロジックは拡張されただけで、既存の動作は保持

## テスト (Tests)

以下のテストケースを実装しました:

1. **test_parent_offset_accumulation**: オフセットの累積が正しく行われることを確認
2. **test_bounds_in_parent_offset_extraction**: BoundsInParentから正しくオフセットを抽出
3. **test_coordinate_transformation_logic**: 座標変換ロジックの正確性を確認
4. **test_cumulative_offset_transformation**: 複数レベルのオフセット累積を確認

すべてのテストが成功しています。

## 将来の拡張 (Future Extensions)

現在はPageオブジェクトのみサポートしていますが、他のオブジェクトタイプにも拡張可能です:

- Fragment with bounding_box
- Group with bounding_box
- その他のコンテナオブジェクト

## 参考資料 (References)

- CDX File Format Specification
- `doc/page_object.txt` - Page object documentation
- `doc/table_object.txt` - Table object documentation
