以下は、**Bond に限らず Node / Fragment / Reaction 等の複数オブジェクトに共通で実装させることを前提**に整理し直した、**Agent 向け指示書（Markdown）**である。
レンダリング Trait を「共通基盤」として設計することを明確にしている。

---

## 目的

* 化学構造を構成する複数のモデルオブジェクト
  （`Bond`, `Node`, `Fragment`, 将来的には `Reaction`, `Arrow` 等）
  を **共通のレンダリング Trait** により egui 上に描画可能にする
* 描画処理をモデル層から分離し、UI 依存を局所化する
* オブジェクトごとの差異は Trait 実装側で吸収し、上位描画ループを単純化する

---

## 基本方針

* **1つの Trait を複数 Struct に実装する**
* Trait は「描画」という単一責務のみを持つ
* モデル（Bond / Node 等）は egui に直接依存しない
* 座標解決・色解決などの環境依存情報は外部から与える

---

## レンダリング Trait の定義方針

### Trait の責務

* 自身が表現するオブジェクトを egui 上に描画する
* 他オブジェクトとの関係解決（例：Bond → Node 座標）は直接行わない
* 描画順序（z-order）は Trait 実装側で考慮する

### 想定する最小インターフェース

```rust
fn draw_objects(
    &self,
    painter: &egui::Painter,
    origin: egui::Pos2,
);
```

* `painter`: egui 描画コンテキスト
* `origin`: ローカル座標系の基準点
* 追加情報が必要な場合は Context 構造体導入を検討する

---

## Trait 設計のスコープ

### 実装対象（必須）

* `Bond`
* `Node`

### 実装対象（将来拡張）

* `Fragment`
* `Reaction`
* `ReactionArrow`
* 補助的アノテーション（ラベル、電荷、立体表示など）

Agent は、**単一オブジェクトだけを前提とした設計を行ってはならない**。

---

## 描画コンテキストの整理

複数オブジェクトに共通して必要となる外部情報を整理する：

* ID → スクリーン座標解決（Node, Control Point 等）
* 色テーブル（CDX color index → `egui::Color32`）
* スケール、ズーム率
* 表示オプション（query 表示、stereo 表示 等）

対応方針（いずれか、または組み合わせ）：

* `RenderContext` 構造体を導入し Trait に渡す
* 上位レイヤーでクロージャ／Resolver を保持する
* Trait 実装側では「参照のみ」を行う

---

## 描画処理の一般的な分解指針

Agent は、各オブジェクトの描画処理を以下の段階に分解する：

1. 可視性判定

   * `visible == false` の場合は即 return
2. 描画属性の決定

   * 色、線幅、フォント、表示フラグ
3. ジオメトリ生成

   * 点、線分、多角形、補助線
4. egui への描画命令発行

   * `Painter::line_segment`
   * `Painter::circle_filled`
   * `Painter::text` 等

Bond 固有の処理（結合次数、二重線位置など）は
**共通フローの一部として局所化**すること。

---

## モジュール構成指針

* Trait 定義は以下のような独立モジュールに置く：

```
render/
 ├─ mod.rs
 ├─ drawable.rs   // Trait 定義
 ├─ context.rs    // RenderContext
 └─ egui_impl/
     ├─ bond.rs
     ├─ node.rs
     └─ fragment.rs
```

* モデル層 (`Bond`, `Node`) は `render` モジュールに依存しない
* egui 依存は `egui_impl` 以下に閉じ込める

---

## 上位レイヤーでの利用イメージ

* 上位コードは「型を意識せず」描画できる状態を目指す

```rust
for obj in objects {
    obj.draw_objects(painter, origin);
}
```

Agent は、この形を実現できる設計になっているかを常に確認すること。

---

## テスト・検証方針

* 単体テストでは以下を重視する：

  * panic しないこと
  * 可視性・フラグによる分岐が正しく評価されること
* 描画結果の正否は egui デモアプリで目視確認する

---

## Agent への最終指示

* Bond 専用の Trait を設計してはならない
* **「複数オブジェクト共通の描画基盤」**として設計すること
* 依存方向（model → render → egui）を厳密に守ること
* 拡張時に Trait 破壊が起きない設計を優先すること

---

この Markdown は、そのまま Agent に渡して設計・実装を進められるレベルの指示書として完結している。
