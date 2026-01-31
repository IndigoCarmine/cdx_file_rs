// =======================
// Geometry Object
// =======================

pub const CDXOBJ_GEOMETRY: u16 = 0x8021; // kCDXObj_Geometry: A geometrical relationship between one or more objects.

// =======================
// Geometry Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // The smallest rectangle that encloses the graphical representation of the object.
pub const CDXPROP_2D_POSITION: u16 = 0x0200; // The 2D location of an object.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // The object is visible if non-zero.
pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front ordering index in 2D drawing.
pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // The foreground color of an object.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // The background color of an object.
pub const CDXPROP_ROTATION_ANGLE: u16 = 0x0205; // The angular orientation of an object in degrees * 65536.

