use crate::cdx_parse_impl::header::CdxHeader;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use binrw::BinWriterExt;
use std::io::{Error, Result, Seek, Write};

pub struct CdxWriter<W: Write + Seek> {
    pub(crate) writer: W,
}

impl<W: Write + Seek> CdxWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }

    pub fn write(&mut self, doc: &RawCdxObject) -> Result<()> {
        // Write default header
        let header = CdxHeader::default();
        self.writer.write_all(&header.magic)?;
        self.writer
            .write_all(&header.reserved_legacy.to_be_bytes())?;
        self.writer.write_all(&header.reserved_zero)?;

        // Write the document object
        self.write_object(doc)?;

        // Write EOF marker (2 zero bytes)
        self.writer.write_all(&[0u8; 2])?;

        Ok(())
    }

    fn write_object(&mut self, obj: &RawCdxObject) -> Result<()> {
        self.writer.write_le(&obj.tag).map_err(Error::other)?;
        self.writer.write_le(&obj.id).map_err(Error::other)?;
        for prop in &obj.properties {
            self.write_property(prop)?;
        }
        for child in &obj.children {
            self.write_object(child)?;
        }
        self.writer.write_le(&0u16).map_err(Error::other)?;
        Ok(())
    }

    fn write_property(&mut self, prop: &RawCdxProperty) -> Result<()> {
        self.writer.write_le(&prop.tag).map_err(Error::other)?;
        let data = &prop.value;
        if data.len() >= 0xFFFF {
            self.writer.write_le(&0xFFFFu16).map_err(Error::other)?;
            self.writer
                .write_le(&(data.len() as u32))
                .map_err(Error::other)?;
        } else {
            self.writer
                .write_le(&(data.len() as u16))
                .map_err(Error::other)?;
        }
        self.writer.write_all(data)?;
        Ok(())
    }
}
