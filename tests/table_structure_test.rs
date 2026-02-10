use cdx_file_rs::{
    cdx::file::{CdxFile, NodePayload},
    node::Node,
};
use std::fs;

#[test]
fn inspect_table_structure() {
    let data = fs::read("sample_cdx/ReactionAnalysis.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&data).expect("Failed to parse CDX");

    println!("Inspecting ReactionAnalysis.cdx table structure:\n");

    let root = cdx_file.tree.root();
    inspect_node(&root, 0);
}

fn inspect_node(node: &dendron::Node<NodePayload>, depth: usize) {
    let indent = "  ".repeat(depth);
    let data = node.borrow_data();
    print!("Node tag=0x{:x} - ", data.tag());

    match &*data {
        NodePayload::Document(d) => {
            println!("{}Document (id={:x})", indent, d.id);
            println!("{}  bounding_box: {:?}", indent, d.bounding_box);
            println!("{}  magnification: {:?}", indent, d.magnification);
        }
        NodePayload::Page(p) => {
            println!("{}Page (id={:x})", indent, p.id);
            println!("{}  bounding_box: {:?}", indent, p.bounding_box);
            println!("{}  bounds_in_parent: {:?}", indent, p.bounds_in_parent);
            println!("{}  width: {:?}, height: {:?}", indent, p.width, p.height);
        }
        NodePayload::Table(t) => {
            println!("{}TABLE (id={:x})", indent, t.id);
            println!("{}  bounding_box: {:?}", indent, t.bounding_box);
            println!("{}  line_width: {:?}", indent, t.line_width);
        }
        NodePayload::TextObject(t) => {
            println!("{}TextObject (id={:x})", indent, t.id);
            println!("{}  bounding_box: {:?}", indent, t.bounding_box);
            println!("{}  position_2d: {:?}", indent, t.position_2d);
            if let Some(ref cdx_str) = t.text {
                let text_str = &cdx_str.text;
                if !text_str.is_empty() {
                    if text_str.len() > 50 {
                        println!("{}  text: \"{}...\"", indent, &text_str[..50]);
                    } else {
                        println!("{}  text: \"{}\"", indent, text_str);
                    }
                }
            }
        }
        NodePayload::ObjectTag(ot) => {
            println!("{}ObjectTag (id={:x})", indent, ot.id);
            println!("{}  tag_name: {:?}", indent, ot.name);
        }
        NodePayload::Fragment(f) => {
            println!("{}Fragment (id={:x})", indent, f.id);
        }
        NodePayload::Node(n) => {
            println!("{}Node (id={:x})", indent, n.id);
        }
        NodePayload::Bond(b) => {
            println!("{}Bond (id={:x})", indent, b.id);
        }
        NodePayload::Arrow(a) => {
            println!("{}Arrow (id={:x})", indent, a.id);
            println!("{}  bounding_box: {:?}", indent, a.bounding_box);
        }
        NodePayload::Graphic(g) => {
            println!("{}Graphic (id={:x})", indent, g.id);
            println!("{}  bounding_box: {:?}", indent, g.bounding_box);
        }
        NodePayload::ReactionScheme(rs) => {
            println!("{}ReactionScheme (id={:x})", indent, rs.id);
        }
        NodePayload::ReactionStep(rs) => {
            println!("{}ReactionStep (id={:x})", indent, rs.id);
        }
        NodePayload::Annotation(a) => {
            println!("{}Anotation (id={:x})", indent, a.id);
        }
        nodepayload => {
            println!("{}{:?}", indent, nodepayload);
        }
    }

    drop(data);

    for child in node.children() {
        inspect_node(&child, depth + 1);
    }
}
