// =======================
// Graphic Object
// =======================

pub const CDXOBJ_GRAPHIC: u16 = 0x8007; // kCDXObj_Graphic: Non-chemical graphic primitive.

// =======================
// Graphic Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.
pub const CDXPROP_REPRESENTS_PROPERTY: u16 = 0x000E; // Indicates chemical meaning in another object.

// =======================
// Graphic Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.
pub const CDXPROP_SUPERSEDED_BY: u16 = 0x0012; // ID of superseding object.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Required: bounding rectangle.
pub const CDXPROP_3D_HEAD: u16 = 0x0207; // 3D head position.
pub const CDXPROP_3D_TAIL: u16 = 0x0208; // 3D tail position.

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Styling Properties
// =======================

pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806; // Bold width.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.

pub const CDXPROP_CAPTION_STYLE: u16 = 0x080B; // Caption font style (unused).
pub const CDXPROP_CAPTION_STYLE_FONT: u16 = 0x081B; // Caption font family.
pub const CDXPROP_CAPTION_STYLE_SIZE: u16 = 0x081D; // Caption font size.
pub const CDXPROP_CAPTION_STYLE_FACE: u16 = 0x081F; // Caption font face.

// =======================
// Graphic-Specific Properties
// =======================

pub const CDXPROP_GRAPHIC_TYPE: u16 = 0x0A00; // Type of graphical object (enumerated).
pub const CDXPROP_LINE_TYPE: u16 = 0x0A01; // Line type (enumerated).
pub const CDXPROP_ARROW_TYPE: u16 = 0x0A02; // Arrow type (enumerated).
pub const CDXPROP_RECTANGLE_TYPE: u16 = 0x0A03; // Rectangle type (enumerated).
pub const CDXPROP_OVAL_TYPE: u16 = 0x0A04; // Oval/ellipse type (enumerated).
pub const CDXPROP_ORBITAL_TYPE: u16 = 0x0A05; // Orbital type (enumerated).
pub const CDXPROP_BRACKET_TYPE: u16 = 0x0A06; // Bracket type (enumerated).
pub const CDXPROP_SYMBOL_TYPE: u16 = 0x0A07; // Symbol type (enumerated).

pub const CDXPROP_ARROWHEAD_SIZE: u16 = 0x0A20; // Arrowhead size.
pub const CDXPROP_ARC_ANGULAR_SIZE: u16 = 0x0A21; // Arc angular size (degrees * 10).
pub const CDXPROP_BRACKET_LIP_SIZE: u16 = 0x0A22; // Bracket lip size.

pub const CDXPROP_BRACKET_USAGE: u16 = 0x0A24; // Bracket chemical meaning (SRU, mer, etc.) (enumerated).
pub const CDXPROP_POLYMER_REPEAT_PATTERN: u16 = 0x0A25; // Head-to-tail connectivity (enumerated).
pub const CDXPROP_POLYMER_FLIP_TYPE: u16 = 0x0A26; // Flip state (enumerated).

pub const CDXPROP_CORNER_RADIUS: u16 = 0x0A3C; // Rounded rectangle corner radius.
pub const CDXPROP_FRAME_TYPE: u16 = 0x0A3D; // Frame type (enumerated).
