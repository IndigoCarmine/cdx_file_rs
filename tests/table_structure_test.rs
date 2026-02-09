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

    match &*data {
        NodePayload::Document(d) => {
            println!("{}Document (id={})", indent, d.id);
            println!("{}  bounding_box: {:?}", indent, d.bounding_box);
            println!("{}  magnification: {:?}", indent, d.magnification);
        }
        NodePayload::Page(p) => {
            println!("{}Page (id={})", indent, p.id);
            println!("{}  bounding_box: {:?}", indent, p.bounding_box);
            println!("{}  bounds_in_parent: {:?}", indent, p.bounds_in_parent);
            println!("{}  width: {:?}, height: {:?}", indent, p.width, p.height);
        }
        NodePayload::Table(t) => {
            println!("{}TABLE (id={})", indent, t.id);
            println!("{}  bounding_box: {:?}", indent, t.bounding_box);
            println!("{}  line_width: {:?}", indent, t.line_width);
        }
        NodePayload::TextObject(t) => {
            println!("{}TextObject (id={})", indent, t.id);
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
            println!("{}ObjectTag (id={})", indent, ot.id);
            println!("{}  tag_name: {:?}", indent, ot.tag_name);
        }
        NodePayload::Fragment(f) => {
            println!("{}Fragment (id={})", indent, f.id);
        }
        NodePayload::Node(n) => {
            println!("{}Node (id={})", indent, n.id);
        }
        NodePayload::Bond(b) => {
            println!("{}Bond (id={})", indent, b.id);
        }
        NodePayload::Arrow(a) => {
            println!("{}Arrow (id={})", indent, a.id);
            println!("{}  bounding_box: {:?}", indent, a.bounding_box);
        }
        NodePayload::Graphic(g) => {
            println!("{}Graphic (id={})", indent, g.id);
            println!("{}  bounding_box: {:?}", indent, g.bounding_box);
        }
        NodePayload::ReactionScheme(rs) => {
            println!("{}ReactionScheme (id={})", indent, rs.id);
        }
        NodePayload::ReactionStep(rs) => {
            println!("{}ReactionStep (id={})", indent, rs.id);
        }
        NodePayload::Annotation(a) => {
            println!("{}Anotation (id={})", indent, a.id);
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
