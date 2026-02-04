/// Test for SVG and PNG export functionality

use cdx_file_rs::renderer::{export_to_png, export_to_svg, render_to_svg, RenderExportOptions};
use std::fs;
use std::path::PathBuf;

/// Get the test output directory path
fn test_output_dir() -> PathBuf {
    let path = PathBuf::from("test_outputs");
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create test_outputs directory");
    }
    path
}

/// Get all CDX sample files
fn get_sample_cdx_files() -> Vec<PathBuf> {
    fs::read_dir("sample_cdx")
        .expect("sample_cdx directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("cdx"))
        .collect()
}

#[test]
fn test_svg_export_all() {
    let sample_files = get_sample_cdx_files();

    if sample_files.is_empty() {
        eprintln!("No CDX files found in sample_cdx directory, skipping test");
        return;
    }

    let options = RenderExportOptions::default();

    for sample_file in &sample_files {
        let file_stem = sample_file.file_stem().unwrap().to_str().unwrap();
        println!("Processing SVG: {:?}", sample_file);

        // Read CDX file
        let data = fs::read(sample_file).expect("Failed to read sample file");
        let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data).expect("Failed to parse CDX file");

        // Test render to SVG string
        let svg_content = render_to_svg(&cdx_file, &options).expect("Failed to render to SVG");

        // Verify SVG content
        assert!(svg_content.contains("<svg"));
        assert!(svg_content.contains("</svg>"));

        // Export to SVG file with same name as CDX
        let output_path = test_output_dir().join(format!("{}.svg", file_stem));
        export_to_svg(&cdx_file, &output_path, &options).expect("Failed to export to SVG");

        // Verify file was created
        assert!(output_path.exists());
        println!("  -> {} ({} bytes)", output_path.display(), svg_content.len());
    }

    println!("Exported {} SVG files", sample_files.len());
}

#[test]
fn test_png_export_all() {
    let sample_files = get_sample_cdx_files();

    if sample_files.is_empty() {
        eprintln!("No CDX files found in sample_cdx directory, skipping test");
        return;
    }

    let options = RenderExportOptions::default();

    for sample_file in &sample_files {
        let file_stem = sample_file.file_stem().unwrap().to_str().unwrap();
        println!("Processing PNG: {:?}", sample_file);

        // Read CDX file
        let data = fs::read(sample_file).expect("Failed to read sample file");
        let cdx_file = cdx_file_rs::cdx::file::CdxFile::from_bytes(&data).expect("Failed to parse CDX file");

        // Export to PNG file with same name as CDX
        let output_path = test_output_dir().join(format!("{}.png", file_stem));
        export_to_png(&cdx_file, &output_path, &options).expect("Failed to export to PNG");

        // Verify file was created and is not empty
        assert!(output_path.exists());
        let metadata = fs::metadata(&output_path).expect("Failed to get PNG file metadata");
        assert!(metadata.len() > 0, "PNG file should not be empty");
        println!("  -> {} ({} bytes)", output_path.display(), metadata.len());
    }

    println!("Exported {} PNG files", sample_files.len());
}

#[test]
fn test_svg_export_with_custom_options() {
    let sample_files = get_sample_cdx_files();

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
