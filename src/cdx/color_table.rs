use crate::error::CdxError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use crate::cdx::binary_codec::BinaryCodec;
/// An RGB color with components in the range 0.0 to 1.0
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RGBColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RGBColor {
    /// Create a new RGB color
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        RGBColor { red, green, blue }
    }

    /// Encode to binary format (6 bytes: 3 u16 values)
    pub fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        buf.write_u16::<LittleEndian>((self.red * 65535.0) as u16)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_u16::<LittleEndian>((self.green * 65535.0) as u16)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_u16::<LittleEndian>((self.blue * 65535.0) as u16)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }

    /// Decode from binary format (6 bytes: 3 u16 values)
    pub fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 6 {
            return Err(CdxError::DecodeError(
                "Not enough bytes for RGBColor".to_string(),
            ));
        }
        let mut cursor = Cursor::new(data);
        let red = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let green = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let blue = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(RGBColor { 
            red: red as f64 / 65535.0, 
            green: green as f64 / 65535.0, 
            blue: blue as f64 / 65535.0 
        })
    }
}

/// Color Table: A collection of RGB colors used throughout the document
///
/// Color indexes 0 and 1 always correspond to black and white.
/// The first and second colors (indexes 2 and 3) are the default background and foreground colors.
/// Other colors are numbered sequentially.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorTable {
    /// List of RGB colors in the table. Must contain at least one color.
    pub colors: Vec<RGBColor>,
}

impl ColorTable {
    /// Create a new ColorTable with the given colors
    pub fn new(colors: Vec<RGBColor>) -> Result<Self, CdxError> {
        if colors.is_empty() {
            return Err(CdxError::DecodeError(
                "ColorTable must contain at least one color".to_string(),
            ));
        }
        Ok(ColorTable { colors })
    }
}
impl BinaryCodec for ColorTable {
    /// Encode the entire color table to binary
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();
        // Write number of colors
        buf.write_u16::<LittleEndian>(self.colors.len() as u16)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        // Write each color
        for color in &self.colors {
            let color_bytes = color.encode()?;
            buf.extend_from_slice(&color_bytes);
        }
        Ok(buf)
    }

    /// Decode the color table from binary
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 4 {
            return Err(CdxError::DecodeError(
                "Not enough bytes for ColorTable header".to_string(),
            ));
        }

        let mut cursor = Cursor::new(data);

        // Read number of colors
        let color_count = cursor
            .read_u16::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))? as usize;

        if color_count == 0 {
            return Err(CdxError::DecodeError(
                "ColorTable must contain at least one color".to_string(),
            ));
        }

        let mut colors = Vec::new();

        // Read each color (24 bytes each: 3 u16 values)
        for _ in 0..color_count {
            let pos = cursor.position() as usize;
            if pos + 2*3 > data.len() {
                return Err(CdxError::DecodeError(
                    "Not enough bytes to read color entry".to_string(),
                ));
            }

            let color = RGBColor::decode(&data[pos..])?;
            colors.push(color);

            cursor.set_position((pos + 2*3) as u64);
        }

        ColorTable::new(colors)
    }
}

impl Default for ColorTable {
    /// Create a default ColorTable with standard colors
    fn default() -> Self {
        // Standard colors: background (white) and foreground (black)
        // Index 0: Black (1.0, 1.0, 1.0 in some systems, but we use 0,0,0)
        // Index 1: White (0.0, 0.0, 0.0 in some systems, but we use 1,1,1)
        // Index 2: Default background (white)
        // Index 3: Default foreground (black)
        ColorTable {
            colors: vec![
                RGBColor::new(1.0, 1.0, 1.0), // White background
                RGBColor::new(0.0, 0.0, 0.0), // Black foreground
            ],
        }
    }
}
