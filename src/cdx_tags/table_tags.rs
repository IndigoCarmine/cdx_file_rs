// =======================
// Table Object Tags
// =======================


// In the document, this is 0x8016. But actual Table objects seem to use 0x802B (probably?)
/// Table object (0x8016)
pub const CDXOBJ_TABLE: u16 = 0x8016;
// pub const CDXOBJ_TABLE: u16 = 0x8023;

// =======================
// Property Tags
// =======================

// Core properties
pub const CDXPROP_Z_ORDER: u16 = 0x000A;
pub const CDXPROP_VISIBLE: u16 = 0x0011;
pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204;

// Color properties
pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301;
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302;

// Line/styling properties
pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806;
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807;
pub const CDXPROP_MARGIN_WIDTH: u16 = 0x0808;

// Label font properties
pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A;
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C;
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E;
