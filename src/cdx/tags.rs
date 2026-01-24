// Objects
pub const DOCUMENT: u16 = 0x8000;
pub const PAGE: u16 = 0x8001;
pub const GROUP: u16 = 0x8002;
pub const FRAGMENT: u16 = 0x8003;
pub const NODE: u16 = 0x8004;
pub const BOND: u16 = 0x8005;
pub const TEXT: u16 = 0x8006;
pub const GRAPHIC: u16 = 0x8007;
pub const OBJECT_TAG: u16 = 0x8011;
pub const GEOMETRY: u16 = 0x8021;
pub const CONSTRAINT: u16 = 0x8022;
pub const TLC_PLATE: u16 = 0x8023;
pub const TLC_LANE: u16 = 0x8024;
pub const TLC_SPOT: u16 = 0x8025;
pub const ARROW: u16 = 0x8027;

// Legacy/Undocumented tags (not found in official documentation)
// These may be ChemDraw-specific extensions or deprecated tags
pub const STOICHIOMETRY_GRID: u16 = 0x8022; // Alias for CONSTRAINT? Used in render.rs/export.rs
pub const SG_COMPONENT: u16 = 0x8023; // Alias for TLC_PLATE? Used in render.rs/export.rs  
pub const SG_DATUM: u16 = 0x8024; // Alias for TLC_LANE? Used in render.rs/export.rs

// Properties
pub const POSITION: u16 = 0x0200;
pub const EXTENT: u16 = 0x0202;
pub const BOUNDING_BOX: u16 = 0x0204;
pub const TAIL_3D: u16 = 0x0208;
pub const HEAD_3D: u16 = 0x0207;

pub const COLOR_TABLE: u16 = 0x0300;
pub const FG_COLOR: u16 = 0x0301; // kCDXProp_ForegroundColor (UINT16)
pub const BG_COLOR: u16 = 0x0302; // kCDXProp_BackgroundColor (INT16)

// Common properties
pub const VISIBLE: u16 = 0x0011; // kCDXProp_Visible (CDXBoolean)

// Node/Atom properties
pub const NODE_TYPE: u16 = 0x0400; // kCDXProp_Node_Type (INT16)
pub const ELEMENT: u16 = 0x0402; // kCDXProp_Node_Element (INT16)
pub const ATOM_CHARGE: u16 = 0x0421; // kCDXProp_Atom_Charge (INT8)
pub const ATOM_NUM_HYDROGENS: u16 = 0x042B; // kCDXProp_Atom_NumHydrogens (UINT16)

// Bond properties
pub const BOND_ORDER: u16 = 0x0600; // kCDXProp_Bond_Order (INT16)
pub const BOND_DOUBLE_POSITION: u16 = 0x0603; // kCDXProp_Bond_DoublePosition (INT16)
pub const BOND_BEGIN: u16 = 0x0604; // kCDXProp_Bond_Begin (UINT32)
pub const BOND_END: u16 = 0x0605; // kCDXProp_Bond_End (UINT32)

pub const TEXT_STRING: u16 = 0x0700;
pub const TEXT_STRING_ALT: u16 = 0x0709;

pub const FONT_TABLE: u16 = 0x0100;
