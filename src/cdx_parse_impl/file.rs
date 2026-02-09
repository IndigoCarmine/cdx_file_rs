use crate::cdx::file::CdxFile;
use crate::cdx::file::*;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

#[macro_export]
/// Macro to implement conversion methods for the `NodePayload` enum.
///
/// This macro generates two methods for the `NodePayload` implementation:
/// - `from_raw`: Converts a `RawCdxObject` into a `NodePayload` variant based on its tag.
///   Returns an error if the tag does not match any known variant.
/// - `to_raw`: Converts a `NodePayload` variant back into a `RawCdxObject`.
///
/// # Parameters
/// - `$ty`: A list of types that implement the `TaggedObject` trait and correspond to variants of `NodePayload`.
///
/// # Example
/// ```ignore
/// define_node_payload!(TypeA, TypeB, TypeC);
/// ```
///
/// This will implement `from_raw` and `to_raw` for `NodePayload` with variants `TypeA`, `TypeB`, and `TypeC`.
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
                    _ => Err(CdxError::Parse(format!("found unknown tag=0x{:x}", raw.tag))),
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
    BracketAttachment,
    BracketedGroup,
    ChemicalProperty,
    Constraint,
    CrossReference,
    CrossingBond,
    Curve,
    Document,
    EmbeddedObject,
    Fragment,
    Geometry,
    Graphic,
    Group,
    NamedAlternativeGroup,
    Node,
    ObjectTag,
    Page,
    ReactionScheme,
    ReactionStep,
    RegistryNumber,
    Sequence,
    Spectrum,
    Splitter,
    Table,
    TemplateGrid,
    TextObject,
    TlcLane,
    TLCPlate,
    TLCSpot,
    Annotation,
    StoichiometryGrid,
    SegDatum,
    SegComponent,
    UnknownObject802B,
    UnknownObject801D,
    UnknownObject801E,
    UnknownObject801F,
);

use dendron;

impl CdxFile {
    pub fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // root を payload で作成
        let root_payload = NodePayload::from_raw(raw.clone())?;
        let root: dendron::Node<NodePayload> = dendron::Node::new_tree(root_payload);

        // 編集 grant を取得
        let grant = root.tree().grant_hierarchy_edit().unwrap();

        // 再帰的に子要素を付ける
        Self::build_tree(&root, &grant, raw.children);

        Ok(CdxFile {
            tree: root.tree().clone(),
        })
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CdxError> {
        let mut parser =
            crate::cdx_parse_impl::reader::RawCdxParser::new(std::io::Cursor::new(bytes));
        let raw_obj = parser
            .parse()
            .map_err(|e| CdxError::Parse(format!("Failed to parse raw CDX object: {}", e)))?;
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
                }
                Err(e) => {
                    // Log unknown tag but continue parsing
                    eprintln!("Warning: Skipping object due to: {}", e);
                }
            }
        }
    }
}
