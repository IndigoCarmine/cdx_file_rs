# Renderer コードレビュー - 拡張性に関する問題点

このドキュメントは、`src/renderer/` ディレクトリのコードレビューで発見された拡張性に関する問題点をまとめたものです。

---

## Issue 1: eguiへの強い依存 - レンダラーバックエンドの抽象化不足

### 問題概要
現在のRendererは`eframe::egui`に直接依存しており、他のレンダリングバックエンド（SVG出力、PDF出力、他のGUIフレームワーク等）への切り替えが困難です。

### 該当箇所
- `src/renderer/renderer.rs`: `RenderContext`が`egui::Painter`を直接保持
- `src/renderer/mod.rs`: `pub use eframe::egui;`でegui依存をexport
- 全ての`Drawable`実装: `egui::Color32`, `egui::Pos2`等を直接使用

### 現状のコード例
```rust
pub struct RenderContext<'a> {
    pub painter: &'a Painter,  // egui直接依存
    pub origin: Pos2,           // egui型
    // ...
}
```

### 推奨される改善案
1. 抽象的な`Painter` traitを定義し、egui実装を1つのバックエンドとして分離
2. `Point2D`, `Color`等の独自型を定義し、各バックエンドへの変換レイヤーを設ける

```rust
// 抽象化例
pub trait AbstractPainter {
    fn line_segment(&self, start: Point2d, end: Point2d, stroke: Stroke);
    fn circle_filled(&self, center: Point2d, radius: f32, color: Color);
    fn text(&self, pos: Point2d, text: &str, font_size: f32, color: Color);
}
```

### 優先度: 高

---

## Issue 2: マジックナンバーの散在

### 問題概要
描画パラメータ（デフォルト半径、オフセット、サイズ等）がハードコードされており、テーマやスタイル変更が困難です。

### 該当箇所

**node.rs (Line 9):**
```rust
let radius = 10.0; // Default atom radius in pixels
```

**node.rs (Line 37):**
```rust
ctx.draw_text(&charge_str, charge_pos, egui::Color32::RED, 8.0);
```

**arrow.rs (Line 42):**
```rust
self.head_size.unwrap_or(10) as f32
```

**bond.rs (Line 53):**
```rust
let spacing = self.bond_spacing.unwrap_or(10) as f32;
```

**text.rs (Lines 100-103):**
```rust
let base_font_size = (run.font_size as f32) / 20.0;
```

**graphic.rs:**
- Line 233: `0.01 * 96.0 * scale / 100.0` (DPI計算のマジックナンバー)
- Line 245: `size * 0.4` (矢印サイズ比率)

### 推奨される改善案
1. `RenderStyle`または`Theme`構造体を作成し、全てのデフォルト値を集約
2. `RenderContext`にスタイル設定を持たせる

```rust
pub struct RenderStyle {
    pub default_atom_radius: f32,
    pub default_font_size: f32,
    pub charge_label_size: f32,
    pub arrowhead_size_ratio: f32,
    pub bond_spacing: f32,
    pub screen_dpi: f32,
    // etc.
}
```

### 優先度: 中

---

## Issue 3: Drawableトレイトの拡張性の制限

### 問題概要
現在の`Drawable`トレイトは最小限のインターフェースしか持っておらず、今後の機能追加（選択状態の描画、ヒットテスト、バウンディングボックス取得等）が困難です。

### 該当箇所
```rust
pub trait Drawable {
    fn draw(&self, ctx: &RenderContext);
}
```

### 問題点
1. ヒットテスト用のbounding box取得メソッドがない
2. 選択・ホバー状態の描画に対応していない
3. Z-orderの制御ができない
4. 描画前後のフック処理がない

### 推奨される改善案
```rust
pub trait Drawable {
    fn draw(&self, ctx: &RenderContext);
    
    // 拡張用メソッド（デフォルト実装付き）
    fn bounding_box(&self, ctx: &RenderContext) -> Option<BoundingBox> {
        None
    }
    
    fn draw_selected(&self, ctx: &RenderContext) {
        self.draw(ctx);
    }
    
    fn draw_hovered(&self, ctx: &RenderContext) {
        self.draw(ctx);
    }
    
    fn z_order(&self) -> i32 {
        0
    }
    
    fn is_visible(&self) -> bool {
        true
    }
}
```

### 優先度: 高

---

## Issue 4: 色解決ロジックの重複

### 問題概要
各`Drawable`実装で色テーブルからの色取得ロジックが重複しています。

