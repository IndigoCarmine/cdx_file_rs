// =======================
// TLC Lane Object
// =======================

// In the document, this is 0x8024. But actual TLCLane objects seem to use 0x801E (probably?)
// pub const CDXOBJ_TLC_LANE: u16 = 0x8024; // kCDXObj_TLCLane: Lane within a TLC plate.
pub const CDXOBJ_TLC_LANE: u16 = 0x801E; // kCDXObj_TLCLane: Lane within a TLC plate.

// =======================
// TLC Lane Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.
pub const CDXOBJ_TLC_SPOT: u16 = 0x8025; // Single TLC spot.

// =======================
// TLC Lane Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.
