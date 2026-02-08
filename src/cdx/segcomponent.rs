
/// Represents a SegComponent object (structural group component)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SegComponent {
    pub id: u32,
    pub width: Option<i32>,
    pub component_is_reactant: Option<bool>,
    pub component_is_header: Option<bool>,
}
