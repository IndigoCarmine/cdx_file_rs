use crate::cdx::file::{NodePayload,Bond,Border,Constraint,Document,Fragment,Geometry,Graphic,Group,Node,ObjectTag,Page,ReactionScheme,ReactionStep,TextObject,TLCPlate,TlcLane,Arrow,UnknownObject802B};
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx::file::CdxFile;
use crate::error::CdxError;

#[macro_export]
macro_rules! define_node_payload {
    (
             $( $ty:ident ),* $(,)? 
    ) => {
        impl NodePayload {
            pub fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
                match raw.tag {
                    $(
                        <$ty as TaggedObject>::TAG =>
                            Ok(NodePayload::$ty(<$ty>::from_raw(raw)?)),
                    )*
                    _ => Err(CdxError::Parse(format!("found unknown tag={}", raw.tag))),
                }
            }

            pub fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
                match self {
                    $(
                        NodePayload::$ty(inner) => inner.to_raw(),
                    )*
                }
            }
        }
    };
}



define_node_payload!(
    Arrow,
    Bond,
    Border,
    Constraint,
    Document,
    Fragment,
    Geometry,
    Graphic,
    Group,
    Node,
    ObjectTag,
    Page,
    ReactionScheme,
    ReactionStep,
    TextObject,
    TlcLane,
    TLCPlate,
    UnknownObject802B,
);


use dendron;

impl CdxFile {
    pub fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // root を payload で作成
        let root_payload = NodePayload::from_raw(raw.clone())?;
        let root:dendron::Node<NodePayload> = dendron::Node::new_tree(root_payload);

        // 編集 grant を取得
        let grant = root.tree().grant_hierarchy_edit().unwrap();

        // 再帰的に子要素を付ける
        Self::build_tree(&root, &grant, raw.children);

        Ok(CdxFile {tree: root.tree().clone() })
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CdxError> {
        let mut parser = crate::cdx_parse_impl::reader::RawCdxParser::new(std::io::Cursor::new(bytes));
        let raw_obj = parser.parse().map_err(|e| CdxError::Parse(format!("Failed to parse raw CDX object: {}", e)))?;
        Self::from_raw(raw_obj)
    }

    fn build_tree(
        parent: &dendron::Node<NodePayload>,
        grant: &dendron::HierarchyEditGrant<NodePayload>,
        raws: Vec<RawCdxObject>,
    ) {
        for raw_child in raws {
            match NodePayload::from_raw(raw_child.clone()) {
                Ok(payload) => {
                    let child = parent.create_as_last_child(grant, payload);
                    // Recursively build tree for children
                    Self::build_tree(&child, grant, raw_child.children);
                },
                Err(e) => {
                    // Log unknown tag but continue parsing
                    eprintln!("Warning: Skipping object due to: {}", e);
                }
            }
        }
    }
}
