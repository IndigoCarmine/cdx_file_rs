use cdx_file_rs::cdx::file::CdxFile;
use std::fs;

#[test]
fn debug_reaction_text() {
    let bytes = fs::read("sample_cdx/Reaction.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&bytes).expect("Failed to parse CDX file");

    use cdx_file_rs::cdx::file::NodePayload;
    let tree = &cdx_file.tree;
    let root = tree.root();

    // Dump document color table
    {
        let data = root.borrow_data();
        if let NodePayload::Document(doc) = &*data {
            println!("\n=== Document properties ===");
            println!("  label_color: {:?}", doc.label_color);
            println!("  caption_color: {:?}", doc.caption_color);
            println!("  label_size: {:?}", doc.label_size);
            println!("  caption_size: {:?}", doc.caption_size);
            if let Some(ct) = &doc.color_table {
                println!("  Color table ({} entries):", ct.colors.len());
                for (i, c) in ct.colors.iter().enumerate() {
                    println!("    [{}]: rgb({:.0},{:.0},{:.0})", i,
                        c.red * 255.0, c.green * 255.0, c.blue * 255.0);
                }
            } else {
                println!("  Color table: NONE (will use default)");
            }
        }
    }

    // Dump text objects and arrows
    dump_nodes(root.clone(), 0);
}

fn dump_nodes(
    node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>,
    depth: usize,
) {
    use cdx_file_rs::cdx::file::NodePayload;
    {
        let data = node.borrow_data();
        let indent = "  ".repeat(depth);
        match &*data {
            NodePayload::TextObject(t) => {
                println!("{}TextObject:", indent);
                println!("{}  position_2d: {:?}", indent, t.position_2d.as_ref().map(|p| (p.x / 65536.0, p.y / 65536.0)));
                println!("{}  visible: {:?}", indent, t.visible);
                if let Some(ref cdx_str) = t.text {
                    println!("{}  text: {:?}", indent, cdx_str.text);
                    println!("{}  style_runs ({}):", indent, cdx_str.style_runs.len());
                    for (i, run) in cdx_str.style_runs.iter().enumerate() {
                        println!("{}    [{}] char_idx={} font_idx={} face={:#06x} size={} color_idx={}",
                            indent, i, run.char_index, run.font_index,
                            run.font_face, run.font_size, run.color_index);
                    }
                }
            }
            NodePayload::Arrow(a) => {
                println!("{}Arrow:", indent);
                println!("{}  bounding_box: {:?}", indent, a.bounding_box.as_ref().map(|b| {
                    (b.left / 65536.0, b.top / 65536.0, b.right / 65536.0, b.bottom / 65536.0)
                }));
                println!("{}  head_3d: {:?}", indent, a.head_3d);
                println!("{}  tail_3d: {:?}", indent, a.tail_3d);
                println!("{}  arrowhead_head: {:?}", indent, a.arrowhead_head);
                println!("{}  line_width: {:?}", indent, a.line_width);
            }
            NodePayload::Node(n) => {
                if let Some(pos) = &n.position_2d {
                    println!("{}Node id={} pos=({:.2},{:.2})", indent, n.id, pos.x/65536.0, pos.y/65536.0);
                }
            }
            NodePayload::Graphic(g) => {
                println!("{}Graphic:", indent);
                println!("{}  graphic_type: {:?}", indent, g.graphic_type);
                println!("{}  arrow_type: {:?}", indent, g.arrow_type);
                println!("{}  line_width: {:?}", indent, g.line_width);
                println!("{}  bounding_box: {:?}", indent, g.bounding_box.as_ref().map(|b| {
                    (b.left / 65536.0, b.top / 65536.0, b.right / 65536.0, b.bottom / 65536.0)
                }));
                println!("{}  bounding_box_raw: {:?}", indent, g.bounding_box.as_ref().map(|b| {
                    (b.left, b.top, b.right, b.bottom)
                }));
                println!("{}  head_3d: {:?}", indent, g.head_3d);
                println!("{}  tail_3d: {:?}", indent, g.tail_3d);
                println!("{}  arrowhead_size: {:?}", indent, g.arrowhead_size);
            }
            other => {
                let type_name = match other {
                    NodePayload::Document(_) => "Document",
                    NodePayload::Page(_) => "Page",
                    NodePayload::Fragment(_) => "Fragment",
                    NodePayload::Bond(_) => "Bond",
                    NodePayload::Group(_) => "Group",
                    NodePayload::ReactionScheme(_) => "ReactionScheme",
                    NodePayload::ReactionStep(_) => "ReactionStep",
                    NodePayload::Curve(_) => "Curve",
                    _ => "Other",
                };
                if depth <= 4 {
                    println!("{}{}", indent, type_name);
                }
            }
        }
    }
    for child in node.children() {
        dump_nodes(child, depth + 1);
    }
}

#[test]
fn debug_benzene_bonds() {
    let bytes = fs::read("sample_cdx/benzene.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&bytes).expect("Failed to parse CDX file");

    use cdx_file_rs::cdx::file::NodePayload;
    let tree = &cdx_file.tree;
    let root = tree.root();

    println!("\n=== benzene.cdx structure ===");
    dump_benzene(root.clone(), 0);
}

fn dump_benzene(node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>, depth: usize) {
    use cdx_file_rs::cdx::file::NodePayload;
    let indent = "  ".repeat(depth);
    {
        let data = node.borrow_data();
        match &*data {
            NodePayload::Document(doc) => {
                println!("{}Document:", indent);
                println!("{}  line_width={:?} bond_spacing={:?} bond_length={:?}",
                    indent, doc.line_width, doc.bond_spacing, doc.bond_length);
            }
            NodePayload::Node(n) => {
                if let Some(pos) = &n.position_2d {
                    println!("{}Node id={} pos=({:.2},{:.2}) line_width={:?}", indent, n.id, pos.x/65536.0, pos.y/65536.0, n.line_width);
                }
            }
            NodePayload::Bond(b) => {
                println!("{}Bond begin={:?} end={:?} order={:?}", indent, b.begin, b.end, b.bond_order);
                println!("{}  double_position={:?} bond_spacing={:?} bond_spacing_abs={:?} line_width={:?}",
                    indent, b.double_position, b.bond_spacing, b.bond_spacing_abs, b.line_width);
            }
            other => {
                let type_name = match other {
                    NodePayload::Document(_) => "Document",
                    NodePayload::Page(_) => "Page",
                    NodePayload::Fragment(_) => "Fragment",
                    _ => "Other",
                };
                if depth <= 3 { println!("{}{}", indent, type_name); }
            }
        }
    }
    for child in node.children() {
        dump_benzene(child, depth + 1);
    }
}

#[test]
fn debug_analysis_text() {
    let bytes = fs::read("sample_cdx/Analysis.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&bytes).expect("Failed to parse CDX file");

    use cdx_file_rs::cdx::file::NodePayload;
    let tree = &cdx_file.tree;
    let root = tree.root();

    println!("\n=== Analysis.cdx text objects ===");
    dump_nodes(root.clone(), 0);
}

#[test]
fn debug_coordinate_system() {
    let bytes = fs::read("sample_cdx/Analysis.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&bytes).expect("Failed to parse CDX file");

    use cdx_file_rs::cdx::file::NodePayload;
    let tree = &cdx_file.tree;
    let root = tree.root();

    let mut atom_positions: Vec<(u32, f64, f64)> = Vec::new();
    let mut fragment_bbox: Option<(f64, f64, f64, f64)> = None;

    collect_coords(root.clone(), &mut atom_positions, &mut fragment_bbox);

    println!("\n=== Coordinate System Analysis ===");
    if let Some((left, top, right, bottom)) = fragment_bbox {
        println!("Fragment bbox (CDX pts): left={:.3}, top={:.3}, right={:.3}, bottom={:.3}",
            left / 65536.0, top / 65536.0, right / 65536.0, bottom / 65536.0);
    }

    println!("\nAtom CDX pt positions:");
    for (id, x, y) in &atom_positions {
        println!("  Atom {}: x={:.3}, y={:.3} CDX pts", id, x / 65536.0, y / 65536.0);
    }

    // Simulate FIXED calculate_auto_scale (converts to CDX pts like core.rs does)
    if !atom_positions.is_empty() {
        let min_x = atom_positions.iter().map(|(_, x, _)| x / 65536.0).fold(f64::INFINITY, f64::min);
        let max_x = atom_positions.iter().map(|(_, x, _)| x / 65536.0).fold(f64::NEG_INFINITY, f64::max);
        let min_y = atom_positions.iter().map(|(_, _, y)| y / 65536.0).fold(f64::INFINITY, f64::min);
        let max_y = atom_positions.iter().map(|(_, _, y)| y / 65536.0).fold(f64::NEG_INFINITY, f64::max);

        let doc_width = max_x - min_x;
        let doc_height = max_y - min_y;
        let available = 760.0f64;
        let auto_scale = (available / doc_width).min(available / doc_height);

        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;
        let origin_x = 400.0 - center_x * auto_scale;
        let origin_y = 300.0 - center_y * auto_scale;

        println!("\n=== FIXED Auto-scale (CDX pts-based, matching core.rs) ===");
        println!("  doc range: x=[{:.3}, {:.3}], y=[{:.3}, {:.3}] CDX pts",
            min_x, max_x, min_y, max_y);
        println!("  doc_width={:.3} CDX pts, doc_height={:.3} CDX pts", doc_width, doc_height);
        println!("  auto_scale={:.4} px/CDX-pt", auto_scale);
        println!("  origin=({:.2}, {:.2}) px", origin_x, origin_y);

        // Verify: bond start (uses to_backend_point, which is just CDX pts here) on screen
        let leftmost_cdx_x = min_x;
        let bond_screen_x = origin_x + leftmost_cdx_x * auto_scale;
        println!("\n  Bond start (leftmost, via to_backend_point) on screen: x={:.2} px", bond_screen_x);

        // Fragment bbox on screen (after to_backend_rect → CDX pts)
        if let Some((left, _top, right, _bottom)) = fragment_bbox {
            let bbox_left_cdx = left / 65536.0;
            let bbox_right_cdx = right / 65536.0;
            let bbox_screen_left = origin_x + bbox_left_cdx * auto_scale;
            let bbox_screen_right = origin_x + bbox_right_cdx * auto_scale;
            println!("  Fragment bbox on screen: x=[{:.2}, {:.2}] px", bbox_screen_left, bbox_screen_right);
            println!("  Molecule starts at: {:.2} px, Box left at: {:.2} px",
                bond_screen_x, bbox_screen_left);
            if bond_screen_x >= bbox_screen_left {
                println!("  ✓ Leftmost atom IS inside the fragment bbox");
            } else {
                println!("  ✗ Leftmost atom is OUTSIDE (left of) the fragment bbox!");
            }
        }
    }
}

#[test]
fn debug_reactionanalysis_grid() {
    let bytes = fs::read("sample_cdx/ReactionAnalysis.cdx").expect("Failed to read file");
    let cdx_file = CdxFile::from_bytes(&bytes).expect("Failed to parse CDX file");

    use cdx_file_rs::cdx::file::NodePayload;
    let tree = &cdx_file.tree;
    let root = tree.root();

    println!("\n=== ReactionAnalysis.cdx grid structure ===");
    dump_grid(root.clone(), 0);
}

fn dump_grid(node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>, depth: usize) {
    use cdx_file_rs::cdx::file::NodePayload;
    let indent = "  ".repeat(depth);
    {
        let data = node.borrow_data();
        match &*data {
            NodePayload::StoichiometryGrid(sg) => {
                println!("{}StoichiometryGrid id={}", indent, sg.id);
            }
            NodePayload::SegComponent(sc) => {
                println!("{}SegComponent id={} width={:?} is_header={:?} is_reactant={:?}",
                    indent, sc.id, sc.width, sc.component_is_header, sc.component_is_reactant);
            }
            NodePayload::SegDatum(sd) => {
                println!("{}SegDatum id={} type={:?}", indent, sd.id, sd.sg_data_type);
            }
            NodePayload::TextObject(t) => {
                let text = t.text.as_ref().map(|s| s.text.as_str()).unwrap_or("");
                let bbox = t.bounding_box.as_ref().map(|b| {
                    (b.left/65536.0, b.top/65536.0, b.right/65536.0, b.bottom/65536.0)
                });
                let pos = t.position_2d.as_ref().map(|p| (p.x/65536.0, p.y/65536.0));
                println!("{}TextObject id=? pos={:?} bbox={:?} vis={:?} text={:?}",
                    indent, pos, bbox, t.visible, &text[..text.len().min(30)]);
            }
            NodePayload::TextObject(t) => {
                let text = t.text.as_ref().map(|s| s.text.as_str()).unwrap_or("");
                let bbox = t.bounding_box.as_ref().map(|b| {
                    (b.left/65536.0, b.top/65536.0, b.right/65536.0, b.bottom/65536.0)
                });
                let pos = t.position_2d.as_ref().map(|p| (p.x/65536.0, p.y/65536.0));
                println!("{}TextObject pos={:?} bbox={:?} vis={:?} text={:?}",
                    indent, pos, bbox, t.visible, &text[..text.len().min(40)]);
            }
            other => {
                let type_name = match other {
                    NodePayload::Document(_) => "Document",
                    NodePayload::Page(_) => "Page",
                    NodePayload::Fragment(_) => "Fragment",
                    NodePayload::ReactionScheme(_) => "ReactionScheme",
                    NodePayload::ReactionStep(_) => "ReactionStep",
                    _ => "Other",
                };
                if depth <= 3 { println!("{}{}", indent, type_name); }
            }
        }
    }
    for child in node.children() {
        dump_grid(child, depth + 1);
    }
}

fn collect_coords(
    node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>,
    atoms: &mut Vec<(u32, f64, f64)>,
    frag_bbox: &mut Option<(f64, f64, f64, f64)>,
) {
    use cdx_file_rs::cdx::file::NodePayload;
    {
        let data = node.borrow_data();
        match &*data {
            NodePayload::Node(n) => {
                if let Some(pos) = &n.position_2d {
                    atoms.push((n.id, pos.x, pos.y));
                }
            }
            NodePayload::Fragment(f) => {
                if frag_bbox.is_none() {
                    if let Some(bbox) = &f.bounding_box {
                        *frag_bbox = Some((bbox.left, bbox.top, bbox.right, bbox.bottom));
                    }
                }
            }
            _ => {}
        }
    }
    for child in node.children() {
        collect_coords(child, atoms, frag_bbox);
    }
}
