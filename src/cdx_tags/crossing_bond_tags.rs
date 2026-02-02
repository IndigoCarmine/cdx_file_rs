// =======================
// Crossing Bond Object
// =======================

pub const CDXOBJ_CROSSING_BOND: u16 = 0x8019; // kCDXObj_CrossingBond: Bond connecting bracketed group to external node.

// =======================
// Crossing Bond Subobjects
// =======================

// (none)

// =======================
// Crossing Bond Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_CROSSING_BOND_ID: u16 = 0x0A2C; // ID of the bond that crosses the bracket.
pub const CDXPROP_CROSSING_BOND_BEGIN_INSIDE: u16 = 0x0A2D; // Begin node is inside bracket flag.
