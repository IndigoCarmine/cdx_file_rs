/// Binary encoding/decoding utilities for CDX values
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use crate::cdx::values::{Point2d, Point3d, Rectangle};
use crate::error::CdxError;
pub trait BinaryCodec: Sized {
    fn encode(&self) -> Result<Vec<u8>, CdxError>;
    fn decode(data: &[u8]) -> Result<Self, CdxError>;
}

// Primitive types
impl BinaryCodec for u8 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        Ok(vec![*self])
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 1 {
            return Err(CdxError::DecodeError("Not enough bytes for u8".to_string()));
        }
        Ok(data[0])
    }
}

impl BinaryCodec for i8 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        Ok(vec![*self as u8])
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 1 {
            return Err(CdxError::DecodeError("Not enough bytes for i8".to_string()));
        }
        Ok(data[0] as i8)
    }
}

impl BinaryCodec for u16 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(2);
        buf.write_u16::<LittleEndian>(*self).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        cursor.read_u16::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for i16 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(2);
        buf.write_i16::<LittleEndian>(*self).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        cursor.read_i16::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for u32 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(4);
        buf.write_u32::<LittleEndian>(*self).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        cursor.read_u32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for i32 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(4);
        buf.write_i32::<LittleEndian>(*self).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        cursor.read_i32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for f64 {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(8);
        buf.write_f64::<LittleEndian>(*self).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        let mut cursor = Cursor::new(data);
        cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for bool {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        Ok(vec![if *self { 1 } else { 0 }])
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 1 {
            return Err(CdxError::DecodeError("Not enough bytes for bool".to_string()));
        }
        Ok(data[0] != 0)
    }
}

impl BinaryCodec for String {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        Ok(self.as_bytes().to_vec())
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        String::from_utf8(data.to_vec()).map_err(|e| CdxError::DecodeError(e.to_string()))
    }
}

impl BinaryCodec for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        Ok(self.clone())
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        Ok(data.to_vec())
    }
}

// Vector of u32 (for ObjectIDArray)
pub fn encode_u32_array(data: &[u32]) -> Result<Vec<u8>, CdxError> {
    let mut buf = Vec::with_capacity(data.len() * 4);
    for &val in data {
        buf.write_u32::<LittleEndian>(val).map_err(|e| CdxError::DecodeError(e.to_string()))?;
    }
    Ok(buf)
}

pub fn decode_u32_array(data: &[u8]) -> Result<Vec<u32>, CdxError> {
    let mut cursor = Cursor::new(data);
    let mut result = Vec::new();
    while cursor.position() < data.len() as u64 {
        result.push(cursor.read_u32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?);
    }
    Ok(result)
}

impl BinaryCodec for Point2d {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        // CDX format uses i32 (fixed point with 16 fractional bits)
        let mut buf = Vec::with_capacity(8);
        let x_fixed = (self.x * 65536.0) as i32;
        let y_fixed = (self.y * 65536.0) as i32;
        buf.write_i32::<LittleEndian>(y_fixed).map_err(|e| CdxError::DecodeError(e.to_string()))?;  // Note: Y comes first in CDX format
        buf.write_i32::<LittleEndian>(x_fixed).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 8 {
            return Err(CdxError::DecodeError(format!("Not enough bytes for Point2d: got {}, need 8", data.len())));
        }
        let mut cursor = Cursor::new(data);
        // CDX format: Y coordinate first, then X, both as i32 (fixed point with 16 fractional bits)
        let y_fixed = cursor.read_i32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let x_fixed = cursor.read_i32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let x = x_fixed as f64 / 65536.0;
        let y = y_fixed as f64 / 65536.0;
        Ok(Point2d { x, y })
    }
}

impl BinaryCodec for Point3d {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(24);
        buf.write_f64::<LittleEndian>(self.x).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.y).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.z).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        if data.len() < 24 {
            return Err(CdxError::DecodeError("Not enough bytes for Point3d".to_string()));
        }
        let mut cursor = Cursor::new(data);
        let x = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let y = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
        let z = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(Point3d { x, y, z })
    }
}

impl BinaryCodec for Rectangle {
    fn encode(&self) -> Result<Vec<u8>, CdxError> {
        let mut buf = Vec::with_capacity(32);
        buf.write_f64::<LittleEndian>(self.left).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.top).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.right).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        buf.write_f64::<LittleEndian>(self.bottom).map_err(|e| CdxError::DecodeError(e.to_string()))?;
        Ok(buf)
    }
    fn decode(data: &[u8]) -> Result<Self, CdxError> {
        // Handle both 32-byte (Rectangle: 4×f64) and 16-byte (2×Point2d: i32 fixed-point) formats
        match data.len() {
            32 => {
                // Standard 32-byte Rectangle format: left, top, right, bottom as f64
                let mut cursor = Cursor::new(data);
                let left = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
                let top = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
                let right = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
                let bottom = cursor.read_f64::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?;
                Ok(Rectangle { left, top, right, bottom })
            }
            16 => {
                // 16-byte format: 2 Point2d values (head and tail) as i32 fixed-point
                // This is used for arrow graphics where the bounding box represents line endpoints
                let head = Point2d::decode(&data[0..8])?;
                let tail = Point2d::decode(&data[8..16])?;
                
                // Create a rectangle from the two points
                let left = head.x.min(tail.x);
                let right = head.x.max(tail.x);
                let top = head.y.min(tail.y);
                let bottom = head.y.max(tail.y);
                
                Ok(Rectangle { left, top, right, bottom })
            }
            _ => {
                Err(CdxError::DecodeError(format!("Invalid Rectangle size: got {}, expected 16 or 32 bytes", data.len())))
            }
        }
    }
}

// Helper functions for ObjectIDArray (Vec<u32>) - stored as little-endian u32 values
pub fn encode_object_id_array(data: &[u32]) -> Result<Vec<u8>, CdxError> {
    let mut buf = Vec::with_capacity(data.len() * 4);
    for &val in data {
        buf.write_u32::<LittleEndian>(val).map_err(|e| CdxError::DecodeError(e.to_string()))?;
    }
    Ok(buf)
}

pub fn decode_object_id_array(data: &[u8]) -> Result<Vec<u32>, CdxError> {
    let mut cursor = Cursor::new(data);
    let mut result = Vec::new();
    while cursor.position() < data.len() as u64 {
        result.push(cursor.read_u32::<LittleEndian>().map_err(|e| CdxError::DecodeError(e.to_string()))?);
    }
    Ok(result)
}

// Helper functions for CdxString (String with limited properties)
// For now, treat as UTF-8 string encoding
pub fn encode_cdx_string(s: &str) -> Result<Vec<u8>, CdxError> {
    Ok(s.as_bytes().to_vec())
}

pub fn decode_cdx_string(data: &[u8]) -> Result<String, CdxError> {
    String::from_utf8(data.to_vec()).map_err(|e| CdxError::DecodeError(e.to_string()))
}

