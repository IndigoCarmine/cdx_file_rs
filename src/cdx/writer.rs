use crate::Result;
use crate::cdx::document::CdxDocument;
use crate::cdx::nodes::CdxNode;
use binrw::BinWriterExt;
use std::io::{Seek, Write};

pub struct CdxWriter<W: Write + Seek> {
    writer: W,
}

impl<W: Write + Seek> CdxWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, doc: &CdxDocument) -> Result<()> {
        self.writer.write_le(&doc.header)?;
        for node in &doc.root {
            self.write_node(node)?;
        }
        self.writer.write_le(&0u16)?;
        Ok(())
    }

    fn write_node(&mut self, node: &CdxNode) -> Result<()> {
        match node {
            CdxNode::Object(obj) => {
                self.writer.write_le(&obj.tag)?;
                self.writer.write_le(&obj.id)?;
                for child in &obj.children {
                    self.write_node(child)?;
                }
                self.writer.write_le(&0u16)?;
            }
            CdxNode::Property(prop) => {
                self.writer.write_le(&prop.tag)?;
                let data = prop.value.to_bytes();
                if data.len() >= 0xFFFF {
                    self.writer.write_le(&0xFFFFu16)?;
                    self.writer.write_le(&(data.len() as u32))?;
                } else {
                    self.writer.write_le(&(data.len() as u16))?;
                }
                self.writer.write_all(&data)?;
            }
        }
        Ok(())
    }
}
