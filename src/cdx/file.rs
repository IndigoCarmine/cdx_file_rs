
use dendron::tree::{Tree};

use crate::error::CdxError;

pub use super::bond::Bond;
pub use super::document::Document;
pub use super::fragment::Fragment;
pub use super::group::Group;
pub use super::node::Node;
pub use super::page::Page;
pub use super::text::TextObject;
pub use super::tlc_lane::TlcLane;
pub use super::tlc_plate::TLCPlate;

pub struct CdxFile{
    pub tree:Tree<NodePayload>

}

impl CdxFile {
    pub fn get_document(&self) -> Result<Document, CdxError> {
        let root = self.tree.root();

        match &*root.borrow_data() {
            NodePayload::Document(doc) => Ok(doc.clone()),
            other => Err(CdxError::Parse(format!(
                "root node is not Document: found {:?}",
                other
            ))),
        }
    }
}



macro_rules! define_node_payload {
    ( $( $ty:ident ),* $(,)? ) => {
        #[derive(Debug)]
        pub enum NodePayload {
            $(
                $ty($ty),
            )*
        }
    };
}


define_node_payload!(
    Bond,
    Document,
    Fragment,
    Group,
    Node,
    Page,
    TextObject,
    TlcLane,
    TLCPlate,
    );