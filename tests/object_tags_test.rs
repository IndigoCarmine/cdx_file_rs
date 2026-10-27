/// Object tags follow ChemDraw's CDXConstants.h: the objects in the sample files
/// must be read as the typed objects they are.
use cdx_file_rs::cdx::file::{CdxFile, NodePayload};
use std::collections::BTreeMap;

fn count_objects(name: &str) -> BTreeMap<&'static str, usize> {
    let path = format!("{}/sample_cdx/{name}", env!("CARGO_MANIFEST_DIR"));
    let data = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let file = CdxFile::from_bytes(&data).unwrap_or_else(|e| panic!("{path}: {e}"));

    let mut counts = BTreeMap::new();
    let mut queue = vec![file.tree.root()];
    while let Some(node) = queue.pop() {
        let kind = match &*node.borrow_data() {
            NodePayload::Arrow(_) => "Arrow",
            NodePayload::Annotation(_) => "Annotation",
            NodePayload::Border(_) => "Border",
            NodePayload::ChemicalProperty(_) => "ChemicalProperty",
            NodePayload::Geometry(_) => "Geometry",
            NodePayload::SegComponent(_) => "SegComponent",
            NodePayload::SegDatum(_) => "SegDatum",
            NodePayload::StoichiometryGrid(_) => "StoichiometryGrid",
            NodePayload::TLCPlate(_) => "TLCPlate",
            NodePayload::TlcLane(_) => "TlcLane",
            NodePayload::TLCSpot(_) => "TLCSpot",
            _ => "other",
        };
        *counts.entry(kind).or_default() += 1;
        queue.extend(node.children());
    }
    counts
}

#[test]
fn reaction_has_an_arrow_and_chemical_properties() {
    let c = count_objects("Reaction.cdx");
    assert_eq!(c.get("Arrow"), Some(&1), "{c:?}");
    assert_eq!(c.get("ChemicalProperty"), Some(&2), "{c:?}");
    assert_eq!(c.get("Annotation"), Some(&2), "{c:?}");
    assert_eq!(c.get("Geometry"), None, "{c:?}");
    assert_eq!(c.get("Border"), None, "{c:?}");
}

#[test]
fn reaction_analysis_has_a_stoichiometry_grid() {
    let c = count_objects("ReactionAnalysis.cdx");
    assert_eq!(c.get("Arrow"), Some(&1), "{c:?}");
    assert_eq!(c.get("StoichiometryGrid"), Some(&1), "{c:?}");
    assert_eq!(c.get("SegComponent"), Some(&4), "{c:?}");
    assert_eq!(c.get("SegDatum"), Some(&48), "{c:?}");
}

#[test]
fn tlc_has_plate_lanes_and_spots() {
    let c = count_objects("tlc.cdx");
    assert_eq!(c.get("TLCPlate"), Some(&1), "{c:?}");
    assert_eq!(c.get("TlcLane"), Some(&2), "{c:?}");
    assert_eq!(c.get("TLCSpot"), Some(&2), "{c:?}");
}
