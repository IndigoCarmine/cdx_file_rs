# cdx-file-rs

English | [日本語](README_ja.md)

A Rust library for reading, writing, and rendering ChemDraw CDX files.

## Overview

`cdx-file-rs` is a pure Rust implementation for parsing and manipulating ChemDraw CDX (Chemical Drawing Exchange) binary files. This library allows you to:

- **Read** CDX files and parse their hierarchical structure
- **Write** CDX files from parsed data structures
- **Render** chemical structures using the built-in viewer

## Features

- Full support for CDX binary format parsing
- Roundtrip read/write capability with binary identity preservation
- Support for common chemical drawing elements:
  - Molecules (atoms and bonds)
  - Reaction schemes and reaction steps
  - Arrows and graphics
  - Text and annotations
  - TLC plates
  - Groups and fragments
- Built-in GUI viewer using `eframe`/`egui`
- Export to SVG and PNG formats using abstracted rendering system

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cdx-file-rs = "0.1.0"
```

## Usage

### Reading a CDX file

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

### Writing a CDX file

```rust
use std::fs::File;
use std::io::Cursor;
use cdx_file_rs::cdx::writer::CdxWriter;
use cdx_file_rs::cdx::reader::RawCdxParser;

fn main() -> std::io::Result<()> {
    // Read an existing file
    let data = std::fs::read("input.cdx")?;
    let mut parser = RawCdxParser::new(Cursor::new(&data));
    let document = parser.parse()?;

    // Write to a new file
    let output: Vec<u8> = Vec::new();
    let mut writer = CdxWriter::new(Cursor::new(output));
    writer.write(&document)?;

    let written_data = writer.into_inner().into_inner();
    std::fs::write("output.cdx", written_data)?;

    Ok(())
}
```

### Converting to high-level Node representation

```rust
use std::fs;
use std::io::Cursor;
use cdx_file_rs::cdx::reader::RawCdxParser;
use cdx_file_rs::cdx::file::Node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read("molecule.cdx")?;
    let mut parser = RawCdxParser::new(Cursor::new(&data));
    let raw_object = parser.parse()?;

    // Convert to high-level Node representation
    let node = Node::from_raw(raw_object)?;

    println!("Node tag: 0x{:04x}", node.tag());
    println!("Node ID: {}", node.id());
    println!("Children: {}", node.children.len());

    Ok(())
}
```

### Exporting to SVG and PNG

The library provides export functionality to convert CDX files to SVG and PNG formats using the abstracted rendering system:

```rust
use std::fs;
use cdx_file_rs::cdx::file::CdxFile;
use cdx_file_rs::renderer::{export_to_svg, export_to_png, RenderExportOptions};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse CDX file
    let data = fs::read("molecule.cdx")?;
    let cdx_file = CdxFile::from_bytes(&data)?;

    // Configure export options
    let mut options = RenderExportOptions::default();
    options.width = 1024;
    options.height = 768;
    options.margin = 50.0;

    // Export to SVG
    export_to_svg(&cdx_file, Path::new("output.svg"), &options)?;

    // Export to PNG (with higher resolution)
    options.scale = 2.0;
    export_to_png(&cdx_file, Path::new("output.png"), &options)?;

    Ok(())
}
```

You can also run the included example:

```bash
cargo run --example export_cdx -- molecule.cdx output
```

This will create `output.svg` and `output.png` from `molecule.cdx`.

## CDX File Format

The CDX format is a binary format used by ChemDraw for storing chemical drawings. Key characteristics:

- **Byte order**: Little-endian
- **Structure**: Header followed by tagged items (objects and properties) in a tree structure
- **Header**: 22 bytes starting with magic string `VjCD0100`
- **Objects**: Identified by tag with bit15=1, contain an ID and nested content
- **Properties**: Identified by tag with bit15=0, contain typed data

## Supported Elements

| Element | Read | Write | Description |
|---------|------|-------|-------------|
| Document | ✓ | ✓ | Root document object |
| Page | ✓ | ✓ | Drawing pages |
| Fragment | ✓ | ✓ | Molecule fragments |
| Node | ✓ | ✓ | Atoms in molecules |
| Bond | ✓ | ✓ | Chemical bonds |
| Arrow | ✓ | ✓ | Reaction arrows |
| Graphic | ✓ | ✓ | Graphical elements |
| Text | ✓ | ✓ | Text annotations |
| Group | ✓ | ✓ | Grouped objects |
| Reaction Scheme | ✓ | ✓ | Reaction schemes |
| Reaction Step | ✓ | ✓ | Individual reaction steps |
| TLC Plate | ✓ | ✓ | TLC plate diagrams |
| Color Table | ✓ | ✓ | Color definitions |

## Running the Viewer

The library includes a built-in GUI viewer for CDX files:

```bash
cargo run --release
```

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the [GNU Lesser General Public License v3.0](LICENSE) (LGPL-3.0).

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
