use crate::cdx_parse_impl::header::CdxHeader;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use std::io::{Read, Result};

pub struct RawCdxParser<R: Read> {
    reader: R,
}

impl<R: Read> RawCdxParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn parse(&mut self) -> Result<RawCdxObject> {
        // Skip the 22-byte header
        let mut header_buf = [0u8; 22];
        self.reader.read_exact(&mut header_buf)?;

        let tag = read_u16_le(&mut self.reader)?;
        if tag & 0x8000 == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected root object tag",
            ));
        }

        let obj = read_object(&mut self.reader, tag)?;
        Ok(obj)
    }

    #[allow(dead_code)]
    fn read_header(&mut self) -> Result<CdxHeader> {
        let mut magic = [0u8; 8];
        self.reader.read_exact(&mut magic)?;
        if &magic != b"VjCD0100" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid CDX header",
            ));
        }

        let reserved_legacy = read_u32_le(&mut self.reader)?;
        let mut reserved_zero = [0u8; 10];
        self.reader.read_exact(&mut reserved_zero)?;

        Ok(CdxHeader {
            magic,
            reserved_legacy,
            reserved_zero,
        })
    }
}

fn read_object<R: Read>(r: &mut R, tag: u16) -> Result<RawCdxObject> {
    let id = read_u32_le(r)?;

    let mut properties = Vec::new();
    let mut children = Vec::new();

    loop {
        let next_tag = read_u16_le(r)?;

        // EndObject marker
        if next_tag == 0x0000 {
            break;
        }

        if next_tag & 0x8000 == 0 {
            properties.push(read_property(r, next_tag)?);
        } else {
            children.push(read_object(r, next_tag)?);
        }
    }

    Ok(RawCdxObject {
        tag,
        id,
        properties,
        children,
    })
}

fn read_u16_le<R: Read>(r: &mut R) -> Result<u16> {
    let mut bytes = [0u8; 2];
    r.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

fn read_u32_le<R: Read>(r: &mut R) -> Result<u32> {
    let mut bytes = [0u8; 4];
    r.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_bytes<R: Read>(r: &mut R, len: usize) -> Result<Vec<u8>> {
    let mut bytes = vec![0u8; len];
    r.read_exact(&mut bytes)?;
    Ok(bytes)
}

fn read_property<R: Read>(r: &mut R, tag: u16) -> Result<RawCdxProperty> {
    // Read property size: if size is 0xFFFF, then read actual size as u32
    let size_or_marker = read_u16_le(r)?;
    let value = if size_or_marker == 0xFFFF {
        let actual_size = read_u32_le(r)?;
        read_bytes(r, actual_size as usize)?
    } else {
        read_bytes(r, size_or_marker as usize)?
    };
    Ok(RawCdxProperty { tag, value })
}
