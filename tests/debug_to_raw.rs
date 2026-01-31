use std::fs;
use std::io::Cursor;
use std::path::Path;
use cdx_file_rs::cdx::reader::RawCdxParser;
use cdx_file_rs::cdx::file::Node;

#[test]
fn debug_document_to_raw() {
    let file_path = Path::new("sample_cdx/benzene.cdx");
    
    // ファイル読み込み
    let original_data = fs::read(file_path).expect("Failed to read file");
    
    // パース
    let mut reader = RawCdxParser::new(Cursor::new(&original_data));
    let raw_doc = reader.parse().expect("Failed to parse");
    
    println!("Original Document: {} properties", raw_doc.properties.len());
    for (i, prop) in raw_doc.properties.iter().enumerate() {
        println!("  [{}] tag=0x{:04x}", i, prop.tag);
    }
    
    // Node に変換
    let node = Node::from_raw(raw_doc.clone()).expect("Failed to convert to Node");
    println!("\nConverted to Node successfully");
    
    // RawCdxObject に戻す
    let reconstructed = node.to_raw().expect("Failed to reconstruct");
    
    println!("\nReconstructed Document: {} properties", reconstructed.properties.len());
    for (i, prop) in reconstructed.properties.iter().enumerate() {
        println!("  [{}] tag=0x{:04x}", i, prop.tag);
    }
    
    // Document オブジェクトを直接取得
    if let Some(obj) = node.object.as_document() {
        println!("\nDocument object fields:");
        println!("  creation_user_name: {:?}", obj.creation_user_name);
        println!("  name: {:?}", obj.name);
        
        // Document の to_raw を直接呼ぶ
        let doc_raw = obj.to_raw().expect("Failed to convert Document to raw");
        println!("\nDirect Document.to_raw(): {} properties", doc_raw.properties.len());
    }
    
    // 0x0008 (NAME) プロパティを確認
    println!("\nRaw properties containing NAME tag (0x0008):");
    for prop in &raw_doc.properties {
        if prop.tag == 0x0008 {
            println!("  Found NAME property: {:?}", prop.value);
        }
    }
    
    // Documentの直接パース
    println!("\nDirect Document parsing from raw:");
    use cdx_file_rs::TaggedObject;
    let doc = cdx_file_rs::Document::from_raw(raw_doc.clone()).expect("Failed to parse Document");
    println!("  name: {:?}", doc.name);
    println!("  creation_user_name: {:?}", doc.creation_user_name);
}
