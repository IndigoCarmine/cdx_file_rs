// =======================
// Node Object
// =======================

pub const CDXOBJ_NODE: u16 = 0x8004; // kCDXObj_Node: Fundamental chemical node (typically an atom).

// =======================
// Node Subobjects
// =======================

pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // Containing fragment.
pub const CDXOBJ_TEXT: u16 = 0x8006; // Atom label text.
pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.

// =======================
// Node Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Z-order (drawing order).
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_2D_POSITION: u16 = 0x0200; // 2D position (Y, X).
pub const CDXPROP_3D_POSITION: u16 = 0x0201; // 3D position (X, Y, Z).

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Node Identity / Type
// =======================

pub const CDXPROP_NODE_TYPE: u16 = 0x0400; // Node type (enumerated).
pub const CDXPROP_NODE_LABEL_DISPLAY: u16 = 0x0401; // Label display characteristics.
pub const CDXPROP_NODE_ELEMENT: u16 = 0x0402; // Atomic number.
pub const CDXPROP_ATOM_ELEMENT_LIST: u16 = 0x0403; // List of allowed elements.
pub const CDXPROP_ATOM_FORMULA: u16 = 0x0404; // Fragment formula (unknown connectivity).

// =======================
// Atomic Properties
// =======================

pub const CDXPROP_ATOM_ISOTOPE: u16 = 0x0420; // Absolute isotope.
pub const CDXPROP_ATOM_CHARGE: u16 = 0x0421; // Atomic charge.
pub const CDXPROP_ATOM_RADICAL: u16 = 0x0422; // Radical state.
pub const CDXPROP_ATOM_RESTRICT_FREE_SITES: u16 = 0x0423; // Allowed extra substituents.
pub const CDXPROP_ATOM_RESTRICT_IMPLICIT_H: u16 = 0x0424; // Disallow implicit hydrogens.
pub const CDXPROP_ATOM_RESTRICT_RING_BOND_COUNT: u16 = 0x0425; // Ring bond count restriction.
pub const CDXPROP_ATOM_RESTRICT_UNSATURATED_BONDS: u16 = 0x0426; // Unsaturation restriction.
pub const CDXPROP_ATOM_RESTRICT_RXN_CHANGE: u16 = 0x0427; // Reaction change restriction.
pub const CDXPROP_ATOM_RESTRICT_RXN_STEREO: u16 = 0x0428; // Reaction stereochemistry change.
pub const CDXPROP_ATOM_ABNORMAL_VALENCE: u16 = 0x0429; // Allow abnormal valence.

pub const CDXPROP_ATOM_NUM_HYDROGENS: u16 = 0x042B; // Explicit hydrogen count.
pub const CDXPROP_ATOM_H_DOT: u16 = 0x042E; // Implicit wedged hydrogen.
pub const CDXPROP_ATOM_H_DASH: u16 = 0x042F; // Implicit hashed hydrogen.

pub const CDXPROP_ATOM_GEOMETRY: u16 = 0x0430; // Bond geometry (enumerated).
pub const CDXPROP_ATOM_BOND_ORDERING: u16 = 0x0431; // Bond ordering for stereochemistry.

pub const CDXPROP_NODE_ATTACHMENTS: u16 = 0x0432; // Required for multi-/variable-attachment nodes.

pub const CDXPROP_ATOM_GENERIC_NICKNAME: u16 = 0x0433; // Generic nickname.
pub const CDXPROP_ATOM_ALT_GROUP_ID: u16 = 0x0434; // Alternative group ID.

pub const CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_UP_TO: u16 = 0x0435; // Max substituents.
pub const CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_EXACTLY: u16 = 0x0436; // Exact substituent count.

pub const CDXPROP_ATOM_CIP_STEREOCHEMISTRY: u16 = 0x0437; // Cahn–Ingold–Prelog stereochemistry.
pub const CDXPROP_ATOM_TRANSLATION: u16 = 0x0438; // Node generality restriction.
pub const CDXPROP_ATOM_NUMBER: u16 = 0x0439; // Atom number label.

pub const CDXPROP_ATOM_SHOW_QUERY: u16 = 0x043A; // Show query indicator.
pub const CDXPROP_ATOM_SHOW_STEREO: u16 = 0x043B; // Show stereochemistry indicator.
pub const CDXPROP_ATOM_SHOW_ATOM_NUMBER: u16 = 0x043C; // Show atom number.

pub const CDXPROP_ATOM_LINK_COUNT_LOW: u16 = 0x043D; // Link node repeat low bound.
pub const CDXPROP_ATOM_LINK_COUNT_HIGH: u16 = 0x043E; // Link node repeat high bound.

pub const CDXPROP_ATOM_ISOTOPIC_ABUNDANCE: u16 = 0x043F; // Isotopic abundance.
pub const CDXPROP_ATOM_EXTERNAL_CONNECTION_TYPE: u16 = 0x0440; // External connection type.
pub const CDXPROP_ATOM_GENERIC_LIST: u16 = 0x0441; // Generic nickname list.

pub const CDXPROP_ATOM_SHOW_ENHANCED_STEREO: u16 = 0x0445; // Show enhanced stereo indicator.
pub const CDXPROP_ATOM_ENHANCED_STEREO_TYPE: u16 = 0x0446; // Enhanced stereo type.
pub const CDXPROP_ATOM_ENHANCED_STEREO_GROUP_NUM: u16 = 0x0447; // Enhanced stereo group number.

// =======================
// Styling Overrides
// =======================

pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.
pub const CDXPROP_LABEL_STYLE: u16 = 0x080A; // Atom label font style (unused).
pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A; // Atom label font family.
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C; // Atom label font size.
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E; // Atom label font face.
