# cdx-file-rs

[English](README.md) | 日本語

ChemDraw CDXファイルの読み込み、書き込み、レンダリングを行うRustライブラリです。

## 概要

`cdx-file-rs`は、ChemDraw CDX（Chemical Drawing Exchange）バイナリファイルを解析・操作するための純粋なRust実装です。このライブラリでは以下のことができます：

- CDXファイルの**読み込み**と階層構造の解析
- 解析されたデータ構造からのCDXファイルの**書き込み**
- 内蔵ビューアを使用した化学構造の**レンダリング**

## 機能

- CDXバイナリフォーマットの完全サポート
- バイナリ同一性を保持したラウンドトリップ読み書き
- 一般的な化学図形要素のサポート：
  - 分子（原子と結合）
  - 反応スキームと反応ステップ
  - 矢印とグラフィックス
  - テキストと注釈
  - TLCプレート
  - グループとフラグメント
- `eframe`/`egui`を使用した内蔵GUIビューア
- 抽象化されたレンダリングシステムを使用したSVGおよびPNG形式へのエクスポート

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
cdx-file-rs = "0.1.0"
```

## 使用方法

### CDXファイルの読み込み

```rust
use std::fs;
use std::io::Cursor;
use cdx_file_rs::cdx::reader::RawCdxParser;

fn main() -> std::io::Result<()> {
    let data = fs::read("molecule.cdx")?;
    let mut parser = RawCdxParser::new(Cursor::new(&data));
    let document = parser.parse()?;
    
    println!("Document tag: 0x{:04x}", document.tag);
    println!("Properties: {}", document.properties.len());
    println!("Children: {}", document.children.len());
    
    Ok(())
}
```

### CDXファイルの書き込み

```rust
use std::fs::File;
use std::io::Cursor;
use cdx_file_rs::cdx::writer::CdxWriter;
use cdx_file_rs::cdx::reader::RawCdxParser;

