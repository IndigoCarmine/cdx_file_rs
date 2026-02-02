// =======================
// Bracket Attachment Object
// =======================

pub const CDXOBJ_BRACKET_ATTACHMENT: u16 = 0x8018; // kCDXObj_BracketAttachment: Linkage to object outside bracket.

// =======================
// Bracket Attachment Subobjects
// =======================

pub const CDXOBJ_CROSSING_BOND: u16 = 0x8019; // kCDXObj_CrossingBond: Bond crossing bracket boundary.

// =======================
// Bracket Attachment Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BRACKET_GRAPHIC_ID: u16 = 0x0A2B; // ID of associated graphic (bracket/brace/parenthesis).
