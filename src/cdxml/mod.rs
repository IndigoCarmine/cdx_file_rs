//! CDXML, the XML form of CDX, read into and written from the same [`RawCdxObject`] tree the
//! binary reader produces, so every typed object (`Node`, `ReactionStep`, …) and every consumer
//! of the raw tree works on either format.
//!
//! An element is an object (`<n>` is a Node, `<step>` a ReactionStep); an attribute is a
//! property, its text converted to the property's binary value as the CDX specification lays it
//! out. The element and attribute names, tags and value types come from the specification pages
//! in `doc/md`, gathered into [`table`] by `tools/gen_cdxml_table.py`. A few properties are not
//! plain attributes and are handled here:
//!
//! - the document's `<colortable>` and `<fonttable>` children are its ColorTable (0x0300) and
//!   FontTable (0x0100) properties;
//! - a `<t>`'s `<s>` children are its Text property (0x0700): one style run per `<s>`. Text
//!   is Latin-1 there; text with characters Latin-1 has not (Δ, β) also gets the UTF-8 Text
//!   property (0x0709), which writing prefers;
//! - a bond's `Order` is a list of names (`1`, `1.5`, `dative`, …) for a bit mask.
//!
//! What has no entry in the table (a newer ChemDraw's attributes, an object this crate does not
//! know) is left out and reported in the warnings, not an error: what can be read is read.

mod table;

use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

/// How a property's value is laid out in binary CDX.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    F64,
    /// One byte, `yes`/`no`.
    Bool,
    /// Present (no bytes) when `yes`, absent otherwise.
    BoolImplied,
    /// INT32 in 1/65536 point; CDXML writes points.
    Coordinate,
    /// y then x; CDXML writes `x y`.
    Point2D,
    /// x, y, z.
    Point3D,
    /// top, left, bottom, right; CDXML writes `left top right bottom`.
    Rectangle,
    ObjectId,
    ObjectIdArray,
    /// A UINT16 count, then the ids.
    ObjectIdArrayWithCounts,
    /// A CDXString without style runs.
    String,
    /// A UINT16 count, then that many (y, x) points.
    CurvePoints,
    /// A UINT16 count, then that many (x, y, z) points.
    CurvePoints3D,
}

const TAG_DOCUMENT: u16 = 0x8000;
const TAG_TEXT: u16 = 0x8006;
const PROP_FONT_TABLE: u16 = 0x0100;
const PROP_COLOR_TABLE: u16 = 0x0300;
const PROP_TEXT: u16 = 0x0700;
/// A Text object's text in UTF-8, with the Text property's style runs (char positions).
const PROP_UTF8_TEXT: u16 = 0x0709;
const PROP_BOND_ORDER: u16 = 0x0600;
/// One point in CDX coordinate units.
const POINT: f64 = 65536.0;

/// Bond order names and their bits (kCDXBondOrder_*).
const BOND_ORDERS: &[(&str, u16)] = &[
    ("1", 0x0001),
    ("2", 0x0002),
    ("3", 0x0004),
    ("4", 0x0008),
    ("5", 0x0010),
    ("6", 0x0020),
    ("0.5", 0x0040),
    ("1.5", 0x0080),
    ("2.5", 0x0100),
    ("3.5", 0x0200),
    ("4.5", 0x0400),
    ("5.5", 0x0800),
    ("dative", 0x1000),
    ("ionic", 0x2000),
    ("hydrogen", 0x4000),
    ("threecenter", 0x8000),
];

/// Font character sets (CDXCharSet, Windows code pages) and their CDXML names.
const CHARSETS: &[(u16, &str)] = &[
    (0, "Unknown"),
    (874, "windows-874"),
    (932, "shift_jis"),
    (936, "gb2312"),
    (949, "ks_c_5601-1987"),
    (950, "big5"),
    (1250, "iso-8859-2"),
    (1251, "iso-8859-5"),
    (1252, "iso-8859-1"),
    (1253, "iso-8859-7"),
    (1254, "iso-8859-9"),
    (1255, "iso-8859-8"),
    (1256, "iso-8859-6"),
    (1257, "iso-8859-4"),
    (1258, "windows-1258"),
    (10000, "x-mac-roman"),
    (65000, "utf-7"),
    (65001, "utf-8"),
    (42, "Symbol"),
];

