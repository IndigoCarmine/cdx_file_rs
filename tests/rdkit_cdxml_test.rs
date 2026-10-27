/// CDXML reading tests against RDKit's CDXML test corpus.
///
/// The corpus is not committed; fetch it first with
/// `scripts/fetch_rdkit_testdata.sh` (or `.ps1`). It lands in tests/data/rdkit/.
/// Without it, only the repository's own sample_cdxml/ files are used.
///
/// See what was read and left out with:
///   cargo test --test rdkit_cdxml_test -- --nocapture

use cdx_file_rs::cdx::file::CdxFile;
use cdx_file_rs::cdx_parse_impl::raw_nodes::RawCdxObject;
use cdx_file_rs::cdxml::{parse_cdxml, parse_cdxml_with_warnings, write_cdxml};
use std::collections::BTreeMap;
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

/// The first difference between two raw trees, if any.
fn first_difference(a: &RawCdxObject, b: &RawCdxObject) -> Option<String> {
    let props = |o: &RawCdxObject| o.properties.iter().map(|p| (p.tag, p.value.clone())).collect::<BTreeMap<_, _>>();
    let at = format!("object 0x{:04X} id {}", a.tag, a.id);
    if (a.tag, a.id) != (b.tag, b.id) || a.children.len() != b.children.len() {
        return Some(at);
    }
    let (pa, pb) = (props(a), props(b));
    if let Some(tag) = pa.keys().chain(pb.keys()).find(|t| pa.get(*t) != pb.get(*t)) {
        return Some(format!("{at} property 0x{tag:04X}: {:02X?} then {:02X?}", pa.get(tag), pb.get(tag)));
    }
    a.children.iter().zip(&b.children).find_map(|(x, y)| first_difference(x, y))
}

/// A warning without the ids and values in it, to count warnings of one kind together.
fn warning_kind(w: &str) -> String {
    let mut out = String::new();
    let mut in_quotes = false;
    for c in w.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                out.push(c);
            }
            _ if in_quotes => {}
            c if c.is_ascii_digit() => {}
            c => out.push(c),
        }
    }
    out
}

/// Warnings expected on the corpus: what ChemDraw's own sources cannot read either.
const EXPECTED_WARNINGS: &[&str] = &[
    // MAY_FAIL fixtures: invalid ids, coordinates and bond orders.
    "id=\"\" is not a number",
    "p=\"\": foofoo is not a number",
    "Order=\"\": unknown flag slartibartfast",
    // Obsolete document settings (kCDXProp_Unused10/11 in CDXConstants.h).
    "attribute ResidueZigZag not known",
    "attribute NumberResidueBlocks not known",
    // A newer ChemDraw's setting the sources do not name.
    "attribute ShowPrimeLabels not known",
    // ChemDraw 6.0 wrote LineHeight="yes" on some text.
    "<t> LineHeight=\"\": not a number",
];

#[test]
fn parse_all_rdkit_cdxml() {
    let mut files = sample_cdxml_files();
    files.extend(rdkit_cdxml_files().unwrap_or_default());
    assert!(!files.is_empty(), "no CDXML files to test");

    let mut ok = 0;
    let mut tolerated = Vec::new();
    let mut unexpected_failures = Vec::new();
    let mut unexpected_successes = Vec::new();
    let mut warnings: BTreeMap<String, (usize, String)> = BTreeMap::new();

    for path in &files {
        let name = file_name(path);
        let xml = match fs::read(path).map_err(|e| e.to_string()).and_then(|b| String::from_utf8(b).map_err(|e| e.to_string())) {
            Ok(xml) => xml,
            Err(e) => {
                unexpected_failures.push(format!("{}: {e}", path.display()));
                continue;
            }
        };
        let result = CdxFile::from_cdxml_str(&xml).map_err(|e| e.to_string()).and_then(|_| {
            let read = parse_cdxml_with_warnings(&xml).map_err(|e| e.to_string())?;
            for w in &read.warnings {
                let entry = warnings.entry(warning_kind(w)).or_insert((0, format!("{name}: {w}")));
                entry.0 += 1;
            }
            // What is read writes back as CDXML that reads to the same tree.
            let again = parse_cdxml(&write_cdxml(&read.document)).map_err(|e| format!("written CDXML: {e}"))?;
            match first_difference(&read.document, &again) {
                Some(d) => Err(format!("written and read back, {d} differs")),
                None => Ok(()),
            }
        });

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
    let unexpected_warnings: Vec<_> =
        warnings.iter().filter(|(kind, _)| !EXPECTED_WARNINGS.iter().any(|e| kind.contains(e))).collect();
    for (kind, (n, example)) in &warnings {
        println!("  warning x{n}: {kind}
      e.g. {example}");
    }

    assert!(
        unexpected_failures.is_empty() && unexpected_successes.is_empty() && unexpected_warnings.is_empty(),
        "{} unexpected failures, {} unexpected successes, {} unexpected kinds of warning (see output above)",
        unexpected_failures.len(),
        unexpected_successes.len(),
        unexpected_warnings.len()
    );
}
