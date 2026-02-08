//! segDatum object definition

#[derive(Debug, Clone, PartialEq)]
pub struct SegDatum {
    pub id: u32,
    pub sg_data_type: u8,
    pub sg_data_value: String,
    pub sg_property_type: u8,
    pub is_read_only: Option<bool>,
}
