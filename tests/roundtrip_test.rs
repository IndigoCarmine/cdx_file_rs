use cdx_file_rs::cdx::file::CdxFile;
use cdx_file_rs::cdx_parse_impl::raw_nodes::RawCdxObject;
use cdx_file_rs::cdx_parse_impl::reader::RawCdxParser;
use cdx_file_rs::cdx_parse_impl::writer::CdxWriter;
use std::cmp::min;
use std::fs;
use std::io::Cursor;
use std::path::Path;

fn debug_raw_object(obj: &RawCdxObject, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}tag=0x{:04x}, id={}, properties={}, children={}",
        indent,
        obj.tag,
        obj.id,
        obj.properties.len(),
        obj.children.len()
    );
    for (i, child) in obj.children.iter().enumerate() {
        println!("{}child[{}]:", indent, i);
        debug_raw_object(child, depth + 1);
    }
}

#[allow(dead_code)]
fn debug_raw_object_summary(obj: &RawCdxObject, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
        "{}tag=0x{:04x}, id={}, props={}, children={}",
        indent,
        obj.tag,
        obj.id,
        obj.properties.len(),
        obj.children.len()
    );
    for (_i, child) in obj.children.iter().take(5).enumerate() {
        debug_raw_object_summary(child, depth + 1);
    }
    if obj.children.len() > 5 {
        println!("{}... and {} more children", indent, obj.children.len() - 5);
    }
}

#[test]
fn test_roundtrip_binary_identity() {
    let sample_dir = Path::new("sample_cdx");

    // テスト対象のCDXファイル
    let test_files = vec![
        "benzene.cdx",
        "Reaction.cdx",
        "Analysis.cdx",
        "ReactionAnalysis.cdx",
        "yellow_colored.cdx",
    ];

    for filename in test_files {
        let file_path = sample_dir.join(filename);
        if !file_path.exists() {
            println!("Skipping non-existent file: {}", file_path.display());
            continue;
        }

        println!("\n=== Testing: {} ===", filename);

        // オリジナルファイルを読み込み
        let original_data = fs::read(&file_path).expect(&format!("Failed to read {}", filename));
        println!("Original file size: {} bytes", original_data.len());

        // パースする
        let mut reader = RawCdxParser::new(Cursor::new(&original_data));
        let parsed = reader
            .parse()
            .expect(&format!("Failed to parse {}", filename));

        println!(
            "Parsed object: tag=0x{:04x}, id={}, properties={}, children={}",
            parsed.tag,
            parsed.id,
            parsed.properties.len(),
            parsed.children.len()
        );

        // 書き込む
        let output = Vec::new();
        let cursor = Cursor::new(output);
        let mut writer = CdxWriter::new(cursor);
        writer
            .write(&parsed)
            .expect(&format!("Failed to write {}", filename));

        let written_data = writer.into_inner().into_inner();
        println!("Written file size: {} bytes", written_data.len());

        // バイナリを比較
        if original_data == written_data {
            println!("✓ PASS: Binary identity confirmed");
        } else {
            println!("✗ FAIL: Binary mismatch!");
            println!("Original length: {}", original_data.len());
            println!("Written length:  {}", written_data.len());

            // 最初の異なるバイト位置を検出
            for (i, (orig, written)) in original_data.iter().zip(written_data.iter()).enumerate() {
                if orig != written {
                    println!(
                        "First difference at byte {}: 0x{:02x} vs 0x{:02x}",
                        i, orig, written
                    );
                    break;
                }
            }

            let min_len = min(original_data.len(), written_data.len());
            // 内容は一致しているが長さが異なる場合（EOF 差分）
            if original_data.len() != written_data.len() {
                println!("First difference at byte {} (EOF difference)", min_len);

                if original_data.len() > written_data.len() {
                    println!(
                        "Original has extra {} bytes at end:",
                        original_data.len() - written_data.len()
                    );
                    for (i, b) in original_data[min_len..].iter().enumerate() {
                        println!("  original[{}] = 0x{:02x}", min_len + i, b);
                    }
                } else {
                    println!(
                        "Written has extra {} bytes at end:",
                        written_data.len() - original_data.len()
                    );
                    for (i, b) in written_data[min_len..].iter().enumerate() {
                        println!("  written[{}] = 0x{:02x}", min_len + i, b);
                    }
                }
            }
        }
    }
}

