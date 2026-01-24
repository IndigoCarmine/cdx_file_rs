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
pub const ARROW: u16 = 0x8021;
pub const STOICHIOMETRY_GRID: u16 = 0x8022;
pub const SG_COMPONENT: u16 = 0x8023;
pub const SG_DATUM: u16 = 0x8024;

// Properties
pub const POSITION: u16 = 0x0200;
pub const EXTENT: u16 = 0x0202;
pub const BOUNDING_BOX: u16 = 0x0204;
pub const TAIL_3D: u16 = 0x0208;
pub const HEAD_3D: u16 = 0x0207;

pub const COLOR_TABLE: u16 = 0x0300;
pub const FG_COLOR: u16 = 0x0301; // kCDXProp_ForegroundColor
pub const BG_COLOR: u16 = 0x0302; // kCDXProp_BackgroundColor

pub const ELEMENT: u16 = 0x0402;
pub const BOND_ORDER: u16 = 0x0600;
pub const BOND_DOUBLE_POSITION: u16 = 0x0601;
pub const BOND_BEGIN: u16 = 0x0604;
pub const BOND_END: u16 = 0x0605;

pub const TEXT_STRING: u16 = 0x0700;
pub const TEXT_STRING_ALT: u16 = 0x0709;

pub const FONT_TABLE: u16 = 0x0100;
