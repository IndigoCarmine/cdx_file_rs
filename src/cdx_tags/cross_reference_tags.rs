// =======================
// Cross-Reference Object
// =======================

pub const CDXOBJ_CROSS_REFERENCE: u16 = 0x8014; // kCDXObj_CrossReference: Link to Sequence object.

// =======================
// Cross-Reference Subobjects
// =======================

// (none)

// =======================
// Cross-Reference Properties
// =======================

pub const CDXPROP_CROSS_REFERENCE_CONTAINER: u16 = 0x0F00; // External container object.
pub const CDXPROP_CROSS_REFERENCE_DOCUMENT: u16 = 0x0F01; // External document path.
pub const CDXPROP_CROSS_REFERENCE_IDENTIFIER: u16 = 0x0F02; // Required: unique cross-reference ID.
pub const CDXPROP_CROSS_REFERENCE_SEQUENCE: u16 = 0x0F03; // Required: target sequence identifier.
