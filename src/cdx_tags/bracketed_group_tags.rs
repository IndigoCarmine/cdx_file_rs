// =======================
// Bracketed Group Object
// =======================

pub const CDXOBJ_BRACKETED_GROUP: u16 = 0x8017; // kCDXObj_BracketedGroup: Collection surrounded by brackets.

// =======================
// Bracketed Group Subobjects
// =======================

pub const CDXOBJ_BRACKET_ATTACHMENT: u16 = 0x8018; // kCDXObj_BracketAttachment: Linkage to outside object.

// =======================
// Bracketed Group Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BRACKET_USAGE: u16 = 0x0A24; // Bracket chemical meaning (SRU, mer, etc.) (enumerated).
pub const CDXPROP_POLYMER_REPEAT_PATTERN: u16 = 0x0A25; // Head-to-tail connectivity (enumerated).
pub const CDXPROP_POLYMER_FLIP_TYPE: u16 = 0x0A26; // Flip state (enumerated).
pub const CDXPROP_BRACKETED_OBJECTS: u16 = 0x0A27; // Object IDs contained in group.
pub const CDXPROP_BRACKET_REPEAT_COUNT: u16 = 0x0A28; // Repeat count.
pub const CDXPROP_BRACKET_COMPONENT_ORDER: u16 = 0x0A29; // Component order.
pub const CDXPROP_BRACKET_SRU_LABEL: u16 = 0x0A2A; // SRU label text.
