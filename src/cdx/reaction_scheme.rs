use serde::{Deserialize, Serialize};

/// ReactionScheme (反応スキーム) object
/// Represents a reaction scheme containing one or more reaction steps
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionScheme {
    pub id: u32,
}

impl ReactionScheme {
    pub fn new(id: u32) -> Self {
        ReactionScheme { id }
    }
}
