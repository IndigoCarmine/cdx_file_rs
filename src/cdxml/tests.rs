use super::*;
use crate::cdx_parse_impl::reader::RawCdxParser;

fn sample(name: &str) -> String {
    std::fs::read_to_string(format!("{}/sample_cdxml/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

fn find<'a>(o: &'a RawCdxObject, id: u32) -> Option<&'a RawCdxObject> {
    if o.id == id {
        return Some(o);
    }
    o.children.iter().find_map(|c| find(c, id))
}

fn count(o: &RawCdxObject, tag: u16) -> usize {
    o.children.iter().map(|c| count(c, tag)).sum::<usize>() + usize::from(o.tag == tag)
}

fn ids(v: &[u8]) -> Vec<u32> {
    v.chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect()
}

fn i32s(v: &[u8]) -> Vec<i32> {
    v.chunks_exact(4).map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect()
}

/// The same tree: tags, ids, properties (in order) and children.
fn assert_same(a: &RawCdxObject, b: &RawCdxObject) {
    assert_eq!((a.tag, a.id), (b.tag, b.id));
    let props = |o: &RawCdxObject| o.properties.iter().map(|p| (p.tag, p.value.clone())).collect::<Vec<_>>();
    assert_eq!(props(a), props(b), "properties of 0x{:04X} id {}", a.tag, a.id);
    assert_eq!(a.children.len(), b.children.len(), "children of 0x{:04X} id {}", a.tag, a.id);
    for (x, y) in a.children.iter().zip(&b.children) {
        assert_same(x, y);
    }
}

#[test]
fn a_multi_step_scheme_reads_its_steps_arrows_and_labels() {
    let read = parse_cdxml_with_warnings(&sample("scheme_two_rows.cdxml")).unwrap();
    let doc = &read.document;
    assert_eq!(doc.tag, 0x8000);
    let page = &doc.children[0];
    assert_eq!(page.tag, 0x8001);
    let top = |tag| page.children.iter().filter(|c| c.tag == tag).count();
    assert_eq!((top(0x8003), top(0x8006), top(0x8007), top(0x8021), top(0x800D), top(0x8020)), (4, 7, 2, 1, 1, 4));
    assert_eq!(count(doc, 0x800E), 1);

    // The step: (I) to (II) over the arrow drawn as graphic 30, "i)" above it.
    let step = find(doc, 61).unwrap();
    assert_eq!(ids(step.get_property(0x0C01).unwrap()), [10]);
    assert_eq!(ids(step.get_property(0x0C02).unwrap()), [40]);
    assert_eq!(ids(step.get_property(0x0C04).unwrap()), [30]);
    assert_eq!(ids(step.get_property(0x0C05).unwrap()), [32]);
    // The graphic is superseded by the arrow object.
    let graphic = find(doc, 30).unwrap();
    assert_eq!(ids(graphic.get_property(0x0013).unwrap()), [31]);
    assert_eq!(graphic.get_property(0x0A00).unwrap(), &1i16.to_le_bytes().to_vec(), "Line");
    assert_eq!(graphic.get_property(0x0A02).unwrap(), &2i16.to_le_bytes().to_vec(), "FullHead");
    // Head3D="220 100 0", in 1/65536 point, x y z.
    let arrow = find(doc, 31).unwrap();
    assert_eq!(i32s(arrow.get_property(0x0207).unwrap()), [220 * 65536, 100 * 65536, 0]);

    // The compound label "(I)" is a text the chemical property displays.
    let property = find(doc, 21).unwrap();
    assert_eq!(ids(property.get_property(0x0BB1).unwrap()), [20]);
    assert_eq!(ids(property.get_property(0x0B82).unwrap()), [11, 12, 13]);
    let (runs, text) = split_cdx_string(find(doc, 20).unwrap().get_property(PROP_TEXT).unwrap());
    assert_eq!(text, b"(I)");
    assert_eq!((runs.len(), runs[0].1.font, runs[0].1.size, runs[0].1.face), (1, 3, 200, 0));
    // "4" is bold.
    let (runs, text) = split_cdx_string(find(doc, 110).unwrap().get_property(PROP_TEXT).unwrap());
    assert_eq!((text.as_slice(), runs[0].1.face), (b"4".as_slice(), 1));
}

#[test]
fn nodes_keep_position_type_and_label() {
    let doc = parse_cdxml(&sample("scheme_two_rows.cdxml")).unwrap();
    // <n id="42" p="264.40 100" NodeType="Nickname"> with the label TEG.
    let n = find(&doc, 42).unwrap();
    let c = |v: f64| (v * 65536.0).round() as i32;
    assert_eq!(i32s(n.get_property(0x0200).unwrap()), [c(100.0), c(264.4)], "y then x");
    assert_eq!(n.get_property(0x0400).unwrap(), &4i16.to_le_bytes().to_vec());
    let label = n.children.iter().find(|c| c.tag == 0x8006).unwrap();
    assert_eq!(split_cdx_string(label.get_property(PROP_TEXT).unwrap()).1, b"TEG");
    // The nickname's own fragment is read too, its connection point a node type of its own.
    assert!(n.children.iter().any(|c| c.tag == 0x8003));
    assert_eq!(find(&doc, 46).unwrap().get_property(0x0400).unwrap(), &12i16.to_le_bytes().to_vec());
}

#[test]
fn document_tables_are_properties() {
    let doc = parse_cdxml(&sample("scheme_two_rows.cdxml")).unwrap();
    let fonts = doc.get_property(PROP_FONT_TABLE).unwrap();
    assert_eq!(u16::from_le_bytes([fonts[2], fonts[3]]), 1, "one font");
    assert_eq!(u16::from_le_bytes([fonts[4], fonts[5]]), 3, "font id 3");
    assert_eq!(u16::from_le_bytes([fonts[6], fonts[7]]), 1252, "iso-8859-1");
    let colors = doc.get_property(PROP_COLOR_TABLE).unwrap();
    assert_eq!(u16::from_le_bytes([colors[0], colors[1]]), 2);
}

#[test]
fn bond_orders_are_bits() {
    let xml = r#"<CDXML><page id="1"><fragment id="2"><n id="3"/><n id="4"/>
        <b id="5" B="3" E="4" Order="1.5"/><b id="6" B="3" E="4" Order="2"/><b id="7" B="3" E="4"/></fragment></page></CDXML>"#;
    let doc = parse_cdxml(xml).unwrap();
    assert_eq!(find(&doc, 5).unwrap().get_property(0x0600).unwrap(), &0x0080u16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 6).unwrap().get_property(0x0600).unwrap(), &0x0002u16.to_le_bytes().to_vec());
    assert!(find(&doc, 7).unwrap().get_property(0x0600).is_none(), "absent is single");
    assert!(write_cdxml(&doc).contains("Order=\"1.5\""));
}

