#!/usr/bin/env python3
"""
Debug script to test CDX file loading and check what gets parsed
"""
import subprocess
import sys

# Create a test program to load and inspect the CDX file
test_code = r'''
use cdx_file_rs::cdx::file::CdxFile;
use std::path::Path;

fn main() {
    let path = Path::new("sample_cdx/Reaction.cdx");
    
    match CdxFile::from_file(path) {
        Ok(cdx_file) => {
            println!("✓ File loaded successfully");
            
            // Get the document
            match cdx_file.get_document() {
                Ok(doc) => {
                    println!("✓ Document found");
                    println!("  Document ID: {}", doc.id);
                    println!("  Has color_table: {}", doc.color_table.is_some());
                    
                    // Traverse tree and find Graphics
                    let tree = &cdx_file.tree;
                    let root = tree.root();
                    count_objects(root, 0);
                }
                Err(e) => println!("✗ Error getting document: {}", e),
            }
        }
        Err(e) => println!("✗ Error loading file: {}", e),
    }
}

fn count_objects(node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>, depth: usize) {
    let indent = "  ".repeat(depth);
    let data = node.borrow_data();
    
    match &*data {
        cdx_file_rs::cdx::file::NodePayload::Document(d) => println!("{}Document (id={})", indent, d.id),
        cdx_file_rs::cdx::file::NodePayload::Page(p) => println!("{}Page (id={})", indent, p.id),
        cdx_file_rs::cdx::file::NodePayload::Node(n) => println!("{}Node (id={})", indent, n.id),
        cdx_file_rs::cdx::file::NodePayload::Bond(b) => println!("{}Bond (id={})", indent, b.id),
        cdx_file_rs::cdx::file::NodePayload::Fragment(f) => println!("{}Fragment (id={})", indent, f.id),
        cdx_file_rs::cdx::file::NodePayload::Text(t) => println!("{}Text (id={})", indent, t.id),
        cdx_file_rs::cdx::file::NodePayload::Graphic(g) => {
            println!("{}Graphic (id={}) - WITH DETAIL", indent, g.id);
            println!("{}  graphic_type: {:?}", indent, g.graphic_type);
            println!("{}  bounding_box: {:?}", indent, g.bounding_box);
            println!("{}  head_3d: {:?}", indent, g.head_3d);
            println!("{}  tail_3d: {:?}", indent, g.tail_3d);
            println!("{}  arrow_type: {:?}", indent, g.arrow_type);
            println!("{}  line_width: {:?}", indent, g.line_width);
            println!("{}  foreground_color: {:?}", indent, g.foreground_color);
        }
        cdx_file_rs::cdx::file::NodePayload::Arrow(_) => println!("{}Arrow", indent),
        cdx_file_rs::cdx::file::NodePayload::Border(_) => println!("{}Border", indent),
        cdx_file_rs::cdx::file::NodePayload::Constraint(_) => println!("{}Constraint", indent),
        cdx_file_rs::cdx::file::NodePayload::Geometry(_) => println!("{}Geometry", indent),
        cdx_file_rs::cdx::file::NodePayload::Group(g) => println!("{}Group (id={})", indent, g.id),
        cdx_file_rs::cdx::file::NodePayload::ObjectTag(_) => println!("{}ObjectTag", indent),
        cdx_file_rs::cdx::file::NodePayload::ReactionScheme(_) => println!("{}ReactionScheme", indent),
        cdx_file_rs::cdx::file::NodePayload::ReactionStep(_) => println!("{}ReactionStep", indent),
        cdx_file_rs::cdx::file::NodePayload::TlcLane(_) => println!("{}TlcLane", indent),
        cdx_file_rs::cdx::file::NodePayload::TLCPlate(_) => println!("{}TLCPlate", indent),
        cdx_file_rs::cdx::file::NodePayload::UnknownObject802B(_) => println!("{}UnknownObject802B", indent),
    }
    
    for child in node.children() {
        count_objects(child, depth + 1);
    }
}
'''

# Write test file
test_path = "tests/inspect_cdx.rs"
with open(test_path, 'w') as f:
    f.write(test_code)

print("Created test file, running...")
result = subprocess.run(
    ['cargo', 'test', '--test', 'inspect_cdx', '--', '--nocapture'],
    cwd='.',
    capture_output=False,
    text=True
)

sys.exit(result.returncode)
