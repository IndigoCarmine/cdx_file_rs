/// Test for SVG and PNG export functionality

use cdx_file_rs::renderer::{export_to_png, export_to_svg, render_to_svg, RenderExportOptions};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_svg_export() {
    // Find a sample CDX file
    let sample_files: Vec<PathBuf> = fs::read_dir("sample_cdx")
        .expect("sample_cdx directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("cdx"))
        .collect();

    if sample_files.is_empty() {
        eprintln!("No CDX files found in sample_cdx directory, skipping test");
        return;
    }

    let sample_file = &sample_files[0];
    println!("Testing with file: {:?}", sample_file);

    // Read CDX file
    let data = fs::read(sample_file).expect("Failed to read sample file");
    let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data).expect("Failed to parse CDX file");

    // Test render to SVG string
    let options = RenderExportOptions::default();
    let svg_content = render_to_svg(&cdx_file, &options).expect("Failed to render to SVG");

    // Verify SVG content
    assert!(svg_content.contains("<svg"));
    assert!(svg_content.contains("</svg>"));
    println!("SVG content length: {} bytes", svg_content.len());

    // Test export to SVG file
    let output_path = std::env::temp_dir().join("test_output.svg");
    export_to_svg(&cdx_file, &output_path, &options).expect("Failed to export to SVG");

    // Verify file was created
    assert!(output_path.exists());
    let written_content = fs::read_to_string(&output_path).expect("Failed to read SVG file");
    assert_eq!(svg_content, written_content);

    // Clean up
    fs::remove_file(output_path).ok();
}

#[test]
fn test_png_export() {
    // Find a sample CDX file
    let sample_files: Vec<PathBuf> = fs::read_dir("sample_cdx")
        .expect("sample_cdx directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("cdx"))
        .collect();

    if sample_files.is_empty() {
        eprintln!("No CDX files found in sample_cdx directory, skipping test");
        return;
    }

    let sample_file = &sample_files[0];
    println!("Testing with file: {:?}", sample_file);

    // Read CDX file
    let data = fs::read(sample_file).expect("Failed to read sample file");
    let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data).expect("Failed to parse CDX file");

    // Test export to PNG file
    let output_path = std::env::temp_dir().join("test_output.png");
    let options = RenderExportOptions::default();
    export_to_png(&cdx_file, &output_path, &options).expect("Failed to export to PNG");

    // Verify file was created and is not empty
    assert!(output_path.exists());
    let metadata = fs::metadata(&output_path).expect("Failed to get PNG file metadata");
    assert!(metadata.len() > 0, "PNG file should not be empty");
    println!("PNG file size: {} bytes", metadata.len());

    // Clean up
    fs::remove_file(output_path).ok();
}

#[test]
fn test_svg_export_with_custom_options() {
    // Find a sample CDX file
    let sample_files: Vec<PathBuf> = fs::read_dir("sample_cdx")
        .expect("sample_cdx directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("cdx"))
        .collect();

    if sample_files.is_empty() {
        eprintln!("No CDX files found in sample_cdx directory, skipping test");
        return;
    }

    let sample_file = &sample_files[0];

    // Read CDX file
    let data = fs::read(sample_file).expect("Failed to read sample file");
    let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data).expect("Failed to parse CDX file");

    // Test with custom options
    let mut options = RenderExportOptions::default();
    options.width = 1024;
    options.height = 768;
    options.margin = 50.0;

    let svg_content = render_to_svg(&cdx_file, &options).expect("Failed to render to SVG");

    // Verify SVG uses custom dimensions
    assert!(svg_content.contains("width=\"1024\""));
    assert!(svg_content.contains("height=\"768\""));
}
