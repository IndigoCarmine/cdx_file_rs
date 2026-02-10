use dendron::tree::Tree;
use std::collections::HashSet;

use crate::error::CdxError;

pub use super::annotation::Annotation;
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
pub use super::seg_datum::SegDatum;
pub use super::segcomponent::SegComponent;
pub use super::sequence::Sequence;
pub use super::spectrum::Spectrum;
pub use super::splitter::Splitter;
pub use super::stoichiometrygrid::StoichiometryGrid;
pub use super::table::Table;
pub use super::template_grid::TemplateGrid;
pub use super::text::TextObject;
pub use super::tlc_lane::TlcLane;
pub use super::tlc_plate::TLCPlate;
pub use super::tlc_spot::TLCSpot;
pub use super::unknown::*;
use crate::cdx_parse_impl::tagged_object::TaggedObject;

macro_rules! extract_id {
    ($payload:expr) => {
        match $payload {
            NodePayload::Arrow(n) => Some(n.id),
            NodePayload::Bond(n) => Some(n.id),
            NodePayload::Border(n) => Some(n.id),
            NodePayload::BracketAttachment(n) => Some(n.id),
            NodePayload::BracketedGroup(n) => Some(n.id),
            NodePayload::ChemicalProperty(n) => Some(n.id),
            NodePayload::Constraint(n) => Some(n.id),
            NodePayload::CrossReference(n) => Some(n.id),
            NodePayload::CrossingBond(n) => Some(n.id),
            NodePayload::Curve(n) => Some(n.id),
            NodePayload::Document(n) => Some(n.id),
            NodePayload::EmbeddedObject(n) => Some(n.id),
            NodePayload::Fragment(n) => Some(n.id),
            NodePayload::Geometry(n) => Some(n.id),
            NodePayload::Graphic(n) => Some(n.id),
            NodePayload::Group(n) => Some(n.id),
            NodePayload::NamedAlternativeGroup(n) => Some(n.id),
            NodePayload::Node(n) => Some(n.id),
            NodePayload::ObjectTag(n) => Some(n.id),
            NodePayload::Page(n) => Some(n.id),
            NodePayload::ReactionScheme(n) => Some(n.id),
            NodePayload::ReactionStep(n) => Some(n.id),
            NodePayload::RegistryNumber(n) => Some(n.id),
            NodePayload::Sequence(n) => Some(n.id),
            NodePayload::Spectrum(n) => Some(n.id),
            NodePayload::Splitter(n) => Some(n.id),
            NodePayload::Table(n) => Some(n.id),
            NodePayload::TemplateGrid(n) => Some(n.id),
            NodePayload::TextObject(n) => Some(n.id),
            NodePayload::TlcLane(n) => Some(n.id),
            NodePayload::TLCPlate(n) => Some(n.id),
            NodePayload::TLCSpot(n) => Some(n.id),
            NodePayload::Annotation(n) => Some(n.id),
            NodePayload::StoichiometryGrid(n) => Some(n.id),
            NodePayload::SegComponent(n) => Some(n.id),
            NodePayload::SegDatum(n) => Some(n.id),
            NodePayload::UnknownObject802B(n) => Some(n.id),
            NodePayload::UnknownObject801D(n) => Some(n.id),
            NodePayload::UnknownObject801E(n) => Some(n.id),
            NodePayload::UnknownObject801F(n) => Some(n.id),
        }
    };
}

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

    /// Get the first Page object from the document
    pub fn get_first_page(&self) -> Option<Page> {
        let root = self.tree.root();
        for child in root.children() {
            if let NodePayload::Page(page) = &*child.borrow_data() {
                return Some(page.clone());
            }
        }
        None
    }

    /// Generate the next unique ID by finding the maximum ID in the tree and adding 1
    pub fn next_id(&self) -> u32 {
        let mut max_id = 0u32;

        // Traverse all nodes to find the maximum ID
        let mut queue = vec![self.tree.root()];
        while let Some(node) = queue.pop() {
            let node_id = extract_id!(&*node.borrow_data()).unwrap_or(0);
            max_id = max_id.max(node_id);

            for child in node.children() {
                queue.push(child);
            }
        }

        max_id + 1
    }

    /// Find a node by its ID
    pub fn find_node_by_id(&self, target_id: u32) -> Option<dendron::Node<NodePayload>> {
        let mut queue = vec![self.tree.root()];
        while let Some(node) = queue.pop() {
            let node_id = extract_id!(&*node.borrow_data()).unwrap_or(0);

            if node_id == target_id {
                return Some(node);
            }

            for child in node.children() {
                queue.push(child);
            }
        }
        None
    }

    /// Find the Fragment containing a given node ID
    pub fn find_fragment_for_node(&self, target_id: u32) -> Option<dendron::Node<NodePayload>> {
        if let Some(node) = self.find_node_by_id(target_id) {
            let mut current = node;
            loop {
                if let Some(parent) = current.parent() {
                    if matches!(&*parent.borrow_data(), NodePayload::Fragment(_)) {
                        return Some(parent);
                    }
                    current = parent;
                } else {
                    break;
                }
            }
        }
        None
    }

    /// Find the first Fragment in the document tree
    pub fn find_first_fragment(&self) -> Option<dendron::Node<NodePayload>> {
        let mut queue = vec![self.tree.root()];
        while let Some(node) = queue.pop() {
            if matches!(&*node.borrow_data(), NodePayload::Fragment(_)) {
                return Some(node);
            }
            for child in node.children() {
                queue.push(child);
            }
        }
        None
    }

    /// Extract the minimal subtree containing all selected node IDs
    /// This function finds the smallest tree that includes all selected nodes
    /// by finding their lowest common ancestor and extracting that subtree
    pub fn extract_selected_subtree(
        &self,
        selected_ids: &HashSet<u32>,
    ) -> Result<CdxFile, CdxError> {
        if selected_ids.is_empty() {
            return Err(CdxError::Parse(
                "No nodes selected for extraction".to_string(),
            ));
        }

        // Find all nodes that should be included (selected nodes + ancestors)
        let mut nodes_to_include = HashSet::new();
        let mut queue = Vec::new();

        // Start from root and traverse to find and mark selected nodes
        let root = self.tree.root();
        queue.push(root.clone());

        // First pass: collect all selected nodes and their ancestors
        while let Some(node) = queue.pop() {
            let node_id = extract_id!(&*node.borrow_data());

            if let Some(id) = node_id {
                if selected_ids.contains(&id) {
                    nodes_to_include.insert(id);

                    // Mark all ancestors as needed
                    let mut current = node.clone();
                    while let Some(parent) = current.parent() {
                        let parent_id = extract_id!(&*parent.borrow_data());
                        if let Some(pid) = parent_id {
                            nodes_to_include.insert(pid);
                        }
                        current = parent;
                    }
                }
            }

            // Add children to queue
            for child in node.children() {
                queue.push(child);
            }
        }

        // Second pass: build new tree with only included nodes
        let new_root = self.tree.root();
        let new_root_data = (*new_root.borrow_data()).clone();
        let new_tree_root = dendron::Node::new_tree(new_root_data);
        let grant = new_tree_root
            .tree()
            .grant_hierarchy_edit()
            .map_err(|_| CdxError::Parse("Failed to get tree edit grant".to_string()))?;

        self.copy_subtree_filtered(&new_root, &new_tree_root, &grant, &nodes_to_include);

        Ok(CdxFile {
            tree: new_tree_root.tree().clone(),
        })
    }

    /// Helper function to recursively copy subtree, including only selected nodes
    fn copy_subtree_filtered(
        &self,
        source_node: &dendron::Node<NodePayload>,
        target_parent: &dendron::Node<NodePayload>,
        grant: &dendron::HierarchyEditGrant<NodePayload>,
        nodes_to_include: &HashSet<u32>,
    ) {
        for child in source_node.children() {
            let child_id = extract_id!(&*child.borrow_data()).unwrap_or(0);

            if nodes_to_include.contains(&child_id) {
                let child_data = (*child.borrow_data()).clone();
                let new_child = target_parent.create_as_last_child(grant, child_data);
                self.copy_subtree_filtered(&child, &new_child, grant, nodes_to_include);
            }
        }
    }
}

macro_rules! define_node_payload {
    ( $( $ty:ident ),* $(,)? ) => {
        #[derive(Debug, Clone)]
        pub enum NodePayload {
            $(
                $ty($ty),
            )*
        }
        impl NodePayload {
            pub fn tag(&self) -> u16 {
                match self {
                    $(
                        NodePayload::$ty(_) => <$ty as TaggedObject>::TAG,
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
    UnknownObject802B,
    UnknownObject801D,
    SegComponent,
    SegDatum,
    UnknownObject801E,
    UnknownObject801F,
    Annotation,
    StoichiometryGrid,
);
