use crate::cdx::values::{CDXString, Point2d, Point3d};

use serde::{Deserialize, Serialize};
/// Node Object: Basic building block representing atoms or attachment points
/// A Node object is the fundamental chemical object, typically representing a single atom.
/// Most Node objects have no required properties or objects.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: u32,

    // Common properties
    pub z_order: Option<i16>,
    pub ignore_warnings: Option<bool>,
    pub chemical_warning: Option<CDXString>,
    pub visible: Option<bool>,

    // Position
    pub position_2d: Option<Point2d>,
    pub position_3d: Option<Point3d>,

    // Color
    pub foreground_color: Option<u16>,
    pub background_color: Option<i16>,

    // Node identity
    pub node_type: Option<i16>,
    pub label_display: Option<i8>,
    pub element: Option<i16>,

    // Atomic properties
    pub isotope: Option<i16>,
    pub charge: Option<i8>,
    pub radical: Option<u8>,
    pub restrict_free_sites: Option<u8>,
    pub restrict_implicit_h: Option<bool>,
    pub restrict_ring_bond_count: Option<i8>,
    pub restrict_unsaturated_bonds: Option<i8>,
    pub restrict_rxn_change: Option<bool>,
    pub restrict_rxn_stereo: Option<i8>,
    pub abnormal_valence: Option<bool>,
    pub num_hydrogens: Option<u16>,
    pub h_dot: Option<bool>,
    pub h_dash: Option<bool>,
    pub geometry: Option<i8>,
    pub bond_ordering: Option<Vec<u32>>,
    pub attachments: Option<Vec<u32>>,

    // Generic/Alternative properties
    pub generic_nickname: Option<CDXString>,
    pub alt_group_id: Option<u32>,
    pub restrict_substituents_up_to: Option<u8>,
    pub restrict_substituents_exactly: Option<u8>,

    // Stereochemistry / Query indicators
    pub cip_stereochemistry: Option<i8>,
    pub atom_translation: Option<i8>,
    pub atom_number: Option<u16>,
    pub show_query: Option<bool>,
    pub show_stereo: Option<bool>,
    pub show_atom_number: Option<bool>,

    // Link node properties
    pub link_count_low: Option<i16>,
    pub link_count_high: Option<i16>,
    pub isotopic_abundance: Option<f64>,
    pub external_connection_type: Option<i8>,
    pub generic_list: Option<CDXString>,

    // Enhanced stereo
    pub show_enhanced_stereo: Option<bool>,
    pub enhanced_stereo_type: Option<i8>,
    pub enhanced_stereo_group_num: Option<u16>,

    // Styling
    pub line_width: Option<f64>,
    pub label_font: Option<i16>,
    pub label_size: Option<i16>,
    pub label_face: Option<i16>,
}

impl Node {
    /// Create a new Node with just an ID
    pub fn new(id: u32) -> Self {
        Node {
            id,
            z_order: None,
            ignore_warnings: None,
            chemical_warning: None,
            visible: None,
            position_2d: None,
            position_3d: None,
            foreground_color: None,
            background_color: None,
            node_type: None,
            label_display: None,
            element: None,
            isotope: None,
            charge: None,
            radical: None,
            restrict_free_sites: None,
            restrict_implicit_h: None,
            restrict_ring_bond_count: None,
            restrict_unsaturated_bonds: None,
            restrict_rxn_change: None,
            restrict_rxn_stereo: None,
            abnormal_valence: None,
            num_hydrogens: None,
            h_dot: None,
            h_dash: None,
            geometry: None,
            bond_ordering: None,
            attachments: None,
            generic_nickname: None,
            alt_group_id: None,
            restrict_substituents_up_to: None,
            restrict_substituents_exactly: None,
            cip_stereochemistry: None,
            atom_translation: None,
            atom_number: None,
            show_query: None,
            show_stereo: None,
            show_atom_number: None,
            link_count_low: None,
            link_count_high: None,
            isotopic_abundance: None,
            external_connection_type: None,
            generic_list: None,
            show_enhanced_stereo: None,
            enhanced_stereo_type: None,
            enhanced_stereo_group_num: None,
            line_width: None,
            label_font: None,
            label_size: None,
            label_face: None,
        }
    }
}