### 該当箇所

**node.rs (Lines 12-17):**
```rust
let color = match self.foreground_color {
    Some(color_idx) => ctx.document.get_color_table()
        .and_then(|ct| ct.get(color_idx as usize))
        .map(|c| c.to_color32())
        .unwrap_or(egui::Color32::GREEN),
    None => egui::Color32::YELLOW,
};
```

**bond.rs (Lines 24-30):** 同様のパターン
**arrow.rs (Lines 29-33):** 同様のパターン
**graphic.rs (Lines 260-266):** 同様のパターン
**text.rs (Lines 35-40, 106-109):** 同様のパターン

### 推奨される改善案
`RenderContext`に統一された色解決メソッドを追加:

```rust
impl RenderContext {
    pub fn resolve_color(&self, color_index: Option<u16>, default: Color32) -> Color32 {
        match color_index {
            Some(idx) => self.document.get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(default),
            None => default,
        }
    }
}
```

### 優先度: 中

---

## Issue 5: define_node_renderer!マクロの保守性問題

### 問題概要
新しい`NodePayload`バリアントを追加する際、マクロの呼び出し部分を手動で更新する必要があり、忘れやすいです。

### 該当箇所
```rust
define_node_renderer!(
    Arrow,
    Bond,
    Border,
    Constraint,
    Document,
    Fragment,
    Geometry,
    Graphic,
    Group,
    Node,
    ObjectTag,
    Page,
    ReactionScheme,
    ReactionStep,
    TextObject,
    TlcLane,
    TLCPlate,
    UnknownObject802B,
);
```

### 問題点
1. `NodePayload`の定義と`define_node_renderer!`の呼び出しが離れている
2. バリアントを追加した際にマクロ更新を忘れるとコンパイルエラーになるが、分かりにくい

### 推奨される改善案
1. enumの定義と同じ場所でマクロを呼び出す
2. derive macroの利用を検討
3. `NodePayload`にデフォルトの`draw`実装を持たせ、未実装の場合はno-opにする

### 優先度: 低

---

## Issue 6: デバッグ出力（eprintln!）の残存

### 問題概要
プロダクションコードにデバッグ用の`eprintln!`マクロ呼び出しが残っています。

### 該当箇所

**graphic.rs:**
- Line 20-21: `eprintln!("Drawing Graphic...")`
- Line 51: `eprintln!("Arrow line_seg...")`
- Line 213: `eprintln!("Arrowhead: line too short...")`
- Line 239: `eprintln!("Arrowhead: size=...")`
- Line 251: `eprintln!("Arrowhead points...")`

### 推奨される改善案
1. `log`クレートまたは`tracing`クレートを使用し、ログレベルで制御
2. `#[cfg(debug_assertions)]`で開発時のみ有効化

```rust
#[cfg(debug_assertions)]
log::debug!("Drawing Graphic id={}: type={}, arrow_type={:?}", ...);
```

### 優先度: 中

---

## Issue 7: エラーハンドリングの不統一

### 問題概要
描画に必要なデータが欠けている場合の処理が一貫していません。一部は早期returnし、一部は無視されます。

### 該当箇所

**bond.rs (Lines 11-19):** `None`の場合は即return
```rust
let start = match _ctx.node_position(self.begin) {
    Some(pos) => pos,
    None => return,
};
```

**arrow.rs (Lines 22-24):** `None`の場合は即return（コメントあり）
```rust
} else {
    // Not enough data to draw
    return;
};
```

**node.rs:** position_2dがNoneの場合は何も描画しない（暗黙的）

### 推奨される改善案
1. 描画失敗時にResultを返すオプションを提供
2. デバッグモードでは警告を出力
3. 描画統計を収集するための仕組みを導入

### 優先度: 低

---

## Issue 8: 座標変換ロジックの分散

### 問題概要
CDX座標からスクリーン座標への変換が各`draw`メソッド内で個別に行われています。

### 該当箇所

**renderer.rs (Lines 301-306):**
```rust
pub fn cdx_to_screen(&self, cdx_pos: &Point2d) -> Pos2 {
    let scale = self.zoom * self.auto_scale;
    Pos2 {
        x: self.origin.x + (cdx_pos.x as f32 * scale),
        y: self.origin.y - (cdx_pos.y as f32 * scale),
    }
}
```

