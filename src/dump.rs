use cdx_file_rs::{CdxNode, CdxParser, Result};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "sample_cdx/benzene.cdx"
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut parser = CdxParser::new(reader);
    let doc = parser.parse()?;

    dump_node(
        &CdxNode::Object(cdx_file_rs::CdxObject {
            tag: 0,
            id: 0,
            children: doc.root,
        }),
        0,
    );

    Ok(())
}

fn dump_node(node: &CdxNode, indent: usize) {
    let space = "  ".repeat(indent);
    match node {
        CdxNode::Object(obj) => {
            println!("{}{:#06x} Object (ID: {})", space, obj.tag, obj.id);
            for child in &obj.children {
                dump_node(child, indent + 1);
            }
            println!("{}EndObject", space);
        }
        CdxNode::Property(prop) => match &prop.value {
            cdx_file_rs::CdxValue::Raw(data) => {
                println!(
                    "{}{:#06x} Property: Raw({:?}) len={} hex={}",
                    space,
                    prop.tag,
                    data,
                    data.len(),
                    data.iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            _ => println!("{}{:#06x} Property: {:?}", space, prop.tag, prop.value),
        },
    }
}
