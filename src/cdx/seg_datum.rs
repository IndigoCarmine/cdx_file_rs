//! segDatum object definition

#[derive(Debug, Clone, PartialEq)]
pub struct SegDatum {
    pub id: u32,
    pub sg_data_type: Option<u8>,
    pub sg_data_value: Option<String>,
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
