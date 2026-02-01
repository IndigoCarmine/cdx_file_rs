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
    let output = Vec::new();
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

## ライセンス

このプロジェクトは[GNU Lesser General Public License v3.0](LICENSE)（LGPL-3.0）の下でライセンスされています。

## コントリビューション

コントリビューションを歓迎します！イシューやプルリクエストをお気軽に提出してください。
