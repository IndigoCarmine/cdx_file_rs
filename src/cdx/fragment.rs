use serde::{Deserialize, Serialize};

/// Fragment Object: Chemically meaningful collection of nodes and bonds
/// A Fragment object is a collection of nodes and their connectivity (bonds).
/// Generally, all nodes within a fragment will be connected, but this is not strictly guaranteed.
/// Unlike Group objects, Fragment objects are guaranteed to be chemically meaningful.
/// A Fragment has no required objects or properties, but a Fragment without any objects is pretty useless.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fragment {
    pub id: u32,
    /// Bounding rectangle of the fragment (Optional)
    pub bounding_box: Option<crate::cdx::values::Rectangle>,
    /// Indicates that the molecule is a racemic mixture (Optional)
    pub mole_racemic: Option<bool>,
    /// Indicates that the molecule has known absolute configuration (Optional)
    pub mole_absolute: Option<bool>,
    /// Indicates that the molecule has known relative stereochemistry, but unknown absolute configuration (Optional)
    pub mole_relative: Option<bool>,
    /// Average molecular weight (Optional)
    pub mole_weight: Option<f64>,
    /// Ordered list of fragment attachment points (Optional)
    pub frag_connection_order: Option<Vec<u32>>,
}

impl Fragment {
    /// Create a new Fragment with required properties
    pub fn new(id: u32) -> Self {
        Fragment {
            id,
            bounding_box: None,
            mole_racemic: None,
            mole_absolute: None,
            mole_relative: None,
            mole_weight: None,
            frag_connection_order: None,
        }
    }
}
