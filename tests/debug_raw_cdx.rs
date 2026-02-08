use cdx_file_rs::cdx::file::CdxFile;
use cdx_file_rs::cdx::text::TextObject;
use cdx_file_rs::cdx_parse_impl::tagged_object::TaggedObject;
use cdx_file_rs::cdx_parse_impl::text;
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
            print_raw_cdx(&raw_doc, 0);
        }
        Err(e) => println!("✗ Parse error: {}", e),
    }
}

fn print_raw_cdx(obj: &cdx_file_rs::cdx_parse_impl::raw_nodes::RawCdxObject, indent: usize) {
    if obj.tag == 0x8006 {
        println!("#Tag: 0x{:04x}", obj.tag);
        println!("ID: {}", obj.id);
        println!("##Properties:");
        for prop in &obj.properties {
            println!("  Tag: 0x{:04x}, Value: {:?}", prop.tag, prop.value);
        }
        let text = TextObject::from_raw(obj.clone()).unwrap();
        println!("##Text: {:#?}", text);
    }
    println!("## Children:");
    for child in &obj.children {
        print_raw_cdx(child, indent + 2);
    }
}
