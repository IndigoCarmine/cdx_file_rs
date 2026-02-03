/// Example: Export CDX file to SVG and PNG
///
/// This example demonstrates how to use the export functionality to convert
/// CDX files to SVG and PNG formats using the abstracted rendering system.
///
/// Usage: cargo run --example export_cdx -- <input.cdx> [output_prefix]

use cdx_file_rs::renderer::{export_to_png, export_to_svg, RenderExportOptions, Color};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input.cdx> [output_prefix]", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} molecule.cdx output", args[0]);
        eprintln!("  (will create output.svg and output.png)");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_prefix = if args.len() > 2 {
        args[2].clone()
    } else {
        input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
            .to_string()
    };

    // Check if input file exists
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist", input_path.display());
        std::process::exit(1);
    }

    println!("Reading CDX file: {}", input_path.display());
    
    // Read and parse CDX file
    let data = fs::read(&input_path).expect("Failed to read input file");
    let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data)
        .expect("Failed to parse CDX file");

    println!("Successfully parsed CDX file");

    // Configure export options
    let mut options = RenderExportOptions::default();
    options.width = 1024;
    options.height = 768;
    options.margin = 50.0;
    options.background_color = Color::WHITE;
    
    // Export to SVG
    let svg_path = PathBuf::from(format!("{}.svg", output_prefix));
    println!("Exporting to SVG: {}", svg_path.display());
    
    export_to_svg(&cdx_file, &svg_path, &options)
        .expect("Failed to export to SVG");
    
    println!("✓ SVG export successful");

    // Export to PNG (with higher resolution)
    let png_path = PathBuf::from(format!("{}.png", output_prefix));
    println!("Exporting to PNG: {}", png_path.display());
    
    options.scale = 2.0; // Higher resolution for PNG
    export_to_png(&cdx_file, &png_path, &options)
        .expect("Failed to export to PNG");
    
    println!("✓ PNG export successful");

    // Display file sizes
    if let Ok(svg_meta) = fs::metadata(&svg_path) {
        println!("\nOutput files:");
        println!("  {} ({} bytes)", svg_path.display(), svg_meta.len());
    }
    if let Ok(png_meta) = fs::metadata(&png_path) {
        println!("  {} ({} bytes)", png_path.display(), png_meta.len());
    }

    println!("\n✓ Export completed successfully!");
}
