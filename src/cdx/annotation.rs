// =======================
// Annotation Object: Fragment? anotation
// =======================
/// tag - 0x802B
/// 
/// Properties:
/// - KeyWord (0x1500) - CDX String
/// - Content (0x1501) - CDX String
use crate::cdx::values::CDXString;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    pub id: u32,
    pub keyword: Option<CDXString>,
    pub content: Option<CDXString>,
}

impl Annotation {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            keyword: None,
            content: None,
        }
    }
}
