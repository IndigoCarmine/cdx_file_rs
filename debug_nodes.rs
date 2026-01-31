use std::fs;
use cdx_file_rs::cdx::file::{CdxFile, NodePayload};

fn main() {
    let data = fs::read("sample_cdx/benzene.cdx").expect("Failed to read file");
    let cdx = CdxFile::from_bytes(&data).expect("Failed to parse CDX");
    
    println!("Loaded CDX file");
    
    let root = cdx.tree.root();
    print_nodes(root, 0);
}

fn print_nodes(node: dendron::Node<NodePayload>, depth: usize) {
    let indent = "  ".repeat(depth);
    let data = node.borrow_data();
    
    match &*data {
        NodePayload::Node(n) => {
            println!("{}Node ID={}, position_2d={:?}, position_3d={:?}", 
                     indent, n.id, n.position_2d, n.position_3d);
        }
        NodePayload::Bond(b) => {
            println!("{}Bond ID={}, {} -> {}", indent, b.id, b.begin, b.end);
        }
        NodePayload::Fragment(f) => {
            println!("{}Fragment ID={}", indent, f.id);
        }
        NodePayload::Page(p) => {
            println!("{}Page ID={}", indent, p.id);
        }
        NodePayload::Document(d) => {
            println!("{}Document", indent);
        }
        _ => {
            println!("{}Other: {:?}", indent, std::mem::discriminant(&*data));
        }
    }
    
    for child in node.children() {
        print_nodes(child, depth + 1);
    }
}
