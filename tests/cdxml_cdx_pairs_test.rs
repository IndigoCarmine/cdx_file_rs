/// sample_cdxml/X.cdxml and sample_cdx/X.cdx are the same ChemDraw document saved in both
/// formats. Reading the CDXML must give what the binary file has: the same objects, ids and
/// property values, up to the rounding of CDXML's decimals.
///
/// Both trees are compared as CDXML written by this crate, so a value compares by what it
/// means (a boolean ChemDraw wrote with no bytes is `yes`, an INT8 and INT16 2 are both `2`).
use cdx_file_rs::cdx_parse_impl::raw_nodes::RawCdxObject;
use cdx_file_rs::cdx_parse_impl::reader::RawCdxParser;
use cdx_file_rs::cdxml::{parse_cdxml_with_warnings, write_cdxml};
use quick_xml::events::Event;
use std::collections::BTreeMap;
use std::io::Cursor;

/// Pairs saved from the same state. Analysis, Reaction and ReactionAnalysis are not: their
/// CDXML and CDX differ in content (another formula, renumbered objects).
const PAIRS: &[&str] = &["benzene", "yellow_colored"];

/// What ChemDraw recomputes or writes differently on each save, not what the file says.
const IGNORED: &[&str] = &[
    // The file name, the window and the zoom.
    "Name",
    "WindowPosition",
    "WindowSize",
    "Magnification",
    // Recomputed from the drawing.
    "BoundingBox",
    // Written to CDX only when set by hand, to CDXML always.
    "DoublePosition",
];

/// CDXML writes two decimals.
const TOLERANCE: f64 = 0.011;

fn read_pair(name: &str) -> (RawCdxObject, RawCdxObject, Vec<String>) {
    let dir = env!("CARGO_MANIFEST_DIR");
    let bytes = std::fs::read(format!("{dir}/sample_cdx/{name}.cdx")).unwrap();
    let cdx = RawCdxParser::new(Cursor::new(bytes)).parse().unwrap();
    let xml = std::fs::read_to_string(format!("{dir}/sample_cdxml/{name}.cdxml")).unwrap();
    let read = parse_cdxml_with_warnings(&xml).unwrap();
    (cdx, read.document, read.warnings)
}

type Elements = BTreeMap<String, BTreeMap<String, String>>;

/// Every element of a CDXML document by its name and id (style runs by their text's object and
/// place), with its attributes; a run's text is its `#text` attribute.
fn elements(xml: &str) -> Elements {
    let mut out = Elements::new();
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut parents: Vec<String> = Vec::new();
    let mut runs = 0;
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) | Event::Empty(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let attrs: BTreeMap<String, String> = e
                    .attributes()
                    .map(|a| {
                        let a = a.unwrap();
                        (String::from_utf8_lossy(a.key.as_ref()).into_owned(), a.unescape_value().unwrap().into_owned())
                    })
                    .collect();
                let key = match (name.as_str(), attrs.get("id")) {
                    ("s", _) => {
                        runs += 1;
                        format!("{} s{runs}", parents.last().cloned().unwrap_or_default())
                    }
                    (_, Some(id)) => {
                        runs = 0;
                        format!("<{name} id={id}>")
                    }
                    _ => format!("<{name}>"),
                };
                out.insert(key.clone(), attrs);
                parents.push(key);
            }
            Event::Text(t) => {
                if let Some(key) = parents.last() {
                    let text = t.xml_content().unwrap();
                    if key.contains(" s") {
                        out.get_mut(key).unwrap().entry("#text".into()).or_default().push_str(&text);
                    }
                }
            }
            Event::End(_) => {
                parents.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    out
}

fn same_value(a: &str, b: &str) -> bool {
    let nums = |s: &str| s.split_whitespace().map(|t| t.parse::<f64>().ok()).collect::<Option<Vec<_>>>();
    match (nums(a), nums(b)) {
        (Some(x), Some(y)) if !x.is_empty() => x.len() == y.len() && x.iter().zip(&y).all(|(p, q)| (p - q).abs() <= TOLERANCE),
        _ => a == b,
    }
}

fn differences(cdx: &Elements, cdxml: &Elements) -> Vec<String> {
    let mut out = Vec::new();
    for (key, a) in cdx {
        let Some(b) = cdxml.get(key) else {
            out.push(format!("{key} only in CDX"));
            continue;
        };
        for (name, x) in a.iter().filter(|(n, _)| !IGNORED.contains(&n.as_str())) {
            match b.get(name) {
                Some(y) if same_value(x, y) => {}
                Some(y) => out.push(format!("{key} {name}: CDX {x:?}, CDXML {y:?}")),
                // CDXML leaves out what is off.
                None if x == "no" => {}
                None => out.push(format!("{key} {name}={x:?} only in CDX")),
            }
        }
        for (name, y) in b.iter().filter(|(n, _)| !IGNORED.contains(&n.as_str()) && !a.contains_key(*n)) {
            out.push(format!("{key} {name}={y:?} only in CDXML"));
        }
    }
    for key in cdxml.keys().filter(|k| !cdx.contains_key(*k)) {
        out.push(format!("{key} only in CDXML"));
    }
    out
}

#[test]
fn cdxml_reads_into_what_chemdraw_writes_as_cdx() {
    let mut all = Vec::new();
    for name in PAIRS {
        let (cdx, cdxml, warnings) = read_pair(name);
        // A newer ChemDraw's document setting that its own sources do not name.
        for w in warnings.iter().filter(|w| !w.contains("ShowPrimeLabels")) {
            all.push(format!("{name}: warning: {w}"));
        }
        for d in differences(&elements(&write_cdxml(&cdx)), &elements(&write_cdxml(&cdxml))) {
            all.push(format!("{name}: {d}"));
        }
    }
    for d in &all {
        println!("{d}");
    }
    assert!(all.is_empty(), "{} differences (see output)", all.len());
}