**graphic.rs:** 個別のスケール計算が複数箇所にある
- Line 112-114: `let scale = ctx.zoom * ctx.auto_scale;`
- Line 151-152: 同様の計算
- Line 228: 同様の計算

### 推奨される改善案
座標変換を完全にRenderContextに集約し、単位変換の責務を明確化:

```rust
impl RenderContext {
    pub fn cdx_length_to_screen(&self, cdx_length: f64) -> f32 {
        (cdx_length as f32) * self.zoom * self.auto_scale
    }
    
    pub fn cdx_to_screen_offset(&self, dx: f64, dy: f64) -> (f32, f32) {
        let scale = self.zoom * self.auto_scale;
        (dx as f32 * scale, -dy as f32 * scale)
    }
}
```

### 優先度: 中

---

## Issue 9: 未実装のDrawable実装が多い

### 問題概要
多くの`Drawable`実装が空の`draw`メソッドのみで、将来の実装が必要な状態です。

### 該当箇所
- `fragment.rs`: 空実装
- `group.rs`: 空実装
- `page.rs`: 空実装
- `document.rs`: 空実装
- `geometry.rs`: 空実装（コメントで意図的と説明）
- `border.rs`: 空実装
- `constraint.rs`: 空実装
- `reaction_scheme.rs`: 空実装
- `reaction_step.rs`: 空実装
- `object_tag.rs`: 空実装
- `tlc_lane.rs`: 空実装
- `tlc_plate.rs`: 空実装
- `unknown_802b.rs`: 空実装

### 問題点
1. どれが意図的に空なのか、どれが未実装なのか区別がつきにくい
2. 将来的に実装が必要なものの追跡が難しい

### 推奨される改善案
1. 意図的に空の場合は明確なコメントを付与
2. 未実装の場合は`todo!()`や`unimplemented!()`マクロを使用
3. `TODO`コメントで実装予定を明示

```rust
impl Drawable for TLCPlate {
    fn draw(&self, _ctx: &RenderContext) {
        // TODO: TLCPlate描画を実装
        // 参考: ChemDraw TLCPlate仕様
    }
}
```

### 優先度: 低

---

## Issue 10: レイヤー/Z-orderサポートの欠如

### 問題概要
現在の実装では描画順序がツリー走査順序に依存しており、明示的なレイヤー制御ができません。

### 該当箇所
**renderer.rs (Lines 182-188):**
```rust
fn render(&self, root: Node<crate::cdx::file::NodePayload>, ctx: &RenderContext) {
    let data = root.borrow_data();
    data.draw(ctx); 
    for child in root.children() {
        self.render(child, ctx);
    }
}
```

### 問題点
1. 特定のオブジェクトを最前面/最背面に描画できない
2. 選択オブジェクトのハイライト表示が困難
3. オーバーレイ（グリッド、ガイド等）の追加が困難

### 推奨される改善案
1. マルチパスレンダリングの導入
2. Z-order属性のサポート
3. 描画レイヤーの概念を導入

```rust
enum RenderPass {
    Background,
    Objects,
    Overlay,
    Selection,
}

impl CdxRenderer {
    fn render_pass(&self, pass: RenderPass, ctx: &RenderContext) {
        // パス別の描画処理
    }
}
```

### 優先度: 高

---

## まとめ

| Issue | タイトル | 優先度 |
|-------|---------|--------|
| 1 | eguiへの強い依存 | 高 |
| 2 | マジックナンバーの散在 | 中 |
| 3 | Drawableトレイトの拡張性の制限 | 高 |
| 4 | 色解決ロジックの重複 | 中 |
| 5 | define_node_renderer!マクロの保守性問題 | 低 |
| 6 | デバッグ出力の残存 | 中 |
| 7 | エラーハンドリングの不統一 | 低 |
| 8 | 座標変換ロジックの分散 | 中 |
| 9 | 未実装のDrawable実装が多い | 低 |
| 10 | レイヤー/Z-orderサポートの欠如 | 高 |

### 優先度の高い問題 (直ちに対処を推奨)
1. **Issue 1**: eGuiへの強い依存 → 将来のSVG/PDF出力対応に必須
2. **Issue 3**: Drawableトレイトの拡張性 → インタラクティブ機能追加に必須
3. **Issue 10**: Z-orderサポート → 選択/ハイライト機能に必須

### 次のステップ
1. 優先度の高い問題から着手
2. Issue 4（色解決の重複）は比較的簡単に改善可能
3. Issue 2（マジックナンバー）は段階的にリファクタリング可能
