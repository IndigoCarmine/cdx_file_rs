//! Binary encoding/decoding for CDXString
//! CDXString is a variable-length data type consisting of:
//! - UINT16: style run count
//! - N StyleRun structs (10 bytes each)
//! - Text content (ISO Latin-1 encoding)

use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::{CDXString, CDXStyleRun};
use crate::error::CdxError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

impl BinaryCodec for CDXString {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();

        // Write style run count (UINT16)
        let run_count = self.style_runs.len() as u16;
        buf.write_u16::<LittleEndian>(run_count)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;

        // Write style runs (10 bytes each: char_index + font_index + font_face + font_size + color_index)
        for run in &self.style_runs {
            buf.write_u16::<LittleEndian>(run.char_index)
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            buf.write_u16::<LittleEndian>(run.font_index)
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            buf.write_u16::<LittleEndian>(run.font_face)
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            buf.write_u16::<LittleEndian>(run.font_size)
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            buf.write_u16::<LittleEndian>(run.color_index)
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        }

        // Write text as ISO Latin-1
        buf.extend_from_slice(self.text.as_bytes());

        Ok(buf)
    }

    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 2 {
            return Err(CdxError::DecodeError(
                "Not enough bytes for CDXString".to_string(),
            ));
        }

        let mut cursor = Cursor::new(data);

        // Read style run count
        let run_count = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))? as usize;

        // Read style runs
        let mut style_runs = Vec::with_capacity(run_count);
        for _ in 0..run_count {
            let char_index = cursor
                .read_u16::<LittleEndian>()
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            let font_index = cursor
                .read_u16::<LittleEndian>()
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            let font_face = cursor
                .read_u16::<LittleEndian>()
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            let font_size = cursor
                .read_u16::<LittleEndian>()
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;
            let color_index = cursor
                .read_u16::<LittleEndian>()
                .map_err(|e| CdxError::DecodeError(e.to_string()))?;

            style_runs.push(CDXStyleRun {
                char_index,
                font_index,
                font_face,
                font_size,
                color_index,
            });
        }

        // Read text as ISO Latin-1
        let text_start = cursor.position() as usize;
        let text_bytes = &data[text_start..];

        // Decode as ISO Latin-1 (Latin1 to UTF-8)
        let text = text_bytes.iter().map(|&b| b as char).collect::<String>();

        Ok(CDXString { style_runs, text })
    }
}