fn main() -> std::io::Result<()> {
    // 既存のファイルを読み込み
    let data = std::fs::read("input.cdx")?;
    let mut parser = RawCdxParser::new(Cursor::new(&data));
    let document = parser.parse()?;
    
    // 新しいファイルに書き込み
    let output: Vec<u8> = Vec::new();
    let mut writer = CdxWriter::new(Cursor::new(output));
    writer.write(&document)?;
    
    let written_data = writer.into_inner().into_inner();
    std::fs::write("output.cdx", written_data)?;
    
    Ok(())
}
```

### 高レベルNode表現への変換

```rust
use std::fs;
use std::io::Cursor;
use cdx_file_rs::cdx::reader::RawCdxParser;
use cdx_file_rs::cdx::file::Node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read("molecule.cdx")?;
    let mut parser = RawCdxParser::new(Cursor::new(&data));
    let raw_object = parser.parse()?;
    
    // 高レベルNode表現に変換
    let node = Node::from_raw(raw_object)?;
    
    println!("Node tag: 0x{:04x}", node.tag());
    println!("Node ID: {}", node.id());
    println!("Children: {}", node.children.len());
    
    Ok(())
}
```

### SVGとPNGへのエクスポート

ライブラリは、抽象化されたレンダリングシステムを使用してCDXファイルをSVGおよびPNG形式に変換するエクスポート機能を提供します：

```rust
use std::fs;
use cdx_file_rs::cdx::file::CdxFile;
use cdx_file_rs::renderer::{export_to_svg, export_to_png, RenderExportOptions};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CDXファイルの読み込みと解析
    let data = fs::read("molecule.cdx")?;
    let cdx_file = CdxFile::from_bytes(&data)?;
    
    // エクスポートオプションの設定
    let mut options = RenderExportOptions::default();
    options.width = 1024;
    options.height = 768;
    options.margin = 50.0;
    
    // SVGへエクスポート
    export_to_svg(&cdx_file, Path::new("output.svg"), &options)?;
    
    // PNGへエクスポート（高解像度）
    options.scale = 2.0;
    export_to_png(&cdx_file, Path::new("output.png"), &options)?;
    
    Ok(())
}
```

付属のサンプルを実行することもできます：

```bash
cargo run --example export_cdx -- molecule.cdx output
```

これにより、`molecule.cdx`から`output.svg`と`output.png`が作成されます。

## CDXファイルフォーマット

CDXフォーマットは、ChemDrawが化学図面を保存するために使用するバイナリフォーマットです。主な特徴：

- **バイトオーダー**: リトルエンディアン
- **構造**: ヘッダの後にタグ付きアイテム（オブジェクトとプロパティ）がツリー構造で続く
- **ヘッダ**: マジック文字列`VjCD0100`で始まる22バイト
- **オブジェクト**: bit15=1のタグで識別され、IDとネストされたコンテンツを含む
- **プロパティ**: bit15=0のタグで識別され、型付きデータを含む

## サポート要素

| 要素 | 読み込み | 書き込み | 説明 |
|------|----------|----------|------|
| Document | ✓ | ✓ | ルートドキュメントオブジェクト |
| Page | ✓ | ✓ | 描画ページ |
| Fragment | ✓ | ✓ | 分子フラグメント |
| Node | ✓ | ✓ | 分子内の原子 |
| Bond | ✓ | ✓ | 化学結合 |
| Arrow | ✓ | ✓ | 反応矢印 |
| Graphic | ✓ | ✓ | グラフィカル要素 |
| Text | ✓ | ✓ | テキスト注釈 |
| Group | ✓ | ✓ | グループ化されたオブジェクト |
| Reaction Scheme | ✓ | ✓ | 反応スキーム |
| Reaction Step | ✓ | ✓ | 個別の反応ステップ |
| TLC Plate | ✓ | ✓ | TLCプレート図 |
| Color Table | ✓ | ✓ | 色定義 |

## ビューアの実行

ライブラリにはCDXファイル用の内蔵GUIビューアが含まれています：

```bash
cargo run --release
```

## テストの実行

```bash
cargo test
```

CDXML 読み込みは RDKit の CDXML テストファイルでテストします。データはコミットしていないので、
取得してから（`tests/data/rdkit/` に shallow + sparse clone、git 管理外）実行します:

```bash
./scripts/fetch_rdkit_testdata.sh   # Windows では scripts/fetch_rdkit_testdata.ps1
cargo test --test rdkit_cdxml_test -- --nocapture
```

全ファイルについて「読める・書き出して読み直すと同じになる・ChemDraw 自身のソースでも読めないもの以外は何も落とさない」ことを確認します。
`tests/cdxml_cdx_pairs_test.rs` は `sample_cdxml/X.cdxml` が ChemDraw の保存した `sample_cdx/X.cdx` と同じ内容に読めることを確認します。

CDXML の要素名・属性名・タグ・値の型・列挙名は ChemDraw 自身のファイル形式ソース
（https://github.com/Glysade/chemdraw, BSD-3）から `src/cdxml/table.rs` に生成しています。再生成:

```bash
./scripts/fetch_chemdraw_sdk.sh     # Windows では scripts/fetch_chemdraw_sdk.ps1（tools/data/chemdraw/ へ）
python tools/gen_cdxml_table.py tools/data/chemdraw/chemdraw > src/cdxml/table.rs
```

## ライセンス

このプロジェクトは [MIT](LICENSE-MIT) または [Apache-2.0](LICENSE-APACHE) のどちらか好きな方（MIT OR Apache-2.0）の下でライセンスされています。

`src/cdxml/table.rs` は ChemDraw のファイル形式のソース（[Glysade/chemdraw](https://github.com/Glysade/chemdraw)）から生成しています。このソースは BSD 3-Clause ライセンス（Copyright (c) 1986-2025, CambridgeSoft Corp, Revvity Inc and others）で公開されています。

## コントリビューション

コントリビューションを歓迎します！イシューやプルリクエストをお気軽に提出してください。
