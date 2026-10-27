/// CDXML reading tests against RDKit's CDXML test corpus.
///
/// The corpus is not committed; fetch it first with
/// `scripts/fetch_rdkit_testdata.sh` (or `.ps1`). It lands in tests/data/rdkit/.
/// Without it, only the repository's own sample_cdxml/ files are used.
///
/// Run the parsing test with:
///   cargo test --test rdkit_cdxml_test -- --ignored --nocapture

use cdx_file_rs::cdx::file::CdxFile;
use std::fs;
use std::path::{Path, PathBuf};

/// Files that are not well-formed XML; reading them must fail.
const MUST_FAIL: &[&str] = &["malformed.cdxml"];

/// Well-formed XML with invalid attribute values (non-numeric id, coordinates,
/// bond order). RDKit reads them but drops the molecules, so a reader may
/// either reject them or recover.
const MAY_FAIL: &[&str] = &[
    "bad-id.cdxml",
    "bad-coords.cdxml",
    "bad-bondorder.cdxml",
    "bad-bondorder2.cdxml",
];

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rdkit_data_dir() -> PathBuf {
    manifest_dir().join("tests").join("data").join("rdkit")
}

fn collect_cdxml(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|s| s.to_str()) != Some(".git") {
                collect_cdxml(&path, out);
            }
        } else if path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("cdxml"))
        {
            out.push(path);
        }
    }
}

/// RDKit corpus files, or None (with a hint) when the corpus has not been fetched.
fn rdkit_cdxml_files() -> Option<Vec<PathBuf>> {
    let dir = rdkit_data_dir();
    if !dir.exists() {
        eprintln!(
            "RDKit test data not found at {}; run scripts/fetch_rdkit_testdata.sh (or .ps1). Skipping.",
            dir.display()
        );
        return None;
    }
    let mut files = Vec::new();
    collect_cdxml(&dir, &mut files);
    files.sort();
    Some(files)
}

fn sample_cdxml_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_cdxml(&manifest_dir().join("sample_cdxml"), &mut files);
    files.sort();
    files
}

fn file_name(path: &Path) -> &str {
    path.file_name().and_then(|s| s.to_str()).unwrap_or("")
}

#[test]
fn rdkit_testdata_layout() {
    let Some(files) = rdkit_cdxml_files() else {
        return;
    };
    assert!(!files.is_empty(), "no .cdxml files under {}", rdkit_data_dir().display());
    for name in MUST_FAIL.iter().chain(MAY_FAIL) {
        assert!(
            files.iter().any(|p| file_name(p) == *name),
            "expected fixture {name} missing from RDKit test data"
        );
    }
    for path in &files {
        let bytes = fs::read(path).expect("failed to read file");
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("<CDXML"), "{} has no <CDXML> element", path.display());
    }
    println!("RDKit CDXML files: {}", files.len());
}

#[test]
#[ignore = "CDXML reader not implemented yet"]
fn parse_all_rdkit_cdxml() {
    let mut files = sample_cdxml_files();
    files.extend(rdkit_cdxml_files().unwrap_or_default());
    assert!(!files.is_empty(), "no CDXML files to test");

    let mut ok = 0;
    let mut tolerated = Vec::new();
    let mut unexpected_failures = Vec::new();
    let mut unexpected_successes = Vec::new();

    for path in &files {
        let name = file_name(path);
        let result = fs::read(path)
            .map_err(|e| e.to_string())
            .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
            .and_then(|xml| CdxFile::from_cdxml_str(&xml).map_err(|e| e.to_string()));

        match result {
            Ok(_) if MUST_FAIL.contains(&name) => unexpected_successes.push(path.display().to_string()),
            Ok(_) => ok += 1,
            Err(_) if MUST_FAIL.contains(&name) => ok += 1,
            Err(e) if MAY_FAIL.contains(&name) => tolerated.push(format!("{}: {}", path.display(), e)),
            Err(e) => unexpected_failures.push(format!("{}: {}", path.display(), e)),
        }
    }

    println!("CDXML files: {}, as expected: {}", files.len(), ok);
    for t in &tolerated {
        println!("  tolerated failure: {t}");
    }
    for f in &unexpected_failures {
        println!("  FAILED: {f}");
    }
    for s in &unexpected_successes {
        println!("  UNEXPECTEDLY PARSED: {s}");
    }

    assert!(
        unexpected_failures.is_empty() && unexpected_successes.is_empty(),
        "{} unexpected failures, {} unexpected successes (see output above)",
        unexpected_failures.len(),
        unexpected_successes.len()
    );
}
