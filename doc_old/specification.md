# CDX File Parser Specification (Rust)

このドキュメントは、RustによるCDX（ChemDraw Binary）ファイルのパーサ実装に必要な仕様をまとめたものです。

## 1. 基本仕様

* **バイトオーダー**: Little-endian (リトルエンディアン)
* **全体構造**:
    ```
    [Fixed Header (32 bytes)]
    [Tagged Items (Recursive Object/Property Tree)]
    [00 00 (EOF)]
    ```

## 2. ファイルヘッダ (32 bytes)

| オフセット | サイズ | 内容 | 値 (Hex) |
| :--- | :--- | :--- | :--- |
| 0x00 | 8 bytes | マジック文字列 | `"VjCD0100"` |
| 0x08 | 4 bytes | 予約領域 (Legacy) | `04 03 02 01` |
| 0x0C | 16 bytes | 予約領域 (Zero) | `00 ... 00` |
| 0x1C | 4 bytes | 追加の予約領域 | (通常 00) |

※ ヘッダ直後から最上位の **Document Object** が開始される。

## 3. Tagged Item の判別

各アイテムの先頭2バイト (Tag ID) によって種類を判別する。

* **MSB (bit 15) = 0**: **Property**
* **MSB (bit 15) = 1**: **Object**
* **bit 14**:
    * 0: システム定義 (Standard Tag)
    * 1: ユーザー定義 (User Tag)

## 4. Property の構造

Propertyは値を保持する属性。

```
[TagID: 2 bytes]
[Length: 2 bytes]
[Data: Length bytes]
```

### 長さ (Length) の特例
* `0x0000`: データなし。フラグとして機能。
* `0xFFFF`: 拡張長。直後に `uint32` (4 bytes) のデータ長が続く。
    * 構造: `[TagID: 0xXXXX] [0xFFFF] [ActualLength: 4 bytes] [Data: ActualLength bytes]`

## 5. Object の構造

Objectは他のPropertyやObjectを内包できるコンテナ。

```
[TagID: 2 bytes]
[ObjectID: 4 bytes]
[Contents: Properties / Objects ...]
[00 00 (EndObject)]
```

* **ObjectID**: 
    * `uint32`
    * `0` の場合はIDなし（他のオブジェクトから参照されない）。
* **EndObject**:
    * `00 00` (2 bytes) でオブジェクトの終了を示す。

## 6. パース・アルゴリズム

パーサは再帰的な構造を持つ必要がある。

1. ヘッダを検証する。
2. 次の2バイト（TagID）を読み込む。
3. TagID が `00 00` であれば、現在のオブジェクトを終了する。
4. TagID の bit 15 を確認：
    * **Property (bit 15 = 0)**:
        * 長さを読み、データをシリアライズされた型として解釈する。
    * **Object (bit 15 = 1)**:
        * ObjectIDを読み、再帰的に内部のアイテムを読み始める。
5. ファイル終端まで繰り返す。

## 7. 編集・書き出し (Editing & Writing)

「読み書き・編集可能」という要件を満たすため、以下の設計指針を採用する。

* **順序の保持**: CDXはバイナリツリー構造を持つ。編集後に再保存した際、可能な限り元の構造やプロパティの順序を維持できるよう、内部データ構造には `IndexMap` 等を使用し、順序を保証する。
* **ObjectIDの管理**: 新規オブジェクト追加や削除時に、ObjectIDの一貫性を保つためのマネージャーを実装する。
* **未知のタグへの対応**: 仕様にない未知のタグ（User Tags等）が含まれていても、バイナリデータとしてそのまま保持し、再書き出し時に消失しないようにする。

## 8. 実装フェーズ（案）

1.  **Phase 1: 基礎データ構造とパース実装**
    *   `CdxObject`, `CdxProperty` の定義。
    *   基本ヘッダと再帰的なパースロジックの実装。
    *   主要なデータ型（Int, Float, String, Point, Rect）のデコード。
2.  **Phase 2: 書き出しの実装**
    *   データ構造からバイナリへのシリアライズ。
    *   パース -> 書き出しでバイナリが一致することの検証（Round-trip test）。
3.  **Phase 3: 編集APIの構築**
    *   ノードの検索、追加、削除、値の変更を行うためのユーティリティ。
4.  **Phase 4: 高度なデータ解釈**
    *   タグ定義に基づいた型安全なプロパティアクセス。
    *   色空間、フォント、テキストスタイルなどの詳細な解釈。

## 9. 使用ライブラリ（選定）

*   **binrw**: バイナリの構造化された読み書き。
*   **thiserror**: エラー定義。
*   **indexmap**: プロパティ順序の保持。
*   **serde**: (将来的なGUIやJSON連携用)

## 10. 現在の進捗 (Current Status)

* [x] **Phase 1: 基礎データ構造とパース実装**
    * 22バイトヘッダの特定と実装。
    * 再帰的な Object/Property ツリーのパースロジック実装。
    * 網羅的な型デコード実装：
        * `CDXString`: スタイル情報付き文字列。
        * `CDXPoint2D`, `CDXPoint3D`: 2D/3D座標。
        * `CDXRectangle`: 矩形。
        * 各種数値型: `Int8/16/32`, `Uint8/16/32`, `Float64`。
        * `CDXBoolean`, `CDXBooleanImplied`。
        * `CDXCoordinate`: 固定小数点数（points / 65536）。
        * `CDXFontTable`, `CDXColorTable`: テーブル構造。
        * `INT16ListWithCounts`: リスト型。
        * `CDXObjectIDArray`: 他オブジェクトへの参照配列。
* [x] **Phase 2: 書き出しの実装**
    * ツリー構造からバイナリへのシリアライズ実装。
    * 全サンプルファイルでの Perfect Round-trip（バイナリ完全一致）の達成。
* [x] **Phase 3: GUI表示のプロトタイプ**
    * `egui` を用いた分子・反応式のレンダリング実装。
* [ ] **Phase 4: 編集APIの構築**
    * 汎用的なノード検索・値変更API。

---

## 質問事項 (Resolved)
*   **ライブラリ**: 自由に使用可 -> `binrw` 等を検討。
*   **サポート範囲**: 汎用的なツリー構造を目指す。
*   **CDXML**: 現時点では考慮不要。
*   **用途**: 読み書き・編集。将来的にGUI表示。