/// A CDXML document read into a raw object tree, with what could not be read.
#[derive(Debug, Clone)]
pub struct CdxmlRead {
    /// The Document object (tag 0x8000).
    pub document: RawCdxObject,
    /// Elements and attributes left out because they are not known, or their value could not
    /// be read.
    pub warnings: Vec<String>,
}

/// Read a CDXML document. See [`parse_cdxml_with_warnings`] for what was left out.
pub fn parse_cdxml(xml: &str) -> Result<RawCdxObject, CdxError> {
    parse_cdxml_with_warnings(xml).map(|r| r.document)
}

/// Read a CDXML document into the raw object tree binary CDX reads into.
pub fn parse_cdxml_with_warnings(xml: &str) -> Result<CdxmlRead, CdxError> {
    let mut reader = Reader::from_str(xml);
    let mut stack: Vec<Open> = Vec::new();
    let mut warnings = Vec::new();
    let mut document: Option<RawCdxObject> = None;
    let mut text_defaults = RunDefaults::default();
    // Inside an element that is skipped: how deep.
    let mut skipping = 0usize;
    loop {
        let event = reader.read_event().map_err(|e| CdxError::DecodeError(format!("CDXML: {e}")))?;
        match event {
            Event::Start(ref e) | Event::Empty(ref e) => {
                let empty = matches!(event, Event::Empty(_));
                if skipping > 0 {
                    if !empty {
                        skipping += 1;
                    }
                    continue;
                }
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let attrs = attributes(e)?;
                let open = match (name.as_str(), stack.last_mut()) {
                    ("CDXML", None) => {
                        text_defaults = RunDefaults::from_document(&attrs);
                        Some(Open::Object(object(TAG_DOCUMENT, &name, &attrs, &mut warnings)))
                    }
                    ("colortable", Some(Open::Object(_))) => Some(Open::Colors(Vec::new())),
                    ("color", Some(Open::Colors(colors))) => {
                        let c = |k: &str| attr(&attrs, k).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                        colors.push([c("r"), c("g"), c("b")]);
                        None
                    }
                    ("fonttable", Some(Open::Object(_))) => Some(Open::Fonts(Vec::new())),
                    ("font", Some(Open::Fonts(fonts))) => {
                        let id = attr(&attrs, "id").and_then(|v| v.parse().ok()).unwrap_or(0);
                        let charset = attr(&attrs, "charset").map(charset_code).unwrap_or(0);
                        fonts.push((id, charset, attr(&attrs, "name").unwrap_or("").to_string()));
                        None
                    }
                    ("s", Some(Open::Object(t))) if t.tag == TAG_TEXT => {
                        Some(Open::Run(text_defaults.run(&attrs), String::new()))
                    }
                    (_, Some(Open::Object(_))) => match table::OBJECTS.iter().find(|(n, _)| *n == name) {
                        Some(&(_, tag)) => Some(Open::Object(object(tag, &name, &attrs, &mut warnings))),
                        None => {
                            warnings.push(format!("unknown element <{name}> left out"));
                            if !empty {
                                skipping = 1;
                            }
                            None
                        }
                    },
                    (_, None) => return Err(CdxError::DecodeError(format!("CDXML: root is <{name}>, not <CDXML>"))),
                    (_, Some(_)) => {
                        warnings.push(format!("<{name}> out of place, left out"));
                        if !empty {
                            skipping = 1;
                        }
                        None
                    }
                };
                if let Some(open) = open {
                    stack.push(open);
                    if empty {
                        close(&mut stack, &mut document);
                    }
                }
            }
            Event::End(_) => {
                if skipping > 0 {
                    skipping -= 1;
                    continue;
                }
                close(&mut stack, &mut document);
            }
            Event::Text(t) => {
                if let (0, Some(Open::Run(_, text))) = (skipping, stack.last_mut()) {
                    text.push_str(&t.xml_content().map_err(|e| CdxError::DecodeError(format!("CDXML: {e}")))?);
                }
            }
            Event::CData(t) => {
                if let (0, Some(Open::Run(_, text))) = (skipping, stack.last_mut()) {
                    text.push_str(&String::from_utf8_lossy(&t));
                }
            }
            Event::GeneralRef(r) => {
                if let (0, Some(Open::Run(_, text))) = (skipping, stack.last_mut()) {
                    match r.resolve_char_ref().ok().flatten() {
                        Some(c) => text.push(c),
                        None => match r.as_ref() {
                            b"amp" => text.push('&'),
                            b"lt" => text.push('<'),
                            b"gt" => text.push('>'),
                            b"quot" => text.push('"'),
                            b"apos" => text.push('\''),
                            other => warnings.push(format!("unknown entity &{};", String::from_utf8_lossy(other))),
                        },
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    let mut document = document.ok_or_else(|| CdxError::DecodeError("CDXML: no <CDXML> element".into()))?;
    assign_missing_ids(&mut document);
    Ok(CdxmlRead { document, warnings })
}

/// An element being read.
enum Open {
    Object(RawCdxObject),
    Colors(Vec<[f64; 3]>),
    Fonts(Vec<(u16, u16, String)>),
    /// A style run of a `<t>`, and its characters so far.
    Run(Run, String),
}

/// The style of one `<s>`.
#[derive(Clone, Copy)]
struct Run {
    font: u16,
    face: u16,
    /// In 1/20 point.
    size: u16,
    color: u16,
}

/// What an `<s>` that leaves out an attribute has: the document's caption style.
#[derive(Clone, Copy)]
struct RunDefaults(Run);

impl Default for RunDefaults {
    fn default() -> Self {
        RunDefaults(Run { font: 0, face: 0, size: 200, color: 0 })
    }
}

impl RunDefaults {
    fn from_document(attrs: &[(String, String)]) -> Self {
        let d = Self::default().0;
        let num = |k: &str| attr(attrs, k).and_then(|v| v.parse::<f64>().ok());
        RunDefaults(Run {
            font: num("CaptionFont").map_or(d.font, |v| v as u16),
            face: num("CaptionFace").map_or(d.face, |v| v as u16),
            size: num("CaptionSize").map_or(d.size, |v| (v * 20.0).round() as u16),
            color: d.color,
        })
    }

    fn run(&self, attrs: &[(String, String)]) -> Run {
        let num = |k: &str| attr(attrs, k).and_then(|v| v.parse::<f64>().ok());
        Run {
            font: num("font").map_or(self.0.font, |v| v as u16),
            face: num("face").map_or(self.0.face, |v| v as u16),
            size: num("size").map_or(self.0.size, |v| (v * 20.0).round() as u16),
            color: num("color").map_or(self.0.color, |v| v as u16),
        }
    }
}

/// Finish the element on top of the stack: hand it to its parent, or make it the document.
fn close(stack: &mut Vec<Open>, document: &mut Option<RawCdxObject>) {
    let Some(done) = stack.pop() else { return };
    match (done, stack.last_mut()) {
        (Open::Object(obj), Some(Open::Object(parent))) => parent.children.push(obj),
        (Open::Object(obj), None) => *document = Some(obj),
        (Open::Colors(colors), Some(Open::Object(parent))) => {
            let mut v = (colors.len() as u16).to_le_bytes().to_vec();
            for c in colors {
                for x in c {
                    v.extend_from_slice(&((x.clamp(0.0, 1.0) * 65535.0).round() as u16).to_le_bytes());
                }
            }
            parent.set_property(PROP_COLOR_TABLE, v);
        }
        (Open::Fonts(fonts), Some(Open::Object(parent))) => {
            // Platform 0, as ChemDraw writes it, then the fonts.
            let mut v = 0u16.to_le_bytes().to_vec();
            v.extend_from_slice(&(fonts.len() as u16).to_le_bytes());
            for (id, charset, name) in fonts {
                let name = latin1(&name);
                v.extend_from_slice(&id.to_le_bytes());
                v.extend_from_slice(&charset.to_le_bytes());
                v.extend_from_slice(&(name.len() as u16).to_le_bytes());
                v.extend_from_slice(&name);
            }
            parent.set_property(PROP_FONT_TABLE, v);
        }
        (Open::Run(run, chars), Some(Open::Object(t))) => append_run(t, run, &chars),
        _ => {}
    }
}

/// Add a style run and its characters to a Text object's Text property (and its UTF-8 Text
/// property, when the text needs one).
fn append_run(t: &mut RawCdxObject, run: Run, chars: &str) {
    let (mut runs, bytes) = t.get_property(PROP_TEXT).map(|v| split_cdx_string(v)).unwrap_or_default();
    let mut text = match t.get_property(PROP_UTF8_TEXT) {
        Some(v) => String::from_utf8_lossy(&split_cdx_string(v).1).into_owned(),
        None => bytes.iter().map(|&b| b as char).collect(),
    };
    // Line breaks are carriage returns in CDX.
    let chars: String = chars.replace("\r\n", "\r").replace('\n', "\r");
    runs.push((text.chars().count() as u16, run));
    text.push_str(&chars);
    let cdx_string = |bytes: Vec<u8>| {
        let mut v = (runs.len() as u16).to_le_bytes().to_vec();
        for (start, r) in &runs {
            for x in [*start, r.font, r.face, r.size, r.color] {
                v.extend_from_slice(&x.to_le_bytes());
            }
        }
        v.extend(bytes);
        v
    };
    t.set_property(PROP_TEXT, cdx_string(latin1(&text)));
    if text.chars().any(|c| c as u32 >= 0x100) {
        t.set_property(PROP_UTF8_TEXT, cdx_string(text.into_bytes()));
    }
}

/// A CDXString's style runs and its bytes.
fn split_cdx_string(v: &[u8]) -> (Vec<(u16, Run)>, Vec<u8>) {
    let u = |i: usize| v.get(i..i + 2).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
    let n = u(0) as usize;
    let runs = (0..n)
        .map(|k| {
            let at = 2 + k * 10;
            (u(at), Run { font: u(at + 2), face: u(at + 4), size: u(at + 6), color: u(at + 8) })
        })
        .collect();
    (runs, v.get(2 + n * 10..).unwrap_or_default().to_vec())
}

/// An object, its id and the properties its attributes give.
fn object(tag: u16, element: &str, attrs: &[(String, String)], warnings: &mut Vec<String>) -> RawCdxObject {
    let id = attr(attrs, "id").and_then(|v| v.parse().ok()).unwrap_or(0);
    let mut obj = RawCdxObject::new(tag, id);
    for (name, value) in attrs {
        if name == "id" {
            continue;
        }
        let encoded = if name == "Order" && tag == 0x8005 {
            bond_order(value).map(|v| Some((PROP_BOND_ORDER, v)))
        } else {
            match table::PROPERTIES.iter().find(|p| p.0 == name) {
                Some(&(_, ptag, kind, names)) => encode(kind, names, value).map(|v| v.map(|v| (ptag, v))),
                None => {
                    warnings.push(format!("<{element}> attribute {name} not known, left out"));
                    continue;
                }
            }
        };
        match encoded {
            Ok(Some((ptag, v))) => obj.add_property(ptag, v),
            Ok(None) => {}
            Err(why) => warnings.push(format!("<{element}> {name}=\"{value}\": {why}, left out")),
        }
    }
    obj
}

/// The element's attributes, unescaped.
fn attributes(e: &BytesStart) -> Result<Vec<(String, String)>, CdxError> {
    e.attributes()
        .map(|a| {
            let a = a.map_err(|e| CdxError::DecodeError(format!("CDXML: {e}")))?;
            let value = a.unescape_value().map_err(|e| CdxError::DecodeError(format!("CDXML: {e}")))?;
            Ok((String::from_utf8_lossy(a.key.as_ref()).into_owned(), value.into_owned()))
        })
        .collect()
}

fn attr<'a>(attrs: &'a [(String, String)], key: &str) -> Option<&'a str> {
    attrs.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

/// Give every object read without an id one no other object has.
fn assign_missing_ids(doc: &mut RawCdxObject) {
    fn max_id(o: &RawCdxObject) -> u32 {
        o.children.iter().map(max_id).fold(o.id, u32::max)
    }
    fn fill(o: &mut RawCdxObject, next: &mut u32, root: bool) {
        if o.id == 0 && !root {
            *next += 1;
            o.id = *next;
        }
        for c in &mut o.children {
            fill(c, next, false);
        }
    }
    let mut next = max_id(doc);
    fill(doc, &mut next, true);
}

/// Text in the CDX string encoding (Latin-1; what it cannot hold becomes `?`).
fn latin1(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 0x100 { c as u8 } else { b'?' }).collect()
}

fn charset_code(name: &str) -> u16 {
    CHARSETS
        .iter()
        .find(|(_, n)| n.eq_ignore_ascii_case(name))
        .map(|&(c, _)| c)
        .or_else(|| name.parse().ok())
        .unwrap_or(0)
}

fn bond_order(value: &str) -> Result<Vec<u8>, String> {
    let mut bits = 0u16;
    for token in value.split_whitespace() {
        bits |= BOND_ORDERS
            .iter()
            .find(|(n, _)| *n == token)
            .map(|&(_, b)| b)
            .ok_or_else(|| format!("unknown bond order {token}"))?;
    }
    Ok(bits.to_le_bytes().to_vec())
}

/// An attribute's binary value. `None` for an implied boolean that is not set.
fn encode(kind: Kind, names: &[(i32, &str)], value: &str) -> Result<Option<Vec<u8>>, String> {
    let int = || -> Result<i64, String> {
        if let Some(&(v, _)) = names.iter().find(|(_, n)| *n == value) {
            return Ok(v as i64);
        }
        value
            .trim()
            .parse::<f64>()
            .map(|v| v.round() as i64)
            .map_err(|_| "not a number or known name".to_string())
    };
    let nums = || -> Result<Vec<f64>, String> {
        value.split_whitespace().map(|t| t.parse::<f64>().map_err(|_| format!("{t} is not a number"))).collect()
    };
    let coord = |pt: f64| ((pt * POINT).round() as i32).to_le_bytes();
    let fits = |v: i64, lo: i64, hi: i64| if (lo..=hi).contains(&v) { Ok(v) } else { Err("out of range".to_string()) };
    let v = match kind {
        Kind::I8 => (fits(int()?, i8::MIN as i64, i8::MAX as i64)? as i8).to_le_bytes().to_vec(),
        Kind::U8 => (fits(int()?, 0, u8::MAX as i64)? as u8).to_le_bytes().to_vec(),
        Kind::I16 => (fits(int()?, i16::MIN as i64, i16::MAX as i64)? as i16).to_le_bytes().to_vec(),
        Kind::U16 => (fits(int()?, 0, u16::MAX as i64)? as u16).to_le_bytes().to_vec(),
        Kind::I32 => (fits(int()?, i32::MIN as i64, i32::MAX as i64)? as i32).to_le_bytes().to_vec(),
        Kind::U32 | Kind::ObjectId => (fits(int()?, 0, u32::MAX as i64)? as u32).to_le_bytes().to_vec(),
        Kind::F64 => value.trim().parse::<f64>().map_err(|_| "not a number".to_string())?.to_le_bytes().to_vec(),
        Kind::Bool => vec![u8::from(yes(value)?)],
        Kind::BoolImplied => return Ok(yes(value)?.then(Vec::new)),
        Kind::Coordinate => coord(*nums()?.first().ok_or("empty")?).to_vec(),
        Kind::Point2D => match nums()?[..] {
            [x, y, ..] => [coord(y), coord(x)].concat(),
            _ => return Err("needs x y".into()),
        },
        Kind::Point3D => match nums()?[..] {
            [x, y, z, ..] => [coord(x), coord(y), coord(z)].concat(),
            [x, y] => [coord(x), coord(y), coord(0.0)].concat(),
            _ => return Err("needs x y z".into()),
        },
        Kind::Rectangle => match nums()?[..] {
            [l, t, r, b] => [coord(t), coord(l), coord(b), coord(r)].concat(),
            _ => return Err("needs left top right bottom".into()),
        },
        Kind::ObjectIdArray | Kind::ObjectIdArrayWithCounts => {
            let ids = value
                .split_whitespace()
                .map(|t| t.parse::<u32>().map_err(|_| format!("{t} is not an id")))
                .collect::<Result<Vec<_>, _>>()?;
            let mut v = Vec::new();
            if kind == Kind::ObjectIdArrayWithCounts {
                v.extend_from_slice(&(ids.len() as u16).to_le_bytes());
            }
            for id in ids {
                v.extend_from_slice(&id.to_le_bytes());
            }
            v
        }
        Kind::String => [0u16.to_le_bytes().to_vec(), latin1(&value.replace('\n', "\r"))].concat(),
        Kind::CurvePoints => {
            let n = nums()?;
            let mut v = ((n.len() / 2) as u16).to_le_bytes().to_vec();
            for p in n.chunks_exact(2) {
                v.extend_from_slice(&coord(p[1]));
                v.extend_from_slice(&coord(p[0]));
            }
            v
        }
        Kind::CurvePoints3D => {
            let n = nums()?;
            let mut v = ((n.len() / 3) as u16).to_le_bytes().to_vec();
            for p in n.chunks_exact(3) {
                for x in p {
                    v.extend_from_slice(&coord(*x));
                }
            }
            v
        }
    };
    Ok(Some(v))
}

fn yes(value: &str) -> Result<bool, String> {
    match value {
        "yes" | "true" | "1" => Ok(true),
        "no" | "false" | "0" => Ok(false),
        _ => Err("not yes/no".into()),
    }
}

/// Write a raw object tree (a Document, tag 0x8000) as CDXML. Properties with no CDXML name
/// are left out; see [`write_cdxml_with_warnings`].
pub fn write_cdxml(doc: &RawCdxObject) -> String {
    write_cdxml_with_warnings(doc).0
}

/// Write a raw object tree as CDXML, with the properties and objects that were left out.
pub fn write_cdxml_with_warnings(doc: &RawCdxObject) -> (String, Vec<String>) {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<!DOCTYPE CDXML SYSTEM \"http://www.cambridgesoft.com/xml/cdxml.dtd\" >\n",
    );
    let mut warnings = Vec::new();
    write_object(&mut out, doc, 0, &mut warnings);
    (out, warnings)
}

fn write_object(out: &mut String, obj: &RawCdxObject, depth: usize, warnings: &mut Vec<String>) {
    let name = if obj.tag == TAG_DOCUMENT {
        "CDXML"
    } else {
        match table::OBJECTS.iter().find(|(_, t)| *t == obj.tag) {
            Some((n, _)) => n,
            None => {
                warnings.push(format!("object 0x{:04X} (id {}) has no CDXML element, left out", obj.tag, obj.id));
                return;
            }
        }
    };
    let pad = " ".repeat(depth);
    out.push_str(&format!("{pad}<{name}"));
    if obj.tag != TAG_DOCUMENT {
        out.push_str(&format!(" id=\"{}\"", obj.id));
    }
    let mut inner = String::new();
    for p in &obj.properties {
        match p.tag {
            PROP_COLOR_TABLE => write_colors(&mut inner, &p.value, depth + 1),
            PROP_FONT_TABLE => write_fonts(&mut inner, &p.value, depth + 1),
            PROP_TEXT if obj.tag == TAG_TEXT => write_runs(&mut inner, &p.value, obj.get_property(PROP_UTF8_TEXT)),
            PROP_UTF8_TEXT if obj.tag == TAG_TEXT => {}
            PROP_BOND_ORDER if obj.tag == 0x8005 => {
                let bits = u16::from_le_bytes([p.value.first().copied().unwrap_or(1), p.value.get(1).copied().unwrap_or(0)]);
                let names: Vec<&str> = BOND_ORDERS.iter().filter(|(_, b)| bits & b != 0).map(|(n, _)| *n).collect();
                out.push_str(&format!(" Order=\"{}\"", names.join(" ")));
            }
            // A tag can name different properties of different objects (0x0209 is TopLeft,
            // a 2D point, and an arrow's Center3D): the one whose size the value has.
            tag => match table::PROPERTIES
                .iter()
                .filter(|q| q.1 == tag)
                .min_by_key(|q| fixed_size(q.2).map_or(1, |n| usize::from(n != p.value.len()) * 2))
            {
                Some(&(attr_name, _, kind, names)) => match decode(kind, names, &p.value) {
                    Some(v) => out.push_str(&format!(" {attr_name}=\"{}\"", escape(&v))),
                    None => warnings.push(format!("<{name} id={}> {attr_name}: value could not be read, left out", obj.id)),
                },
                None => warnings.push(format!("<{name} id={}> property 0x{tag:04X} has no CDXML name, left out", obj.id)),
            },
        }
    }
    let mut children = String::new();
    for c in &obj.children {
        write_object(&mut children, c, depth + 1, warnings);
    }
    if inner.is_empty() && children.is_empty() {
        out.push_str("/>\n");
    } else if obj.tag == TAG_TEXT && children.is_empty() {
        // Runs on one line: whitespace between them would be read as text.
        out.push('>');
        out.push_str(inner.trim());
        out.push_str(&format!("</{name}>\n"));
    } else {
        out.push_str(">\n");
        out.push_str(&inner);
        out.push_str(&children);
        out.push_str(&format!("{pad}</{name}>\n"));
    }
}

fn write_colors(out: &mut String, v: &[u8], depth: usize) {
    let u = |i: usize| v.get(i..i + 2).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
    let pad = " ".repeat(depth);
    out.push_str(&format!("{pad}<colortable>\n"));
    for k in 0..u(0) as usize {
        let at = 2 + k * 6;
        let c = |i| fmt(u(at + i) as f64 / 65535.0, 3);
        out.push_str(&format!("{pad} <color r=\"{}\" g=\"{}\" b=\"{}\"/>\n", c(0), c(2), c(4)));
    }
    out.push_str(&format!("{pad}</colortable>\n"));
}

fn write_fonts(out: &mut String, v: &[u8], depth: usize) {
    let u = |i: usize| v.get(i..i + 2).map_or(0, |b| u16::from_le_bytes([b[0], b[1]]));
    let pad = " ".repeat(depth);
    out.push_str(&format!("{pad}<fonttable>\n"));
    let mut at = 4;
    for _ in 0..u(2) {
        let (id, charset, len) = (u(at), u(at + 2), u(at + 4) as usize);
        let name: String = v.get(at + 6..at + 6 + len).unwrap_or_default().iter().map(|&b| b as char).collect();
        let charset = CHARSETS.iter().find(|(c, _)| *c == charset).map_or(charset.to_string(), |(_, n)| n.to_string());
        out.push_str(&format!("{pad} <font id=\"{id}\" charset=\"{charset}\" name=\"{}\"/>\n", escape(&name)));
        at += 6 + len;
    }
    out.push_str(&format!("{pad}</fonttable>\n"));
}

/// A Text property as `<s>` runs; the characters from the UTF-8 Text property when there is
/// one that reads as UTF-8.
fn write_runs(out: &mut String, v: &[u8], utf8: Option<&Vec<u8>>) {
    let (runs, bytes) = split_cdx_string(v);
    let text = match utf8.and_then(|u| String::from_utf8(split_cdx_string(u).1).ok()) {
        Some(text) => text,
        None => bytes.iter().map(|&b| b as char).collect(),
    };
    let text = text.replace('\r', "\n");
    let chars: Vec<char> = text.chars().collect();
    let runs = if runs.is_empty() { vec![(0, RunDefaults::default().0)] } else { runs };
    for (k, (start, r)) in runs.iter().enumerate() {
        let end = runs.get(k + 1).map_or(chars.len(), |n| n.0 as usize).min(chars.len());
        let start = (*start as usize).min(end);
        let s: String = chars[start..end].iter().collect();
        out.push_str(&format!(
            "<s font=\"{}\" size=\"{}\" face=\"{}\" color=\"{}\">{}</s>",
            r.font,
            fmt(r.size as f64 / 20.0, 2),
            r.face,
            r.color,
            escape(&s)
        ));
    }
}

/// The byte length of a value of this kind, when it has only one.
fn fixed_size(kind: Kind) -> Option<usize> {
    match kind {
        Kind::I8 | Kind::U8 | Kind::Bool => Some(1),
        Kind::I16 | Kind::U16 => Some(2),
        Kind::I32 | Kind::U32 | Kind::ObjectId | Kind::Coordinate => Some(4),
        Kind::F64 | Kind::Point2D => Some(8),
        Kind::Point3D => Some(12),
        Kind::Rectangle => Some(16),
        Kind::BoolImplied => Some(0),
        _ => None,
    }
}

/// A property's CDXML text, or `None` when its bytes do not hold the value its type needs.
fn decode(kind: Kind, names: &[(i32, &str)], v: &[u8]) -> Option<String> {
    let i32_at = |i: usize| v.get(i..i + 4).map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    // Six places: enough to read back the same 1/65536 point.
    let pt = |i: usize| i32_at(i).map(|c| fmt(c as f64 / POINT, 6));
    let named = |n: i64| names.iter().find(|(v, _)| *v as i64 == n).map_or(n.to_string(), |(_, s)| s.to_string());
    Some(match kind {
        Kind::I8 => named(*v.first()? as i8 as i64),
        Kind::U8 => named(*v.first()? as i64),
        Kind::I16 => named(i16::from_le_bytes(v.get(..2)?.try_into().ok()?) as i64),
        Kind::U16 => named(u16::from_le_bytes(v.get(..2)?.try_into().ok()?) as i64),
        Kind::I32 => named(i32_at(0)? as i64),
        Kind::U32 | Kind::ObjectId => named(u32::from_le_bytes(v.get(..4)?.try_into().ok()?) as i64),
        Kind::F64 => fmt(f64::from_le_bytes(v.get(..8)?.try_into().ok()?), 6),
        Kind::Bool => if *v.first()? != 0 { "yes" } else { "no" }.to_string(),
        Kind::BoolImplied => "yes".to_string(),
        Kind::Coordinate => pt(0)?,
        Kind::Point2D => format!("{} {}", pt(4)?, pt(0)?),
        Kind::Point3D => format!("{} {} {}", pt(0)?, pt(4)?, pt(8)?),
        Kind::Rectangle => format!("{} {} {} {}", pt(4)?, pt(0)?, pt(12)?, pt(8)?),
        Kind::ObjectIdArray | Kind::ObjectIdArrayWithCounts => {
            let body = if kind == Kind::ObjectIdArrayWithCounts { v.get(2..)? } else { v };
            body.chunks_exact(4)
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }
        Kind::String => {
            let (_, bytes) = split_cdx_string(v);
            bytes.iter().map(|&b| if b == b'\r' { '\n' } else { b as char }).collect()
        }
        Kind::CurvePoints => {
            let n = u16::from_le_bytes(v.get(..2)?.try_into().ok()?) as usize;
            (0..n).map(|k| Some(format!("{} {}", pt(2 + k * 8 + 4)?, pt(2 + k * 8)?))).collect::<Option<Vec<_>>>()?.join(" ")
        }
        Kind::CurvePoints3D => {
            let n = u16::from_le_bytes(v.get(..2)?.try_into().ok()?) as usize;
            (0..n * 3).map(|k| pt(2 + k * 4)).collect::<Option<Vec<_>>>()?.join(" ")
        }
    })
}

/// A number with at most `places` decimals, without trailing zeros.
fn fmt(v: f64, places: usize) -> String {
    let s = format!("{v:.places$}");
    let s = if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s };
    if s == "-0" { "0".to_string() } else { s }
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\n' => out.push_str("&#10;"),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests;
