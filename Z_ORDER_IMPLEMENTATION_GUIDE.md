# Z-Order/レイヤーサポート実装ガイド

このドキュメントは、Issue 10「レイヤー/Z-orderサポートの欠如」を解決するための具体的な実装手順を示します。

---

## 背景

現在のRendererは単純なツリー走査により描画を行っており、以下の問題があります：
- オブジェクトの描画順序を制御できない
- 選択状態のハイライト表示ができない
- グリッド・ガイドなどのオーバーレイ追加が困難

---

## 実装ステップ

### Step 1: RenderPass enumの定義

**ファイル**: `src/renderer/renderer.rs`

```rust
/// 描画パス - 描画順序を制御するための列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderPass {
    /// 最背面：背景色、グリッド
    Background = 0,
    /// 通常オブジェクト：分子構造、テキストなど
    Objects = 1,
    /// オーバーレイ：選択ハイライト、ホバーエフェクト
    Overlay = 2,
    /// 最前面：選択ハンドル、カーソル
    Selection = 3,
}

impl RenderPass {
    /// 全てのパスを描画順序で返す
    pub fn all_passes() -> &'static [RenderPass] {
        &[
            RenderPass::Background,
            RenderPass::Objects,
            RenderPass::Overlay,
            RenderPass::Selection,
        ]
    }
}
```

---

### Step 2: RenderLayer enumの定義（オブジェクト用）

**ファイル**: `src/renderer/renderer.rs`

```rust
/// オブジェクトの描画レイヤー
/// 同一パス内でのZ順序を制御
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderLayer {
    /// 最背面（Container objects: Page, Document）
    Back = 0,
    /// 通常（Bonds, Graphics）
    Normal = 1,
    /// 前面（Nodes/Atoms）
    Front = 2,
    /// 最前面（Text, Labels）
    Top = 3,
}

impl Default for RenderLayer {
    fn default() -> Self {
        RenderLayer::Normal
    }
}
```

---

### Step 3: Drawable traitの拡張

**ファイル**: `src/renderer/renderer.rs`

```rust
/// Common trait for rendering chemical model objects in egui.
pub trait Drawable {
    /// メインの描画メソッド
    fn draw(&self, ctx: &RenderContext);
    
    /// このオブジェクトが描画されるべきパスを返す
    /// デフォルトはObjects
    fn render_pass(&self) -> RenderPass {
        RenderPass::Objects
    }
    
    /// このオブジェクトの描画レイヤーを返す
    /// 同一パス内でのZ順序を決定
    fn render_layer(&self) -> RenderLayer {
        RenderLayer::Normal
    }
    
    /// 選択状態の描画（Selection パスで呼ばれる）
    /// デフォルトは何もしない
    fn draw_selection(&self, _ctx: &RenderContext, _selected: bool) {
        // デフォルト実装: 何もしない
    }
    
    /// ホバー状態の描画（Overlay パスで呼ばれる）
    /// デフォルトは何もしない
    fn draw_hover(&self, _ctx: &RenderContext) {
        // デフォルト実装: 何もしない
    }
}
```

---

### Step 4: RenderContextの拡張

**ファイル**: `src/renderer/renderer.rs`

```rust
use std::collections::HashSet;

pub struct RenderContext<'a> {
    pub painter: &'a Painter,
    pub origin: Pos2,
    pub document: &'a Document,
    pub node_positions: HashMap<u32, Point2d>,
    pub zoom: f32,
    pub auto_scale: f32,
    // 新規追加フィールド
    pub current_pass: RenderPass,
    pub selected_ids: &'a HashSet<u32>,
    pub hovered_id: Option<u32>,
}

impl<'a> RenderContext<'a> {
    /// 既存のnewメソッド（後方互換性のため維持）
    pub fn new(
        painter: &'a Painter,
        origin: Pos2,
        document: &'a Document,
        node_positions: HashMap<u32, Point2d>,
        zoom: f32,
        auto_scale: f32,
    ) -> Self {
        // デフォルトはObjectsパス、選択なし
        static EMPTY_SET: std::sync::LazyLock<HashSet<u32>> = 
            std::sync::LazyLock::new(HashSet::new);
        RenderContext {
            painter,
            origin,
            document,
            node_positions,
            zoom,
            auto_scale,
            current_pass: RenderPass::Objects,
            selected_ids: &EMPTY_SET,
            hovered_id: None,
        }
    }
    
    /// 新しいコンストラクタ（パス・選択情報付き）
    pub fn new_with_pass(
        painter: &'a Painter,
        origin: Pos2,
        document: &'a Document,
        node_positions: HashMap<u32, Point2d>,
        zoom: f32,
        auto_scale: f32,
        current_pass: RenderPass,
        selected_ids: &'a HashSet<u32>,
        hovered_id: Option<u32>,
    ) -> Self {
        RenderContext {
            painter,
            origin,
            document,
            node_positions,
            zoom,
            auto_scale,
            current_pass,
            selected_ids,
            hovered_id,
        }
    }
    
    /// 現在のオブジェクトが選択されているか確認
    pub fn is_selected(&self, id: u32) -> bool {
        self.selected_ids.contains(&id)
    }
    
    /// 現在のオブジェクトがホバー中か確認
    pub fn is_hovered(&self, id: u32) -> bool {
        self.hovered_id == Some(id)
    }
}
```

