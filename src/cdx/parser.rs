use crate::cdx::document::CdxDocument;
use crate::cdx::nodes::{CdxNode, CdxObject, CdxProperty};
use crate::cdx::values::CdxValue;
use crate::{CdxError, CdxHeader, Result};
use binrw::BinReaderExt;
use std::io::{Read, Seek};

pub struct CdxParser<R: Read + Seek> {
    reader: R,
}

impl<R: Read + Seek> CdxParser<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn parse(&mut self) -> Result<CdxDocument> {
        let header = self.reader.read_le::<CdxHeader>()?;
        if &header.magic != b"VjCD0100" {
            return Err(CdxError::InvalidHeader);
        }

        let mut root = Vec::new();
        while let Some(node) = self.read_node()? {
            root.push(node);
        }

        Ok(CdxDocument { header, root })
    }

    fn read_node(&mut self) -> Result<Option<CdxNode>> {
        let tag = match self.reader.read_le::<u16>() {
            Ok(t) => t,
            Err(_) => return Ok(None),
        };

        if tag == 0 {
            return Ok(None);
        }

        if tag & 0x8000 != 0 {
            let id = self.reader.read_le::<u32>()?;
            let mut children = Vec::new();
            while let Some(child) = self.read_node()? {
                children.push(child);
            }
            Ok(Some(CdxNode::Object(CdxObject { tag, id, children })))
        } else {
            let mut len = self.reader.read_le::<u16>()? as usize;
            if len == 0xFFFF {
                len = self.reader.read_le::<u32>()? as usize;
            }
            let mut data = vec![0u8; len];
            self.reader.read_exact(&mut data)?;
            Ok(Some(CdxNode::Property(CdxProperty {
                tag,
                value: CdxValue::from_bytes(tag, &data),
            })))
        }
    }
}
