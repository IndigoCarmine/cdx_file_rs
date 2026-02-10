use crate::cdx::values::BooleanImplied;

/// Represents a SegComponent object (structural group component)
#[derive(Debug, Clone, PartialEq)]
pub struct SegComponent {
    pub id: u32,
    pub width: Option<i32>,
    pub component_is_reactant: Option<BooleanImplied>,
    pub component_is_header: Option<BooleanImplied>,
}

impl SegComponent {
    pub fn new(id: u32) -> Self {
        SegComponent {
            id,
            width: None,
            component_is_reactant: None,
            component_is_header: None,
        }
    }
}
