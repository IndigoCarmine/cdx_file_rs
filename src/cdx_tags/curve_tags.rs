// =======================
// Curve Object
// =======================

pub const CDXOBJ_CURVE: u16 = 0x8008; // kCDXObj_Curve: Bézier curve.

// =======================
// Curve Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Curve Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle.

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Curve-Specific Properties
// =======================

pub const CDXPROP_CURVE_TYPE: u16 = 0x0A08; // Curve type (bit-encoded).

pub const CDXPROP_ARROWHEAD_SIZE: u16 = 0x0A20; // Arrowhead size.
pub const CDXPROP_CURVE_POINTS: u16 = 0x0A23; // Required: Bézier control points.
pub const CDXPROP_CURVE_POINTS3D: u16 = 0x0A2E; // 3D Bézier control points.
pub const CDXPROP_ARROWHEAD_TYPE: u16 = 0x0A2F; // Arrowhead type (enumerated).
pub const CDXPROP_ARROWHEAD_CENTER_SIZE: u16 = 0x0A30; // Arrowhead center size.
pub const CDXPROP_ARROWHEAD_WIDTH: u16 = 0x0A31; // Arrowhead half-width.

pub const CDXPROP_ARROW_ARROWHEAD_HEAD: u16 = 0x0A35; // Head arrowhead type (enumerated).
pub const CDXPROP_ARROW_ARROWHEAD_TAIL: u16 = 0x0A36; // Tail arrowhead type (enumerated).
pub const CDXPROP_FILL_TYPE: u16 = 0x0A37; // Fill type (enumerated).
pub const CDXPROP_CLOSED: u16 = 0x0A38; // Closed curve flag.
pub const CDXPROP_CURVE_SPACING: u16 = 0x0A39; // Spacing for doubled curves.
