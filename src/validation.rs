use cdx_file_rs::{CdxParser, CdxWriter, Result};
use std::fs;
use std::io::{BufReader, BufWriter};

fn main() -> Result<()> {
    let dir = "sample_cdx";
    let entries = fs::read_dir(dir)?;

    let mut all_success = true;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("cdx") {
            if path.to_str().unwrap().contains("_out.cdx") {
                continue;
            }

            println!("--- Testing: {:?} ---", path.file_name().unwrap());

            let doc = {
                let file = fs::File::open(&path)?;
                let reader = BufReader::new(file);
                let mut parser = CdxParser::new(reader);
                parser.parse()?
            };

            let out_path = path.with_extension("out.cdx");
            {
                let file = fs::File::create(&out_path)?;
                let writer = BufWriter::new(file);
                let mut writer = CdxWriter::new(writer);
                writer.write(&doc)?;
            }

            // Compare
            let original = fs::read(&path)?;
            let written = fs::read(&out_path)?;

            if original == written {
                println!("SUCCESS: Round-trip matches perfectly.");
            } else {
                all_success = false;
                println!("FAILURE: Mismatch found.");
                println!("  Original size: {}", original.len());
                println!("  Written size:  {}", written.len());

                let min_len = original.len().min(written.len());
                for i in 0..min_len {
                    if original[i] != written[i] {
                        println!(
                            "  First mismatch at {:#x}: original={:#02x}, written={:#02x}",
                            i, original[i], written[i]
                        );
                        break;
                    }
                }
            }
            println!();
        }
    }

    if all_success {
        println!("All CDX files passed the round-trip test!");
    } else {
        println!("Some tests failed.");
    }

    Ok(())
}
