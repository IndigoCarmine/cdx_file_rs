# ChemDraw CDX File Viewer & Parser (Rust Implementation)

このプロジェクトは、ChemDrawのバイナリ形式（`.cdx`）を解析し、Rustで描画・操作するためのツール群です。

## プロジェクトの進捗状況（ワークフロー）

1.  **基盤構築**:
    - `binrw`ライブラリを採用し、CDXのリトルエンディアン形式を効率的にデコードする Parser を実装。
    - Round-trip試験（読み込み→書き出し→バイナリ比較）により、データ整合性を担保。
2.  **GUI Viewerの実装**:
    - `eframe` (egui) を使用したクロスプラットフォーム・ビューアを作成。
    - 原子(Node)、結合(Bond)、テキスト、反応矢印(Arrow)の描画に対応。
3.  **座標系の解析と修正**:
    - 2D座標（Point2d）に加え、ChemDraw内部で多用される3D座標（Point3d）のデコード順序（X, Y, Z）を特定し、描画ズレを解消。
4.  **色の再現**:
    - CDXのカラーテーブル（BGR 16-bit）を解析。ドキュメントの背景色および各オブジェクトへの着色を実装。
5.  **リファクタリング**:
    - コード内のタグ番号（0x...）を定数化。
    - オブジェクトからのプロパティ取得をカプセル化（`get_pos2d()`, `get_bond_order()` 等のアクセサを導入）。
6.  **UXの向上**:
    - マウス位置を中心としたズーム機能の搭載。

## ファイル構造

### コア・ロジック (`src/`)
- **`lib.rs`**: Parser (`CdxParser`) と Writer (`CdxWriter`) の実装。
- **`cdx_types.rs`**: 
    - CDXの全データ型（`CdxValue`）の定義。
    - タグ定数（`tags`モジュール）。
    - オブジェクト操作用の高レベルAPI。
- **`main.rs`**: GUIアプリケーション本体。`egui`による描画ループとカメラ制御。
- **`dump.rs`**: CDXファイルの内容をテキスト形式で出力するデバッグツール。
- **`validation.rs`**: `sample_cdx/` 内の全ファイルに対してバリデーションを行うツール。

### リソース
- **`sample_cdx/`**: テスト用の ChemDraw バイナリファイル群。
  - `benzene.cdx`: 基本構造。
  - `Reaction.cdx`: 反応矢印。
  - `ReactionAnalysis.cdx`: Stoichiometry Grid。
  - `yellow_colored.cdx`: 色設定の検証用。
- **`pycdxml/`**: Python用のCDX解析ライブラリ。参考用。
- **`sample_cdxml/`**: テスト用の ChemDraw XMLファイル群。


## 次のステップへのヒント
- **詳細なスタイル**: 結合のステレオ（Wedge/Dash）や、フォントの書体切り替えには未対応。
- **編集機能**: 現在はビューアとしての機能がメイン。ノードの移動や値の変更を書き戻す機能の拡張が可能。