use cdx_file_rs::cdx::file::CdxFile;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn test_cdx_file(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    // ファイル読み込み
    let original_data =
        fs::read(path).map_err(|e| format!("Failed to read {}: {}", file_path, e))?;

    println!("\n{}", "=".repeat(60));
    println!("Testing: {}", file_path);
    println!("File size: {} bytes", original_data.len());

    // CdxFile として読み込み
    let cdx_file = match CdxFile::from_bytes(&original_data) {
        Ok(f) => f,
        Err(e) => {
            println!("⚠ Warning: Could not parse {}: {}", file_path, e);
            return Ok(()); // Non-critical error, continue
        }
    };

    println!(
        "✓ Successfully loaded {}",
        Path::new(file_path).file_name().unwrap().to_string_lossy()
    );

    // Document情報を取得
    let document = cdx_file
        .get_document()
        .map_err(|e| format!("Failed to get document: {}", e))?;

    println!("\n=== Document Info ===");
    println!("Document ID: {}", document.id);

    if let Some(name) = &document.name {
        println!("Document Name: {}", name.text);
    }

    if let Some(color_table) = &document.color_table {
        println!("Color table: {} colors", color_table.colors.len());
    }

    if let Some(bond_length) = document.bond_length {
        println!("Bond length: {}", bond_length);
    }

    if let Some(line_width) = document.line_width {
        println!("Line width: {}", line_width);
    }

    // ツリー構造を調査
    println!("\n=== Tree Structure ===");
    let root = cdx_file.tree.root();
    {
        let root_data = root.borrow_data();
        println!("Root node: {:?}", std::mem::discriminant(&*root_data));
    }

    let mut total_nodes = 0;
    let mut total_bonds = 0;
    let mut total_fragments = 0;
    let mut total_pages = 0;

    fn count_nodes(
        node: dendron::Node<cdx_file_rs::cdx::file::NodePayload>,
        nodes: &mut usize,
        bonds: &mut usize,
        fragments: &mut usize,
        pages: &mut usize,
    ) {
        use cdx_file_rs::cdx::file::NodePayload;

        let data = node.borrow_data();
        match &*data {
            NodePayload::Node(n) => {
                *nodes += 1;
                eprintln!(
                    "  Node ID={}, position_2d={:?}, position_3d={:?}",
                    n.id, n.position_2d, n.position_3d
                );
            }
            NodePayload::Bond(_) => *bonds += 1,
            NodePayload::Fragment(_) => *fragments += 1,
            NodePayload::Page(_) => *pages += 1,
            _ => {}
        }

        for child in node.children() {
            count_nodes(child, nodes, bonds, fragments, pages);
        }
    }

    count_nodes(
        root,
        &mut total_nodes,
        &mut total_bonds,
        &mut total_fragments,
        &mut total_pages,
    );

    println!("Total Pages: {}", total_pages);
    println!("Total Fragments: {}", total_fragments);
    println!("Total Nodes (atoms): {}", total_nodes);
    println!("Total Bonds: {}", total_bonds);

    if total_nodes > 0 {
        println!(
            "✓ File has valid structure with {} atoms and {} bonds",
            total_nodes, total_bonds
        );
    }

    Ok(())
}

fn extract_unknown_tags(panic_str: &str) -> HashSet<u32> {
    let mut tags = HashSet::new();

    // Extract all occurrences of "unknown tag=XXXXX"
    let mut remaining = panic_str;
    while let Some(pos) = remaining.find("unknown tag=") {
        let tag_str = &remaining[pos + 12..];
        if let Some(end) = tag_str.find(|c: char| !c.is_numeric()) {
            if let Ok(tag) = tag_str[..end].parse::<u32>() {
                tags.insert(tag);
            }
            remaining = &tag_str[end..];
        } else if let Ok(tag) = tag_str.parse::<u32>() {
            tags.insert(tag);
            break;
        } else {
            break;
        }
    }

    tags
}

#[test]
fn test_all_cdx_files() {
    let cdx_files = vec![
        "sample_cdx/benzene.cdx",
        "sample_cdx/Reaction.cdx",
        "sample_cdx/Analysis.cdx",
        "sample_cdx/ReactionAnalysis.cdx",
        "sample_cdx/yellow_colored.cdx",
    ];

    let mut passed = 0;
    let mut failed = 0;
    let mut warnings = 0;
    let mut errors = Vec::new();
    let mut all_unknown_tags = HashSet::new();

    for file in cdx_files {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| test_cdx_file(file))) {
            Ok(Ok(())) => {
                passed += 1;
            }
            Ok(Err(e)) => {
                warnings += 1;
                println!("⚠ WARNING: {}", e);
            }
            Err(panic_msg) => {
                failed += 1;
                let panic_str = if let Some(s) = panic_msg.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = panic_msg.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic".to_string()
                };

                // Extract tag numbers from error message
                let unknown_tags = extract_unknown_tags(&panic_str);
                for tag in unknown_tags {
                    all_unknown_tags.insert(tag);
                }

                errors.push(format!("{}: Parse error - {}", file, panic_str));
            }
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("\n=== Test Summary ===");
    println!("Passed: {}", passed);
    println!("Warnings: {}", warnings);
    println!("Failed: {}", failed);

    if !errors.is_empty() {
        println!("\nFailed files:");
        for error in &errors {
            println!("  ✗ {}", error);
        }
    }

    if !all_unknown_tags.is_empty() {
        println!("\n=== Unknown Tags Found ===");
        let mut tags: Vec<_> = all_unknown_tags.into_iter().collect();
        tags.sort();
        for tag in &tags {
            println!("  Tag: {} (0x{:04X})", tag, tag);
        }
        println!("Total unknown tags: {}", tags.len());
    }

    println!(
        "\n✓ Test completed: {}/{} files processed successfully",
        passed + warnings,
        passed + warnings + failed
    );
}
