use dendron::tree::Tree;

use crate::error::CdxError;

pub use super::arrow::Arrow;
pub use super::bond::Bond;
pub use super::border::Border;
pub use super::bracket_attachment::BracketAttachment;
pub use super::bracketed_group::BracketedGroup;
pub use super::chemical_property::ChemicalProperty;
pub use super::color_table::ColorTable;
pub use super::constraint::Constraint;
pub use super::cross_reference::CrossReference;
pub use super::crossing_bond::CrossingBond;
pub use super::curve::Curve;
pub use super::document::Document;
pub use super::embedded_object::EmbeddedObject;
pub use super::fragment::Fragment;
pub use super::geometry::Geometry;
pub use super::graphic::Graphic;
pub use super::group::Group;
pub use super::named_alternative_group::NamedAlternativeGroup;
pub use super::node::Node;
pub use super::object_tag::ObjectTag;
pub use super::page::Page;
pub use super::reaction_scheme::ReactionScheme;
pub use super::reaction_step::ReactionStep;
pub use super::registry_number::RegistryNumber;
pub use super::sequence::Sequence;
pub use super::spectrum::Spectrum;
pub use super::splitter::Splitter;
pub use super::table::Table;
pub use super::template_grid::TemplateGrid;
pub use super::text::TextObject;
pub use super::tlc_lane::TlcLane;
pub use super::tlc_plate::TLCPlate;
pub use super::tlc_spot::TLCSpot;
pub use super::unknown_802b::UnknownObject802B;

pub struct CdxFile {
    pub tree: Tree<NodePayload>,
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
    Arrow,
    Bond,
    Border,
    BracketAttachment,
    BracketedGroup,
    ChemicalProperty,
    ColorTable,
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
    UnknownObject802B,
);