---

### Step 5: RenderableItem構造体（描画順序制御用）

**ファイル**: `src/renderer/renderer.rs`

```rust
use std::cmp::Ordering;

/// 描画順序を決定するためのラッパー構造体
/// 
/// この構造体は、描画可能オブジェクトをパス、レイヤー、Z-indexの順でソートするために使用される。
/// ソート後の順序で描画することで、正しいZ順序が保証される。
/// 
/// # フィールド
/// - `node`: 元のNodePayloadを保持するノード
/// - `pass`: このオブジェクトが描画されるパス
/// - `layer`: 同一パス内での描画レイヤー
/// - `z_index`: 将来の拡張用（現在は常に0、明示的なZ-index制御が必要になった場合に使用）
struct RenderableItem<'a> {
    node: Node<NodePayload>,
    pass: RenderPass,
    layer: RenderLayer,
    /// 将来の拡張用フィールド。現在は使用されず常に0。
    /// 明示的なZ-index制御が必要になった場合に、各オブジェクトの
    /// z_order属性からこの値を設定する。
    z_index: i32,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> RenderableItem<'a> {
    fn new(node: Node<NodePayload>) -> Self {
        let data = node.borrow_data();
        let pass = data.render_pass();
        let layer = data.render_layer();
        drop(data);
        
        RenderableItem {
            node,
            pass,
            layer,
            z_index: 0,  // 現時点では未使用、将来の拡張で活用
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a> Ord for RenderableItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pass.cmp(&other.pass)
            .then(self.layer.cmp(&other.layer))
            .then(self.z_index.cmp(&other.z_index))
    }
}

impl<'a> PartialOrd for RenderableItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for RenderableItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass && self.layer == other.layer && self.z_index == other.z_index
    }
}

impl<'a> Eq for RenderableItem<'a> {}
```

---

### Step 6: CdxRenderer::render_all の改修

**ファイル**: `src/renderer/renderer.rs`

