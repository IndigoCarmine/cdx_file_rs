use cdx_file_rs::cdx::file::CdxFile;
use std::path::Path;
use std::fs;

#[test]
fn inspect_reaction_cdx() {
    let path = Path::new("sample_cdx/Reaction.cdx");
    
    let bytes = fs::read(path).expect("Failed to read file");
    match CdxFile::from_bytes(&bytes) {
        Ok(cdx_file) => {
            println!("✓ File loaded successfully");
            
            // Traverse tree and find Graphics
            let tree = &cdx_file.tree;
            let root = tree.root();
            count_objects(root, 0);
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
        cdx_file_rs::cdx::file::NodePayload::TextObject(t) => println!("{}Text (id={})", indent, t.id),
        cdx_file_rs::cdx::file::NodePayload::Graphic(g) => {
            println!("{}Graphic (id={}) - WITH DETAIL", indent, g.id);
            println!("{}  graphic_type: {:?}", indent, g.graphic_type);
            println!("{}  arrow_type: {:?}", indent, g.arrow_type);
            println!("{}  arrowhead_size: {:?}", indent, g.arrowhead_size);
            println!("{}  bounding_box: {:?}", indent, g.bounding_box);
            println!("{}  head_3d: {:?}", indent, g.head_3d);
            println!("{}  tail_3d: {:?}", indent, g.tail_3d);
            println!("{}  line_width: {:?}", indent, g.line_width);
            println!("{}  foreground_color: {:?}", indent, g.foreground_color);
            println!("{}  background_color: {:?}", indent, g.background_color);
            println!("{}  visible: {:?}", indent, g.visible);
            println!("{}  superseded_by: {:?}", indent, g.superseded_by);
            println!("{}  z_order: {:?}", indent, g.z_order);
            
            // Count properties
            let prop_count = (g.z_order.is_some() as usize)
                + (g.visible.is_some() as usize)
                + (g.bounding_box.is_some() as usize)
                + (g.head_3d.is_some() as usize)
                + (g.tail_3d.is_some() as usize)
                + (g.graphic_type.is_some() as usize)
                + (g.arrow_type.is_some() as usize)
                + (g.line_width.is_some() as usize)
                + (g.foreground_color.is_some() as usize);
            println!("{}  Total properties set: {}", indent, prop_count);
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
