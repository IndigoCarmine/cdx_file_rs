use crate::cdx::geometry::{Point2d, Point3d, Rectangle};
use crate::cdx::tags;
use crate::cdx::text::{FontEntry, FontStyle, StyleRun, StyledString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CdxValue {
    Raw(Vec<u8>),
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Float64(f64),
    Boolean(bool),
    BooleanImplied(bool),
    String(StyledString),
    Point2d(Point2d),
    Point3d(Point3d),
    Rectangle(Rectangle),
    Color { r: u16, g: u16, b: u16 },
    ColorList(Vec<(u16, u16, u16)>),
    FontList { os_type: u16, fonts: Vec<FontEntry> },
    ObjectIDArray(Vec<u32>),
    Int16List(Vec<i16>),
    Date(u32),
    Coordinate(f64),
}

impl CdxValue {
    pub fn from_bytes(tag: u16, data: &[u8]) -> Self {
        match tag {
            tags::FONT_TABLE if data.len() >= 4 => {
                let os_type = u16::from_le_bytes([data[0], data[1]]);
                let num_fonts = u16::from_le_bytes([data[2], data[3]]) as usize;
                let mut fonts = Vec::new();
                let mut offset = 4;
                for _ in 0..num_fonts {
                    if offset + 6 > data.len() {
                        break;
                    }
                    let id = u16::from_le_bytes([data[offset], data[offset + 1]]);
                    let charset = u16::from_le_bytes([data[offset + 2], data[offset + 3]]);
                    let name_len =
                        u16::from_le_bytes([data[offset + 4], data[offset + 5]]) as usize;
                    offset += 6;
                    if offset + name_len > data.len() {
                        break;
                    }
                    let name =
                        String::from_utf8_lossy(&data[offset..offset + name_len]).to_string();
                    fonts.push(FontEntry { id, charset, name });
                    offset += name_len;
                }
                CdxValue::FontList { os_type, fonts }
            }
            tags::COLOR_TABLE if data.len() >= 2 => {
                let num_colors = u16::from_le_bytes([data[0], data[1]]) as usize;
                let mut colors = Vec::new();
                for i in 0..num_colors {
                    let base = 2 + i * 6;
                    if base + 6 > data.len() {
                        break;
                    }
                    colors.push((
                        u16::from_le_bytes([data[base], data[base + 1]]),
                        u16::from_le_bytes([data[base + 2], data[base + 3]]),
                        u16::from_le_bytes([data[base + 4], data[base + 5]]),
                    ));
                }
                CdxValue::ColorList(colors)
            }
            0x0001
            | 0x0003
            | 0x0004
            | 0x0006
            | 0x0008
            | 0x0009
            | 0x000B
            | 0x000C
            | 0x0010
            | 0x0433
            | 0x0439
            | 0x0449
            | tags::TEXT_STRING
            | tags::TEXT_STRING_ALT
            | 0x0815
            | 0x0817
            | 0x0A2A
            | 0x0A84
            | 0x0A85 => {
                if data.len() >= 2 {
                    let style_runs_count = u16::from_le_bytes([data[0], data[1]]) as usize;
                    let header_len = 2 + style_runs_count * 10;
                    if data.len() >= header_len {
                        let mut runs = Vec::new();
                        for i in 0..style_runs_count {
                            let base = 2 + i * 10;
                            runs.push(StyleRun {
                                start: u16::from_le_bytes([data[base], data[base + 1]]),
                                style: FontStyle {
                                    font_id: u16::from_le_bytes([data[base + 2], data[base + 3]]),
                                    face: u16::from_le_bytes([data[base + 4], data[base + 5]]),
                                    size: u16::from_le_bytes([data[base + 6], data[base + 7]]),
                                    color: u16::from_le_bytes([data[base + 8], data[base + 9]]),
                                },
                            });
                        }
                        let text = String::from_utf8_lossy(&data[header_len..]).to_string();
                        return CdxValue::String(StyledString { text, runs });
                    }
                }
                CdxValue::Raw(data.to_vec())
            }
            0x0421 | 0x0422 | 0x0425 | 0x0426 | 0x0428 | 0x0438 | 0x043F | 0x0606 | 0x0A3B
            | 0x0B80
                if data.len() == 1 =>
            {
                CdxValue::Int8(i8::from_le_bytes([data[0]]))
            }
            0x0423 | 0x0435 | 0x0436 | 0x0608 | 0x0609 if data.len() == 1 => {
                CdxValue::Uint8(data[0])
            }
            0x000A
            | tags::BG_COLOR
            | tags::ELEMENT
            | 0x0420
            | 0x043D
            | 0x043E
            | 0x081A
            | 0x081B
            | 0x081C
            | 0x081D
            | 0x081E
            | 0x081F
            | 0x0820
            | 0x0821
            | 0x0A08
            | 0x0A20
            | 0x0A22
            | 0x0A29
            | 0x0A3C
            | 0x0A3D
            | 0x0A82
            | 0x0A83
            | 0x0A87
            | 0x0B02
            | 0x0D00
            | 0x1200
            | 0x1201
            | tags::BOND_ORDER
                if data.len() == 2 =>
            {
                CdxValue::Int16(i16::from_le_bytes([data[0], data[1]]))
            }
            tags::FG_COLOR
            | 0x042B
            | 0x0447
            | 0x080E
            | 0x080F
            | 0x0810
            | 0x0A30
            | 0x0A31
            | 0x0A32
            | 0x0A33
            | 0x0A34
            | 0x0A38
            | 0x0A3E
            | 0x0A3F
            | 0x1208
                if data.len() == 2 =>
            {
                CdxValue::Uint16(u16::from_le_bytes([data[0], data[1]]))
            }
            0x0205 | 0x0803 | 0x0A6A | 0x0A6B | 0x0A6C | 0x0A6D | 0x0AB4 | 0x0AB5 | 0x0AB6
            | 0x0AB9 | 0x0AF0
                if data.len() == 4 =>
            {
                CdxValue::Int32(i32::from_le_bytes([data[0], data[1], data[2], data[3]]))
            }
            0x0013
            | 0x0434
            | tags::BOND_BEGIN
            | tags::BOND_END
            | 0x0A2B
            | 0x0A2C
            | 0x0A2D
            | 0x0BB0
            | 0x0BB1
                if data.len() == 4 =>
            {
                CdxValue::Uint32(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
            }
            0x0504 | 0x0A28 | 0x0A80 | 0x0A81 | 0x0A86 | 0x0A88 | 0x0A89 | 0x0AA0 | 0x0AA1
            | 0x0AB0 | 0x0ABA | 0x0B81 | 0x0B84 | 0x0B85
                if data.len() == 8 =>
            {
                CdxValue::Float64(f64::from_le_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]))
            }
            0x0011 | 0x043A | 0x043B | 0x043C | 0x0442 | 0x0443 | 0x0444 | 0x0445 | 0x0500
            | 0x0501 | 0x0502 | 0x060C | 0x060D | 0x060F | 0x0708 | 0x080D | 0x0AA2 | 0x0AA3
            | 0x0AA5 | 0x0AB2 | 0x0A3A | 0x0BB2
                if data.len() == 1 =>
            {
                CdxValue::Boolean(data[0] != 0)
            }
            0x000F | 0x0424 | 0x0427 | 0x0429 | 0x0448 | 0x0819 | 0x0900 | 0x0A39 | 0x0AA4
            | 0x0AB3 | 0x0AB7 | 0x0B86 | 0x0B87 | 0x0B88 | 0x1203 | 0x1204 | 0x1205 | 0x1206
            | 0x1207
                if data.is_empty() =>
            {
                CdxValue::BooleanImplied(true)
            }
            0x1202 | 0x0D05 => {
                if data.len() == 8 {
                    CdxValue::Float64(f64::from_le_bytes([
                        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                    ]))
                } else {
                    CdxValue::Raw(data.to_vec())
                }
            }
            tags::POSITION
            | tags::EXTENT
            | 0x0209
            | 0x020A
            | 0x020B
            | 0x020C
            | 0x0824
            | 0x0826
            | 0x0901
            | 0x0902
                if data.len() == 8 =>
            {
                let y = i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f64 / 65536.0;
                let x = i32::from_le_bytes([data[4], data[5], data[6], data[7]]) as f64 / 65536.0;
                CdxValue::Point2d(Point2d { x, y })
            }
            0x0201 | 0x0203 | tags::HEAD_3D | tags::TAIL_3D | 0x020D | 0x020E | 0x020F
                if data.len() == 12 =>
            {
                let x = i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f64 / 65536.0;
                let y = i32::from_le_bytes([data[4], data[5], data[6], data[7]]) as f64 / 65536.0;
                let z = i32::from_le_bytes([data[8], data[9], data[10], data[11]]) as f64 / 65536.0;
                CdxValue::Point3d(Point3d { x, y, z })
            }
            tags::BOUNDING_BOX | 0x0802 | 0x0B00 | 0x0B01 if data.len() == 16 => {
                let top = i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f64 / 65536.0;
                let left =
                    i32::from_le_bytes([data[4], data[5], data[6], data[7]]) as f64 / 65536.0;
                let bottom =
                    i32::from_le_bytes([data[8], data[9], data[10], data[11]]) as f64 / 65536.0;
                let right =
                    i32::from_le_bytes([data[12], data[13], data[14], data[15]]) as f64 / 65536.0;
                CdxValue::Rectangle(Rectangle {
                    top,
                    left,
                    bottom,
                    right,
                })
            }
            0x0805 | 0x0806 | 0x0807 | 0x0808 | 0x0809 | 0x0812 | 0x0813 | 0x0814 | 0x0816
            | 0x0818 | 0x0822 | 0x0ACF
                if data.len() == 4 =>
            {
                CdxValue::Coordinate(
                    i32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f64 / 65536.0,
                )
            }
            0x0431 | 0x0505 | 0x060B | 0x060E | 0x0A27 | 0x0B82 | 0x0C00 | 0x0C01 | 0x0C02
            | 0x0C03 | 0x0C04 | 0x0C05
                if data.len().is_multiple_of(4) =>
            {
                let mut ids = Vec::new();
                for i in 0..(data.len() / 4) {
                    let base = i * 4;
                    ids.push(u32::from_le_bytes([
                        data[base],
                        data[base + 1],
                        data[base + 2],
                        data[base + 3],
                    ]));
                }
                CdxValue::ObjectIDArray(ids)
            }
            0x0704 if data.len() >= 2 => {
                let count = u16::from_le_bytes([data[0], data[1]]) as usize;
                let mut list = Vec::new();
                for i in 0..count {
                    let base = 2 + i * 2;
                    if base + 2 > data.len() {
                        break;
                    }
                    list.push(i16::from_le_bytes([data[base], data[base + 1]]));
                }
                CdxValue::Int16List(list)
            }
            0x0002 | 0x0005 if data.len() == 4 => {
                CdxValue::Date(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
            }
            _ => CdxValue::Raw(data.to_vec()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            CdxValue::Raw(data) => data.clone(),
            CdxValue::Int8(v) => vec![*v as u8],
            CdxValue::Uint8(v) => vec![*v],
            CdxValue::Int16(v) => v.to_le_bytes().to_vec(),
            CdxValue::Uint16(v) => v.to_le_bytes().to_vec(),
            CdxValue::Int32(v) => v.to_le_bytes().to_vec(),
            CdxValue::Uint32(v) => v.to_le_bytes().to_vec(),
            CdxValue::Float64(v) => v.to_le_bytes().to_vec(),
            CdxValue::Boolean(v) => vec![if *v { 1 } else { 0 }],
            CdxValue::BooleanImplied(_) => vec![],
            CdxValue::String(s) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&(s.runs.len() as u16).to_le_bytes());
                for run in &s.runs {
                    bytes.extend_from_slice(&run.start.to_le_bytes());
                    bytes.extend_from_slice(&run.style.font_id.to_le_bytes());
                    bytes.extend_from_slice(&run.style.face.to_le_bytes());
                    bytes.extend_from_slice(&run.style.size.to_le_bytes());
                    bytes.extend_from_slice(&run.style.color.to_le_bytes());
                }
                bytes.extend_from_slice(s.text.as_bytes());
                bytes
            }
            CdxValue::Point2d(p) => {
                let mut bytes = Vec::new();
                let y = (p.y * 65536.0) as i32;
                let x = (p.x * 65536.0) as i32;
                bytes.extend_from_slice(&y.to_le_bytes());
                bytes.extend_from_slice(&x.to_le_bytes());
                bytes
            }
            CdxValue::Point3d(p) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&((p.x * 65536.0) as i32).to_le_bytes());
                bytes.extend_from_slice(&((p.y * 65536.0) as i32).to_le_bytes());
                bytes.extend_from_slice(&((p.z * 65536.0) as i32).to_le_bytes());
                bytes
            }
            CdxValue::Rectangle(r) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&((r.top * 65536.0) as i32).to_le_bytes());
                bytes.extend_from_slice(&((r.left * 65536.0) as i32).to_le_bytes());
                bytes.extend_from_slice(&((r.bottom * 65536.0) as i32).to_le_bytes());
                bytes.extend_from_slice(&((r.right * 65536.0) as i32).to_le_bytes());
                bytes
            }
            CdxValue::Color { r, g, b } => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&r.to_le_bytes());
                bytes.extend_from_slice(&g.to_le_bytes());
                bytes.extend_from_slice(&b.to_le_bytes());
                bytes
            }
            CdxValue::ColorList(colors) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&(colors.len() as u16).to_le_bytes());
                for (r, g, b) in colors {
                    bytes.extend_from_slice(&r.to_le_bytes());
                    bytes.extend_from_slice(&g.to_le_bytes());
                    bytes.extend_from_slice(&b.to_le_bytes());
                }
                bytes
            }
            CdxValue::FontList { os_type, fonts } => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&os_type.to_le_bytes());
                bytes.extend_from_slice(&(fonts.len() as u16).to_le_bytes());
                for font in fonts {
                    bytes.extend_from_slice(&font.id.to_le_bytes());
                    bytes.extend_from_slice(&font.charset.to_le_bytes());
                    bytes.extend_from_slice(&(font.name.len() as u16).to_le_bytes());
                    bytes.extend_from_slice(font.name.as_bytes());
                }
                bytes
            }
            CdxValue::ObjectIDArray(ids) => {
                let mut bytes = Vec::new();
                for id in ids {
                    bytes.extend_from_slice(&id.to_le_bytes());
                }
                bytes
            }
            CdxValue::Int16List(v) => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&(v.len() as u16).to_le_bytes());
                for x in v {
                    bytes.extend_from_slice(&x.to_le_bytes());
                }
                bytes
            }
            CdxValue::Date(v) => v.to_le_bytes().to_vec(),
            CdxValue::Coordinate(v) => ((*v * 65536.0) as i32).to_le_bytes().to_vec(),
        }
    }
}