#[test]
fn what_is_not_known_is_a_warning() {
    let xml = r#"<CDXML Mystery="1"><page id="1"><gizmo id="9"><n id="10"/></gizmo><t id="2" p="1 2" LineHeight="auto"><s font="3" size="10" face="1">A&amp;B&#946;</s></t></page></CDXML>"#;
    let read = parse_cdxml_with_warnings(xml).unwrap();
    assert!(read.warnings.iter().any(|w| w.contains("Mystery")));
    assert!(read.warnings.iter().any(|w| w.contains("gizmo")));
    assert!(!read.warnings.iter().any(|w| w.contains("LineHeight")), "{:?}", read.warnings);
    assert!(find(&read.document, 10).is_none(), "an unknown element's children are left out with it");
    // LineHeight="auto" is the automatic line height, 1.
    assert_eq!(find(&read.document, 2).unwrap().get_property(0x0702), Some(&vec![1, 0]));
    let (_, text) = split_cdx_string(find(&read.document, 2).unwrap().get_property(PROP_TEXT).unwrap());
    assert_eq!(text, b"A&B?", "entities resolved; what Latin-1 cannot hold is ?");
}

#[test]
fn objects_without_ids_get_unique_ones() {
    let doc = parse_cdxml(r#"<CDXML><page><fragment id="7"><n/><n/></fragment></page></CDXML>"#).unwrap();
    let mut all = Vec::new();
    fn collect(o: &RawCdxObject, all: &mut Vec<u32>) {
        all.push(o.id);
        o.children.iter().for_each(|c| collect(c, all));
    }
    doc.children.iter().for_each(|c| collect(c, &mut all));
    let mut unique = all.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), all.len());
    assert!(!all.contains(&0));
}

