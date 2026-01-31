// =======================
// TLC Spot Object
// =======================

pub const CDXOBJ_TLC_SPOT: u16 = 0x8025; // kCDXObj_TLCSpot: Individual spot on a TLC lane.

// =======================
// TLC Spot Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.

// =======================
// TLC Spot Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_WIDTH: u16 = 0x0812; // Spot width (unrotated reference frame).
pub const CDXPROP_HEIGHT: u16 = 0x0813; // Spot height (unrotated reference frame).

pub const CDXPROP_CURVE_TYPE: u16 = 0x0A08; // Curve type (bit-encoded).

pub const CDXPROP_TLC_RF: u16 = 0x0AB0; // Retention factor (Rf).
pub const CDXPROP_TLC_TAIL: u16 = 0x0AB1; // Tail length.
pub const CDXPROP_TLC_SHOW_RF: u16 = 0x0AB2; // Display Rf value.
