use cdx_file_rs::cdx::file::CdxFile;
use std::fs;

#[test]
fn debug_raw_cdx() {
    let bytes = fs::read("sample_cdx/Reaction.cdx").expect("Failed to read file");

    // Parse raw without converting to typed objects
    use cdx_file_rs::cdx_parse_impl::reader::RawCdxParser;
    use std::io::Cursor;

    let mut parser = RawCdxParser::new(Cursor::new(&bytes));
    match parser.parse() {
        Ok(raw_doc) => {
            println!("✓ Raw document parsed");
            println!("  Tag: 0x{:04x}", raw_doc.tag);
            println!("  ID: {}", raw_doc.id);
            println!("  Properties: {}", raw_doc.properties.len());
            println!("  Children: {}", raw_doc.children.len());

            // Find Graphic with id=89
            find_and_print_graphic(&raw_doc);
        }
        Err(e) => println!("✗ Parse error: {}", e),
    }
}

fn find_and_print_graphic(obj: &cdx_file_rs::cdx_parse_impl::raw_nodes::RawCdxObject) {
    if obj.id == 89 {
        println!("\n=== Found Graphic (id=89) ===");
        println!("Tag: 0x{:04x}", obj.tag);
        println!("Properties ({} total):", obj.properties.len());

        for (i, prop) in obj.properties.iter().enumerate() {
            println!(
                "  [{}] Tag=0x{:04x}, Value length={} bytes",
                i,
                prop.tag,
                prop.value.len()
            );

            // Show hex for first 40 bytes of each property
            let display_len = std::cmp::min(40, prop.value.len());
            let hex_str = prop.value[..display_len]
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");
            println!("      Data: {}", hex_str);

            // Try to identify common tags
            match prop.tag {
                0x0A00 => println!("      -> GraphicType"),
                0x0A02 => println!("      -> ArrowType"),
                0x000A => println!("      -> ZOrder"),
                0x0204 => println!("      -> BoundingBox (should be 4 * f64 = 32 bytes)"),
                0x0207 => println!("      -> Head3D (should be 3 * f64 = 24 bytes)"),
                0x0208 => println!("      -> Tail3D (should be 3 * f64 = 24 bytes)"),
                0x0807 => println!("      -> LineWidth (should be f64 = 8 bytes)"),
                0x0301 => println!("      -> ForegroundColor (should be u16 = 2 bytes)"),
                _ => {}
            }
        }

        println!("Children: {}", obj.children.len());
        for child in &obj.children {
            println!("  Child: Tag=0x{:04x}, ID={}", child.tag, child.id);
        }
    }

    // Recurse
    for child in &obj.children {
        find_and_print_graphic(child);
    }
}