#[test]
fn written_cdxml_reads_back_to_the_same_tree() {
    for name in ["scheme_two_rows.cdxml", "Reaction.cdxml", "ReactionAnalysis.cdxml", "Analysis.cdxml", "benzene.cdxml", "yellow_colored.cdxml"] {
        let first = parse_cdxml(&sample(name)).unwrap_or_else(|e| panic!("{name}: {e}"));
        let written = write_cdxml(&first);
        let second = parse_cdxml(&written).unwrap_or_else(|e| panic!("{name} written: {e}\n{written}"));
        assert_same(&first, &second);
    }
}

#[test]
fn binary_cdx_writes_as_cdxml_and_reads_back() {
    for name in ["Reaction", "benzene"] {
        let bytes = std::fs::read(format!("{}/sample_cdx/{name}.cdx", env!("CARGO_MANIFEST_DIR"))).unwrap();
        let binary = RawCdxParser::new(std::io::Cursor::new(bytes)).parse().unwrap();
        let (xml, _) = write_cdxml_with_warnings(&binary);
        let read = parse_cdxml(&xml).unwrap();
        // Every node keeps its position, every bond its ends.
        for tag in [0x8004u16, 0x8005, 0x8003, 0x800E] {
            assert_eq!(count(&binary, tag), count(&read, tag), "{name}: objects 0x{tag:04X}");
        }
        fn each<'a>(o: &'a RawCdxObject, f: &mut impl FnMut(&'a RawCdxObject)) {
            f(o);
            o.children.iter().for_each(|c| each(c, f));
        }
        each(&binary, &mut |o| {
            if o.tag == 0x8004 || o.tag == 0x8005 {
                let r = find(&read, o.id).unwrap();
                for tag in [0x0200u16, 0x0402, 0x0604, 0x0605, 0x0600] {
                    assert_eq!(o.get_property(tag), r.get_property(tag), "{name}: id {} prop 0x{tag:04X}", o.id);
                }
            }
        });
    }
}

#[test]
fn numbers_are_written_short() {
    assert_eq!(fmt(1.50, 2), "1.5");
    assert_eq!(fmt(-0.001, 2), "0");
    assert_eq!(fmt(14.4, 2), "14.4");
    assert_eq!(fmt(3.0, 2), "3");
}

#[test]
fn text_latin1_cannot_hold_keeps_its_utf8() {
    let xml = r#"<CDXML><page id="1"><t id="2" p="1 2"><s font="3" size="10" face="0">Δ, 80 °C</s><s font="3" size="10" face="32">2</s></t></page></CDXML>"#;
    let doc = parse_cdxml(xml).unwrap();
    let t = find(&doc, 2).unwrap();
    assert_eq!(split_cdx_string(t.get_property(PROP_TEXT).unwrap()).1, b"?, 80 \xb0C2");
    assert_eq!(String::from_utf8(split_cdx_string(t.get_property(PROP_UTF8_TEXT).unwrap()).1).unwrap(), "Δ, 80 °C2");
    let written = write_cdxml(&doc);
    assert!(written.contains(">Δ, 80 °C</s>"), "{written}");
    assert!(written.contains("face=\"32\" color=\"0\">2</s>"), "{written}");
    assert_same(&doc, &parse_cdxml(&written).unwrap());
}

