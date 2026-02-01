use crate::error::CdxError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

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

    /// Encode to binary format (24 bytes: 3 f64 values)
    pub fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(24);
        buf.write_f64::<LittleEndian>(self.red)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.green)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.blue)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }

    /// Decode from binary format (24 bytes: 3 f64 values)
    pub fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 24 {
            return Err(CdxError::DecodeError(
                "Not enough bytes for RGBColor".to_string(),
            ));
        }
        let mut cursor = Cursor::new(data);
        let red = cursor
            .read_f64::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let green = cursor
            .read_f64::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let blue = cursor
            .read_f64::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(RGBColor { red, green, blue })
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

    /// Encode the entire color table to binary
    pub fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::new();

        // Write number of colors as u32
        buf.write_u32::<LittleEndian>(self.colors.len() as u32)
            .map_err(|e| CdxError::DecodeError(e.to_string()))?;

        // Write each color
        for color in &self.colors {
            buf.extend(color.encode()?);
        }

        Ok(buf)
    }

    /// Decode the color table from binary
    pub fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 4 {
            return Err(CdxError::DecodeError(
                "Not enough bytes for ColorTable header".to_string(),
            ));
        }

        let mut cursor = Cursor::new(data);

        // Read number of colors
        let color_count = cursor
            .read_u32::<LittleEndian>()
            .map_err(|e| CdxError::DecodeError(e.to_string()))? as usize;

        if color_count == 0 {
            return Err(CdxError::DecodeError(
                "ColorTable must contain at least one color".to_string(),
            ));
        }

        let mut colors = Vec::new();

        // Read each color (24 bytes each: 3 f64 values)
        for _ in 0..color_count {
            let pos = cursor.position() as usize;
            if pos + 24 > data.len() {
                return Err(CdxError::DecodeError(
                    "Not enough bytes to read color entry".to_string(),
                ));
            }

            let color = RGBColor::decode(&data[pos..])?;
            colors.push(color);

            cursor.set_position((pos + 24) as u64);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_encode_decode() {
        let color = RGBColor::new(0.5, 0.25, 0.75);
        let encoded = color.encode().unwrap();
        let decoded = RGBColor::decode(&encoded).unwrap();

        assert_eq!(color.red, decoded.red);
        assert_eq!(color.green, decoded.green);
        assert_eq!(color.blue, decoded.blue);
    }

    #[test]
    fn test_color_table_encode_decode() {
        let colors = vec![
            RGBColor::new(1.0, 1.0, 1.0), // White
            RGBColor::new(0.0, 0.0, 0.0), // Black
            RGBColor::new(1.0, 0.0, 0.0), // Red
        ];

        let table = ColorTable::new(colors).unwrap();
        let encoded = table.encode().unwrap();
        let decoded = ColorTable::decode(&encoded).unwrap();

        assert_eq!(table.colors.len(), decoded.colors.len());
        for (orig, dec) in table.colors.iter().zip(decoded.colors.iter()) {
            assert_eq!(orig.red, dec.red);
            assert_eq!(orig.green, dec.green);
            assert_eq!(orig.blue, dec.blue);
        }
    }

    #[test]
    fn test_color_table_default() {
        let table = ColorTable::default();
        assert!(table.colors.len() >= 2);
    }

    #[test]
    fn test_color_table_empty_error() {
        let result = ColorTable::new(vec![]);
        assert!(result.is_err());
    }
}
