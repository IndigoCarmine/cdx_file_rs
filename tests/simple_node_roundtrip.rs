use std::fs;
use std::path::Path;
use cdx_file_rs::cdx::file::CdxFile;

#[test]
fn test_benzene_cdx_loading() {
    let file_path = Path::new("sample_cdx/benzene.cdx");
    
    // ファイル読み込み
    let original_data = fs::read(file_path).expect("Failed to read file");
    
    println!("File size: {} bytes", original_data.len());
    
    // CdxFile として読み込み
    let cdx_file = CdxFile::from_bytes(&original_data).expect("Failed to parse CDX file");
    
    println!("✓ Successfully loaded benzene.cdx");
    
    // Document情報を取得
    let document = cdx_file.get_document().expect("Failed to get document");
    
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
                eprintln!("  Node ID={}, position_2d={:?}, position_3d={:?}", 
                         n.id, n.position_2d, n.position_3d);
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
    
    count_nodes(root, &mut total_nodes, &mut total_bonds, &mut total_fragments, &mut total_pages);
    
    println!("Total Pages: {}", total_pages);
    println!("Total Fragments: {}", total_fragments);
    println!("Total Nodes (atoms): {}", total_nodes);
    println!("Total Bonds: {}", total_bonds);
    
    // ベンゼンは6個の炭素原子と6個の結合を持つはず
    assert!(total_nodes >= 6, "Expected at least 6 nodes (carbon atoms), found {}", total_nodes);
    assert!(total_bonds >= 6, "Expected at least 6 bonds, found {}", total_bonds);
    
    println!("\n✓ benzene.cdx structure validated");
}