#[test]
fn flags_are_sets_of_names() {
    let xml = r#"<CDXML><page id="1"><fragment id="2"><n id="3"/><n id="4"/>
        <b id="5" B="3" E="4" Order="1 2"/><b id="6" B="3" E="4" Order="any"/><b id="7" B="3" E="4" Order="threecenter"/></fragment>
        <graphic id="8" GraphicType="Oval" OvalType="Circle Filled"/></page></CDXML>"#;
    let read = parse_cdxml_with_warnings(xml).unwrap();
    assert!(read.warnings.is_empty(), "{:?}", read.warnings);
    let doc = read.document;
    assert_eq!(find(&doc, 5).unwrap().get_property(0x0600).unwrap(), &0x0003u16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 6).unwrap().get_property(0x0600).unwrap(), &0xFFFFu16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 8).unwrap().get_property(0x0A04).unwrap(), &(1u16 | 4).to_le_bytes().to_vec());
    let written = write_cdxml(&doc);
    for s in ["Order=\"1 2\"", "Order=\"any\"", "Order=\"threecenter\"", "OvalType=\"Circle Filled\""] {
        assert!(written.contains(s), "{s} in {written}");
    }
}

/// Read, check there were no warnings, write and read back to the same tree.
fn read_clean(xml: &str) -> RawCdxObject {
    let read = parse_cdxml_with_warnings(xml).unwrap();
    assert!(read.warnings.is_empty(), "{:?}", read.warnings);
    let written = write_cdxml(&read.document);
    assert_same(&read.document, &parse_cdxml(&written).unwrap());
    read.document
}

