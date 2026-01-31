// =======================
// Fragment Object
// =======================

pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // kCDXObj_Fragment: Chemically meaningful collection of nodes and bonds.

// =======================
// Fragment Subobjects
// =======================

pub const CDXOBJ_NODE: u16 = 0x8004; // kCDXObj_Node: Atomic node.
pub const CDXOBJ_BOND: u16 = 0x8005; // kCDXObj_Bond: Connection between two nodes.
pub const CDXOBJ_GRAPHIC: u16 = 0x8007; // kCDXObj_Graphic: Non-chemical graphic primitive.
pub const CDXOBJ_CURVE: u16 = 0x8008; // kCDXObj_Curve: Bézier curve.
pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Fragment Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle of the fragment.

pub const CDXPROP_MOLE_RACEMIC: u16 = 0x0500; // Racemic mixture flag.
pub const CDXPROP_MOLE_ABSOLUTE: u16 = 0x0501; // Known absolute configuration.
pub const CDXPROP_MOLE_RELATIVE: u16 = 0x0502; // Known relative stereochemistry only.

pub const CDXPROP_MOLE_FORMULA: u16 = 0x0503; // Molecular formula.
pub const CDXPROP_MOLE_WEIGHT: u16 = 0x0504; // Average molecular weight.

pub const CDXPROP_FRAG_CONNECTION_ORDER: u16 = 0x0505; // Ordered list of fragment attachment points.
