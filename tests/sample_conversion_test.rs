/// Comprehensive test for converting all sample CDX files to SVG and PNG
///
/// This test iterates through all CDX files in the sample_cdx directory
/// and converts them to both SVG and PNG formats, saving outputs to test_outputs/

use cdx_file_rs::renderer::{export_to_png, export_to_svg, RenderExportOptions, Color};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn test_convert_all_sample_cdx_files() {
    // Create output directory
    let output_dir = PathBuf::from("test_outputs");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    // Find all CDX files in sample_cdx directory
    let sample_dir = PathBuf::from("sample_cdx");
    assert!(sample_dir.exists(), "sample_cdx directory should exist");

    let cdx_files: Vec<PathBuf> = fs::read_dir(&sample_dir)
        .expect("Failed to read sample_cdx directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("cdx"))
        .collect();

    assert!(!cdx_files.is_empty(), "Should have at least one CDX file");
    println!("Found {} CDX files to convert", cdx_files.len());

    // Configure export options
    let mut options = RenderExportOptions::default();
    options.width = 1024;
    options.height = 768;
    options.margin = 50.0;
    options.background_color = Color::WHITE;

    let mut successful_conversions = 0;
    let mut failed_conversions = Vec::new();

    // Convert each CDX file
    for cdx_path in &cdx_files {
        let file_stem = cdx_path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Invalid file name");

        println!("\nConverting: {}", file_stem);

        // Read and parse CDX file
        let data = match fs::read(cdx_path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("  ✗ Failed to read {}: {}", file_stem, e);
                failed_conversions.push((file_stem.to_string(), format!("Read error: {}", e)));
                continue;
            }
        };

        let cdx_file = match cdx_file_rs::cdx::file::CdxFile::from_bytes(&data) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("  ✗ Failed to parse {}: {}", file_stem, e);
                failed_conversions.push((file_stem.to_string(), format!("Parse error: {}", e)));
                continue;
            }
        };

        // Export to SVG
        let svg_path = output_dir.join(format!("{}.svg", file_stem));
        match export_to_svg(&cdx_file, &svg_path, &options) {
            Ok(_) => {
                let size = fs::metadata(&svg_path).map(|m| m.len()).unwrap_or(0);
                println!("  ✓ SVG: {} ({} bytes)", svg_path.display(), size);
            }
            Err(e) => {
                eprintln!("  ✗ SVG export failed: {}", e);
                failed_conversions.push((file_stem.to_string(), format!("SVG error: {}", e)));
                continue;
            }
        }

        // Export to PNG with higher resolution
        options.scale = 2.0;
        let png_path = output_dir.join(format!("{}.png", file_stem));
        match export_to_png(&cdx_file, &png_path, &options) {
            Ok(_) => {
                let size = fs::metadata(&png_path).map(|m| m.len()).unwrap_or(0);
                println!("  ✓ PNG: {} ({} bytes)", png_path.display(), size);
                successful_conversions += 1;
            }
            Err(e) => {
                eprintln!("  ✗ PNG export failed: {}", e);
                failed_conversions.push((file_stem.to_string(), format!("PNG error: {}", e)));
            }
        }
        options.scale = 1.0; // Reset scale
    }

    // Print summary
    println!("\n=== Conversion Summary ===");
    println!("Total files: {}", cdx_files.len());
    println!("Successful: {}", successful_conversions);
    println!("Failed: {}", failed_conversions.len());

    if !failed_conversions.is_empty() {
        println!("\nFailed conversions:");
        for (name, error) in &failed_conversions {
            println!("  - {}: {}", name, error);
        }
    }

    // Verify that output files were created
    let svg_count = fs::read_dir(&output_dir)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("svg"))
        .count();

    let png_count = fs::read_dir(&output_dir)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("png"))
        .count();

    println!("\nOutput files created:");
    println!("  SVG files: {}", svg_count);
    println!("  PNG files: {}", png_count);

    // Assert that at least some files were converted successfully
    assert!(
        successful_conversions > 0,
        "At least one CDX file should be converted successfully"
    );
    // Note: The output directory may contain additional files from other tests,
    // so we only check that we have at least as many files as successful conversions
    assert!(
        svg_count >= successful_conversions,
        "SVG count should be at least {}, got {}",
        successful_conversions,
        svg_count
    );
    assert!(
        png_count >= successful_conversions,
        "PNG count should be at least {}, got {}",
        successful_conversions,
        png_count
    );
}

#[test]
fn test_convert_specific_samples_with_different_options() {
    let output_dir = PathBuf::from("test_outputs");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    // Test benzene with different sizes
    let benzene_path = Path::new("sample_cdx/benzene.cdx");
    if !benzene_path.exists() {
        println!("Skipping: benzene.cdx not found");
        return;
    }

    let data = fs::read(benzene_path).expect("Failed to read benzene.cdx");
    let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data)
        .expect("Failed to parse benzene.cdx");

    // Small size
    let mut options = RenderExportOptions::default();
    options.width = 512;
    options.height = 384;
    options.margin = 20.0;

    let svg_path = output_dir.join("benzene_small.svg");
    export_to_svg(&cdx_file, &svg_path, &options).expect("Failed to export small SVG");
    println!("Created: {}", svg_path.display());

    // Large size
    options.width = 2048;
    options.height = 1536;
    options.margin = 100.0;

    let svg_path = output_dir.join("benzene_large.svg");
    export_to_svg(&cdx_file, &svg_path, &options).expect("Failed to export large SVG");
    println!("Created: {}", svg_path.display());

    // High resolution PNG
    options.scale = 4.0;
    let png_path = output_dir.join("benzene_hires.png");
    export_to_png(&cdx_file, &png_path, &options).expect("Failed to export high-res PNG");
    println!("Created: {}", png_path.display());

    assert!(svg_path.exists());
    assert!(png_path.exists());
}
