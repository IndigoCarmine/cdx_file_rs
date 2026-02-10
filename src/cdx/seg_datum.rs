//! segDatum object definition


use crate::cdx::binary_codec::BinaryCodec;

#[derive(Debug, Clone, PartialEq)]
pub enum SegDatumData{
    Float64(f64),
    String(String),
}
impl BinaryCodec for SegDatumData {
    fn encode(&self) -> Result<Vec<u8>, crate::error::CdxError> {
        match self {
            SegDatumData::Float64(v) => v.encode(),
            SegDatumData::String(s) => s.encode(),
        }
    }

    fn decode(data: &[u8]) -> Result<Self, crate::error::CdxError> {
        // This is a placeholder implementation.
        // Actual implementation would depend on how to distinguish between Float64 and String.
        if data.len() == 8 {
            let value = f64::decode(data)?;
            Ok(SegDatumData::Float64(value))
        } else {
            let value = String::decode(data)?;
            Ok(SegDatumData::String(value))
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SegDatum {
    pub id: u32,
    pub sg_data_type: Option<u8>,
    pub sg_data_value: Option<SegDatumData>,
    pub sg_property_type: Option<u8>,
    pub is_read_only: Option<bool>,
}
impl SegDatum {
    pub fn new(id: u32) -> Self {
        SegDatum {
            id,
            sg_data_type: None,
            sg_data_value: None,
            sg_property_type: None,
            is_read_only: None,
        }
    }
}