```rust
impl<'a> CdxRenderer<'a> {
    /// Render all objects from a CdxFile
    pub fn render_all(
        &self,
        painter: &Painter,
        cdx_file: &crate::cdx::file::CdxFile,
    ) {
        self.render_all_with_selection(painter, cdx_file, &HashSet::new(), None);
    }
    
    /// Render all objects with selection/hover support
    pub fn render_all_with_selection(
        &self,
        painter: &Painter,
        cdx_file: &crate::cdx::file::CdxFile,
        selected_ids: &HashSet<u32>,
        hovered_id: Option<u32>,
    ) {
        let bg_color = self.background_color();
        let document = match cdx_file.get_document() {
            Ok(doc) => doc,
            Err(_) => return,
        };
        let mut node_positions: HashMap<u32, Point2d> = HashMap::new();
        let tree = &cdx_file.tree;
        let root = tree.root();
        self.collect_node_positions(root, &mut node_positions);

        // Fill background (Background pass)
        let rect = painter.clip_rect();
        painter.rect_filled(rect, 0.0, bg_color);

        // Collect all renderable items
        let mut items = Vec::new();
        self.collect_renderables(root, &mut items);
        
        // Sort by render order
        items.sort();

        // Render each pass
        for pass in RenderPass::all_passes() {
            let ctx = RenderContext::new_with_pass(
                painter,
                Pos2 { 
                    x: self.center_offset.x + self.offset.x, 
                    y: self.center_offset.y + self.offset.y 
                },
                &document,
                node_positions.clone(),
                self.zoom,
                self.auto_scale,
                *pass,
                selected_ids,
                hovered_id,
            );
            
            for item in items.iter().filter(|i| i.pass == *pass) {
                let data = item.node.borrow_data();
                match pass {
                    RenderPass::Background => {
                        // Background handled separately
                    }
                    RenderPass::Objects => {
                        data.draw(&ctx);
                    }
                    RenderPass::Overlay => {
                        // Draw hover effect if applicable
                        if let Some(hid) = hovered_id {
                            if self.get_node_id(&item.node) == Some(hid) {
                                data.draw_hover(&ctx);
                            }
                        }
                    }
                    RenderPass::Selection => {
                        // Draw selection if applicable
                        if let Some(id) = self.get_node_id(&item.node) {
                            let is_selected = selected_ids.contains(&id);
                            data.draw_selection(&ctx, is_selected);
                        }
                    }
                }
            }
        }
    }
    
    /// Collect all renderable items from tree
    fn collect_renderables(
        &self,
        root: Node<NodePayload>,
        items: &mut Vec<RenderableItem>,
    ) {
        items.push(RenderableItem::new(root.clone()));
        for child in root.children() {
            self.collect_renderables(child, items);
        }
    }
    
    /// Get node ID if available
    /// 
    /// 各NodePayloadバリアントからIDを取得する。
    /// Container types（Document, Page, Fragment, Group）はIDを持たないため
    /// Noneを返す。これらは選択対象ではないため問題ない。
    fn get_node_id(&self, node: &Node<NodePayload>) -> Option<u32> {
        let data = node.borrow_data();
        match &*data {
            // 選択可能なオブジェクト（IDを持つ）
            NodePayload::Node(n) => Some(n.id),
            NodePayload::Bond(b) => Some(b.id),
            NodePayload::Arrow(a) => Some(a.id),
            NodePayload::TextObject(t) => Some(t.id),
            NodePayload::Graphic(g) => Some(g.id),
            NodePayload::Border(b) => Some(b.id),
            NodePayload::Constraint(c) => Some(c.id),
            NodePayload::Geometry(g) => Some(g.id),
            NodePayload::ObjectTag(o) => Some(o.id),
            NodePayload::ReactionScheme(r) => Some(r.id),
            NodePayload::ReactionStep(r) => Some(r.id),
            NodePayload::TlcLane(t) => Some(t.id),
            NodePayload::TLCPlate(t) => Some(t.id),
            NodePayload::UnknownObject802B(u) => Some(u.id),
            // Container types - IDを持たない、選択対象外
            NodePayload::Document(_) => None,
            NodePayload::Page(_) => None,
            NodePayload::Fragment(_) => None,
            NodePayload::Group(_) => None,
        }
    }
}
```

---

### Step 7: NodePayload マクロの更新

**ファイル**: `src/renderer/renderer.rs`

```rust
#[macro_export]
macro_rules! define_node_renderer {
    (
        $( $ty:ident ),* $(,)? 
    ) => {
        impl NodePayload {
            pub fn draw(&self, ctx: &RenderContext) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw(ctx),
                    )*
                }
            }
            
            pub fn render_pass(&self) -> RenderPass {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.render_pass(),
                    )*
                }
            }
            
            pub fn render_layer(&self) -> RenderLayer {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.render_layer(),
                    )*
                }
            }
            
            pub fn draw_selection(&self, ctx: &RenderContext, selected: bool) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw_selection(ctx, selected),
                    )*
                }
            }
            
            pub fn draw_hover(&self, ctx: &RenderContext) {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.draw_hover(ctx),
                    )*
                }
            }
        }
    };
}
```

---

### Step 8: 各Drawable実装の更新

各オブジェクト種別に適切な`render_layer`を設定します。

**Bond (src/renderer/bond.rs)**:
```rust
impl Drawable for Bond {
    fn draw(&self, ctx: &RenderContext) {
        // 既存の実装
    }
    
    fn render_layer(&self) -> RenderLayer {
        RenderLayer::Normal  // Bonds are drawn in normal layer
    }
    
    fn draw_selection(&self, ctx: &RenderContext, selected: bool) {
        if !selected { return; }
        // Draw selection highlight (e.g., thicker stroke, blue color)
        // ... selection drawing logic
    }
}
```