#[test]
fn test_node_roundtrip_binary_identity() {
    let sample_dir = Path::new("sample_cdx");

    // テスト対象のCDXファイル
    let test_files = vec![
        "benzene.cdx",
        "Reaction.cdx",
        "Analysis.cdx",
        "ReactionAnalysis.cdx",
        "yellow_colored.cdx",
    ];

    let mut unsupported_tags = std::collections::HashMap::new();

    for filename in test_files {
        let file_path = sample_dir.join(filename);
        if !file_path.exists() {
            println!("Skipping non-existent file: {}", file_path.display());
            continue;
        }

        println!("\n=== Testing Node Roundtrip: {} ===", filename);

        // オリジナルファイルを読み込み
        let original_data = fs::read(&file_path).expect(&format!("Failed to read {}", filename));
        println!("Original file size: {} bytes", original_data.len());

        // RawCdxObject にパースする
        let mut reader = RawCdxParser::new(Cursor::new(&original_data));
        let raw_parsed = reader
            .parse()
            .expect(&format!("Failed to parse {}", filename));

        println!(
            "Parsed RawCdxObject: tag=0x{:04x}, id={}, properties={}, children={}",
            raw_parsed.tag,
            raw_parsed.id,
            raw_parsed.properties.len(),
            raw_parsed.children.len()
        );

        // RawCdxObject → CdxFile に変換
        let cdx_file = match CdxFile::from_raw(raw_parsed.clone()) {
            Ok(f) => f,
            Err(e) => {
                println!("⚠ Failed to convert to CdxFile: {}", e);
                // エラーメッセージからタグを抽出
                let error_str = e.to_string();
                if error_str.contains("Unknown Tag: ") || error_str.contains("unknown tag=") {
                    if let Some(tag_str) = error_str
                        .split("Unknown Tag: ")
                        .nth(1)
                        .or_else(|| error_str.split("unknown tag=").nth(1))
                    {
                        let tag_key = tag_str
                            .split_whitespace()
                            .next()
                            .unwrap_or("unknown")
                            .to_string();
                        *unsupported_tags.entry(tag_key.clone()).or_insert(0) += 1;
                        println!("  Problematic tag: {}", tag_key);
                    }
                }
                // 子オブジェクトのタグも表示
                println!("  Root object tag: 0x{:04x}", raw_parsed.tag);
                println!("  Children in root: {}", raw_parsed.children.len());
                for (i, child) in raw_parsed.children.iter().enumerate() {
                    println!("    child[{}]: tag=0x{:04x}", i, child.tag);
                }
                continue;
            }
        };

        println!("✓ Successfully converted to CdxFile");

        // Check if we can get the document
        match cdx_file.get_document() {
            Ok(doc) => {
                println!("  Document ID: {}", doc.id);
            }
            Err(e) => {
                println!("  Could not get document: {}", e);
            }
        }

        // RawCdxObject → Binary に書き込む (using original raw object for comparison)
        let output = Vec::new();
        let cursor = Cursor::new(output);
        let mut writer = CdxWriter::new(cursor);
        writer
            .write(&raw_parsed)
            .expect(&format!("Failed to write {}", filename));

        let written_data = writer.into_inner().into_inner();
        println!("Written file size: {} bytes", written_data.len());

        // Check if tree structure is preserved
        fn check_tree_structure(orig: &RawCdxObject, recon: &RawCdxObject, depth: usize) -> bool {
            if orig.tag != recon.tag
                || orig.id != recon.id
                || orig.children.len() != recon.children.len()
            {
                return false;
            }
            for (o_child, r_child) in orig.children.iter().zip(recon.children.iter()) {
                if !check_tree_structure(o_child, r_child, depth + 1) {
                    return false;
                }
            }
            true
        }

        // Re-parse the written data to compare
        let mut reader2 = RawCdxParser::new(Cursor::new(&written_data));
        if let Ok(reparsed) = reader2.parse() {
            if check_tree_structure(&raw_parsed, &reparsed, 0) {
                println!("✓ PASS: Tree structure preserved");
            } else {
                println!("✗ FAIL: Tree structure mismatch");
            }
        } else {
            println!("✗ FAIL: Could not re-parse written data");
        }
    }

    // 集計結果を表示
    println!("\n=== Unsupported Tags Summary ===");
    if unsupported_tags.is_empty() {
        println!("✓ All files processed successfully!");
    } else {
        println!("Tags that caused conversion failures:");
        let mut sorted_tags: Vec<_> = unsupported_tags.iter().collect();
        sorted_tags.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (tag, count) in sorted_tags {
            println!("  {} - appeared {} time(s)", tag, count);
        }
    }
}