#[test]
fn element_and_generic_lists_count_negative_for_not() {
    let doc = read_clean(r#"<CDXML><page id="1"><fragment id="2"><n id="3" NodeType="ElementList" ElementList="NOT 6 7 8"/><n id="4" NodeType="GenericNickname" GenericList="R X"/></fragment></page></CDXML>"#);
    let i16s = |v: &[u8]| v.chunks(2).map(|b| i16::from_le_bytes([b[0], b[1]])).collect::<Vec<_>>();
    assert_eq!(i16s(find(&doc, 3).unwrap().get_property(0x0403).unwrap()), [-3, 6, 7, 8]);
    // Count, then each name with its length (+2) and no style runs.
    assert_eq!(find(&doc, 4).unwrap().get_property(0x0441).unwrap(), &[2, 0, 3, 0, 0, 0, b'R', 3, 0, 0, 0, b'X']);
}

#[test]
fn represent_elements_are_the_represents_property() {
    let doc = read_clean(r#"<CDXML><page id="1"><fragment id="2"><n id="3" Charge="1"><t id="4"><s font="3" size="10" face="0">+</s><represent attribute="Charge" object="3"/></t></n></fragment></page></CDXML>"#);
    // The object's id, then the property's tag (Charge, 0x0421).
    assert_eq!(find(&doc, 4).unwrap().get_property(0x000E).unwrap(), &[3, 0, 0, 0, 0x21, 0x04]);
}

#[test]
fn label_and_caption_attributes_are_one_style_each() {
    let doc = read_clean(r#"<CDXML LabelFont="3" LabelSize="10" LabelFace="96" CaptionFont="4" CaptionSize="12.5"/>"#);
    let u16s = |v: &[u8]| v.chunks(2).map(|b| u16::from_le_bytes([b[0], b[1]])).collect::<Vec<_>>();
    // font, face, size in 1/20 point, color (the foreground, 3, when not given).
    assert_eq!(u16s(doc.get_property(0x080A).unwrap()), [3, 96, 200, 3]);
    assert_eq!(u16s(doc.get_property(0x080B).unwrap()), [4, 0, 250, 3]);
    assert!(doc.get_property(0x081A).is_none(), "no separate LabelFont property");
}

#[test]
fn attributes_in_other_units_are_converted() {
    let doc = read_clean(r#"<CDXML ChainAngle="120" BondSpacing="18"><page id="1"><t id="2" LineHeight="auto"/><t id="3" LineHeight="12"/><fragment id="4"><n id="5" Charge="-2"/><n id="6" Charge="0.5"/></fragment></page></CDXML>"#);
    assert_eq!(doc.get_property(0x0803).unwrap(), &(120 * 65536i32).to_le_bytes().to_vec());
    assert_eq!(doc.get_property(0x0804).unwrap(), &180i16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 2).unwrap().get_property(0x0702).unwrap(), &1u16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 3).unwrap().get_property(0x0702).unwrap(), &240u16.to_le_bytes().to_vec());
    assert_eq!(find(&doc, 5).unwrap().get_property(0x0421).unwrap(), &vec![0xFE]);
    assert_eq!(find(&doc, 6).unwrap().get_property(0x0421).unwrap(), &(1i32 << 23).to_le_bytes().to_vec());
    let written = write_cdxml(&doc);
    for s in ["ChainAngle=\"120\"", "BondSpacing=\"18\"", "LineHeight=\"auto\"", "LineHeight=\"12\"", "Charge=\"-2\"", "Charge=\"0.5\""] {
        assert!(written.contains(s), "{s} in {written}");
    }
}

#[test]
fn values_typed_by_another_attribute() {
    let doc = read_clean(r#"<CDXML><page id="1"><objecttag id="2" TagType="Double" Name="mass" Value="12.5"/><objecttag id="3" TagType="String" Name="label" Value="abc"/>
        <stoichiometrygrid id="4"><sgcomponent id="5"><sgdatum id="6" SGDataType="4" SGDataValue="MW"/><sgdatum id="7" SGDataType="3" SGDataValue="216.5"/></sgcomponent></stoichiometrygrid></page></CDXML>"#);
    assert_eq!(find(&doc, 2).unwrap().get_property(0x0D05).unwrap(), &12.5f64.to_le_bytes().to_vec());
    // An object tag's string Value is a CDXString; its Name plain text.
    assert_eq!(find(&doc, 3).unwrap().get_property(0x0D05).unwrap(), &[0, 0, b'a', b'b', b'c']);
    assert_eq!(find(&doc, 3).unwrap().get_property(0x0008).unwrap(), b"label");
    assert_eq!(find(&doc, 6).unwrap().get_property(0x1202).unwrap(), b"MW");
    assert_eq!(find(&doc, 7).unwrap().get_property(0x1202).unwrap(), &216.5f64.to_le_bytes().to_vec());
}

#[test]
fn binary_data_is_hex_or_base64() {
    let doc = read_clean(r#"<CDXML MacPrintInfo="00FF10"><page id="1"><embeddedobject id="2" CompressedOLEObject="AAEC/w=="/></page></CDXML>"#);
    assert_eq!(doc.get_property(0x0800).unwrap(), &[0x00, 0xFF, 0x10]);
    let tag = table::PROPERTIES.iter().find(|p| p.0 == "CompressedOLEObject").unwrap().1;
    assert_eq!(find(&doc, 2).unwrap().get_property(tag).unwrap(), &[0, 1, 2, 0xFF]);
}

#[test]
fn an_id_that_is_not_a_number_is_a_warning() {
    let read = parse_cdxml_with_warnings(r#"<CDXML><page id="p1"/></CDXML>"#).unwrap();
    assert!(read.warnings.iter().any(|w| w.contains("id=\"p1\"")), "{:?}", read.warnings);
    assert_ne!(read.document.children[0].id, 0);
}
