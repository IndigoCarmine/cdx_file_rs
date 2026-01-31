#!/usr/bin/env python3
"""
Inspect Arrow objects (Tag 0x802B) in CDX files using Rust's CdxFile directly
"""

import subprocess
import json

# Create a simple Rust program to inspect Arrow objects
rust_code = '''
use std::io::Cursor;

fn main() {
    if let Ok(bytes) = std::fs::read("sample_cdx/Reaction.cdx") {
        println!("File size: {} bytes", bytes.len());
        
        match cdx_file_rs::cdx::file::CdxFile::from_bytes(&bytes) {
            Ok(cdx_file) => {
                println!("✓ Successfully parsed Reaction.cdx");
                inspect_tree(&cdx_file);
            },
            Err(e) => {
                println!("✗ Parse error: {}", e);
            }
        }
    }
}

fn inspect_tree(cdx_file: &cdx_file_rs::cdx::file::CdxFile) {
    use dendron::Node;
    use cdx_file_rs::cdx::file::NodePayload;
    
    let root = cdx_file.tree.root();
    
    fn traverse(node: &Node<NodePayload>, depth: usize) {
        let indent = "  ".repeat(depth);
        let payload = node.data();
        
        match payload {
            NodePayload::Arrow(arrow) => {
                println!("{}[ARROW] ID: {}", indent, arrow.id);
                if let Some(bounds) = &arrow.bounding_box {
                    println!("{}  Bounds: ({}, {}, {}, {})", indent, bounds.0, bounds.1, bounds.2, bounds.3);
                }
                if let Some(head) = &arrow.head_3d {
                    println!("{}  Head3D: ({}, {}, {})", indent, head.0, head.1, head.2);
                }
                if let Some(tail) = &arrow.tail_3d {
                    println!("{}  Tail3D: ({}, {}, {})", indent, tail.0, tail.1, tail.2);
                }
                if let Some(color) = &arrow.color {
                    println!("{}  Color: {}", indent, color);
                }
            },
            NodePayload::Bond(bond) => {
                println!("{}[BOND] ID: {} ({}-{})", indent, bond.id, bond.begin, bond.end);
            },
            NodePayload::Node(node) => {
                println!("{}[NODE] ID: {}", indent, node.id);
            },
            NodePayload::Document(doc) => {
                println!("{}[DOCUMENT] ID: {}", indent, doc.id);
            },
            NodePayload::Fragment(frag) => {
                println!("{}[FRAGMENT] ID: {}", indent, frag.id);
            },
            NodePayload::Page(page) => {
                println!("{}[PAGE] ID: {}", indent, page.id);
            },
            NodePayload::Group(group) => {
                println!("{}[GROUP] ID: {}", indent, group.id);
            },
            NodePayload::TextObject(text) => {
                println!("{}[TEXT] ID: {}", indent, text.id);
            },
            NodePayload::TlcLane(lane) => {
                println!("{}[TLC_LANE] ID: {}", indent, lane.id);
            },
            NodePayload::TLCPlate(plate) => {
                println!("{}[TLC_PLATE] ID: {}", indent, plate.id);
            },
        }
        
        for child in node.children() {
            traverse(&child, depth + 1);
        }
    }
    
    traverse(&root, 0);
}
'''

# Write to a temporary file and compile
print("Creating inspection tool...")
with open("src/bin/inspect_arrow.rs", "w") as f:
    f.write(rust_code)

print("Building inspection tool...")
result = subprocess.run(["cargo", "build", "--bin", "inspect_arrow", "--release"], 
                       capture_output=True, text=True, cwd=".")

if result.returncode != 0:
    print("Build error:")
    print(result.stderr)
    exit(1)

print("Running inspection tool...")
result = subprocess.run(["cargo", "run", "--bin", "inspect_arrow", "--release"],
                       capture_output=True, text=True, cwd=".")

print(result.stdout)
if result.stderr:
    print("Warnings/Errors:")
    print(result.stderr)