**Node (src/renderer/node.rs)**:
```rust
impl Drawable for Node {
    fn draw(&self, ctx: &RenderContext) {
        // 既存の実装
    }
    
    fn render_layer(&self) -> RenderLayer {
        RenderLayer::Front  // Atoms are drawn in front of bonds
    }
    
    fn draw_selection(&self, ctx: &RenderContext, selected: bool) {
        if !selected { return; }
        // Draw selection highlight (e.g., larger circle, blue outline)
    }
}
```

**TextObject (src/renderer/text.rs)**:
```rust
impl Drawable for TextObject {
    fn draw(&self, ctx: &RenderContext) {
        // 既存の実装
    }
    
    fn render_layer(&self) -> RenderLayer {
        RenderLayer::Top  // Text is drawn on top
    }
}
```

**Container types (Document, Page, Fragment, Group)**:
```rust
impl Drawable for Document {
    fn draw(&self, _ctx: &RenderContext) {}
    
    fn render_layer(&self) -> RenderLayer {
        RenderLayer::Back  // Containers are in back
    }
}
```

---

### Step 9: 選択ハイライトのスタイル定義

**新規ファイル**: `src/renderer/selection_style.rs`

```rust
use eframe::egui::{Color32, Stroke};

/// 選択状態の描画スタイル
pub struct SelectionStyle {
    /// 選択時のアウトラインカラー
    pub selection_color: Color32,
    /// 選択時のアウトライン幅
    pub selection_stroke_width: f32,
    /// ホバー時のアウトラインカラー
    pub hover_color: Color32,
    /// ホバー時のアウトライン幅
    pub hover_stroke_width: f32,
}

impl Default for SelectionStyle {
    fn default() -> Self {
        SelectionStyle {
            selection_color: Color32::from_rgb(0, 120, 215),  // Windows blue
            selection_stroke_width: 2.0,
            hover_color: Color32::from_rgba_unmultiplied(0, 120, 215, 128),
            hover_stroke_width: 1.5,
        }
    }
}

impl SelectionStyle {
    pub fn selection_stroke(&self) -> Stroke {
        Stroke::new(self.selection_stroke_width, self.selection_color)
    }
    
    pub fn hover_stroke(&self) -> Stroke {
        Stroke::new(self.hover_stroke_width, self.hover_color)
    }
}
```

---

## 実装順序（推奨）

1. **Phase 1: 基盤整備**
   - [ ] `RenderPass` enumを追加
   - [ ] `RenderLayer` enumを追加
   - [ ] `Drawable` traitにデフォルトメソッドを追加
   - [ ] `RenderContext`に新フィールドを追加

2. **Phase 2: マルチパスレンダリング**
   - [ ] `RenderableItem`構造体を実装
   - [ ] `render_all_with_selection`メソッドを実装
   - [ ] 既存の`render_all`を新メソッドに委譲

3. **Phase 3: 各オブジェクトの更新**
   - [ ] Bond: `render_layer()` を `Normal` に
   - [ ] Node: `render_layer()` を `Front` に
   - [ ] TextObject: `render_layer()` を `Top` に
   - [ ] Container types: `render_layer()` を `Back` に

4. **Phase 4: 選択/ホバー機能**
   - [ ] `SelectionStyle`を追加
   - [ ] 主要オブジェクト（Node, Bond, TextObject）に`draw_selection`を実装
   - [ ] 主要オブジェクトに`draw_hover`を実装

---

## 検証方法

1. **視覚的検証**
   - 分子構造が正しく表示されることを確認
   - Bond が Node の下に描画されることを確認
   - TextObject が最前面に描画されることを確認

2. **選択機能の検証**
   - オブジェクト選択時にハイライトが表示されることを確認
   - 複数選択時に全てがハイライトされることを確認

3. **ホバー機能の検証**
   - マウスホバー時にエフェクトが表示されることを確認

---

## 後方互換性

- 既存の `render_all()` メソッドは維持
- 新機能はオプトイン方式（`render_all_with_selection()`を使用する場合のみ有効）
- `Drawable` traitの新メソッドはデフォルト実装を持つため、既存の実装を壊さない

---

## 将来の拡張

1. **グリッド表示**: `RenderPass::Background` を使用
2. **スナップガイド**: `RenderPass::Overlay` を使用
3. **ドラッグプレビュー**: `RenderPass::Selection` を使用
4. **カスタムZ-index**: `z_index` フィールドを活用

---

## 備考

- この実装ではdendronクレートのNode型を使用
- egui 0.29に依存
- HashSet/HashMapはstd::collectionsを使用
