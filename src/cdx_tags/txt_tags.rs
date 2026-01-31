// =======================
// Text Object
// =======================

pub const CDXOBJ_TEXT: u16 = 0x8006; // kCDXObj_Text: Arbitrary styled text block.

// =======================
// Text Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.
// Subobject "s" (styled text run) has no CDX object constant.

// =======================
// Text Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A;
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F;
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010;
pub const CDXPROP_VISIBLE: u16 = 0x0011;

pub const CDXPROP_2D_POSITION: u16 = 0x0200;
pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204;
pub const CDXPROP_ROTATION_ANGLE: u16 = 0x0205;

pub const CDXPROP_TEXT: u16 = 0x0700; // Required for Text objects.
pub const CDXPROP_JUSTIFICATION: u16 = 0x0701;
pub const CDXPROP_LINE_HEIGHT: u16 = 0x0702;
pub const CDXPROP_WORD_WRAP_WIDTH: u16 = 0x0703;
pub const CDXPROP_LINE_STARTS: u16 = 0x0704;
pub const CDXPROP_LABEL_ALIGNMENT: u16 = 0x0705;
pub const CDXPROP_LABEL_LINE_HEIGHT: u16 = 0x0706;
pub const CDXPROP_CAPTION_LINE_HEIGHT: u16 = 0x0707;
pub const CDXPROP_INTERPRET_CHEMICALLY: u16 = 0x0708;

pub const CDXPROP_LABEL_STYLE: u16 = 0x080A;
pub const CDXPROP_CAPTION_STYLE: u16 = 0x080B;
pub const CDXPROP_CAPTION_JUSTIFICATION: u16 = 0x080C;

pub const CDXPROP_LABEL_FONT: u16 = 0x081A;
pub const CDXPROP_CAPTION_FONT: u16 = 0x081B;
pub const CDXPROP_LABEL_SIZE: u16 = 0x081C;
pub const CDXPROP_CAPTION_SIZE: u16 = 0x081D;
pub const CDXPROP_LABEL_FACE: u16 = 0x081E;
pub const CDXPROP_CAPTION_FACE: u16 = 0x081F;
pub const CDXPROP_LABEL_COLOR: u16 = 0x0820;
pub const CDXPROP_CAPTION_COLOR: u16 = 0x0821;
pub const CDXPROP_LABEL_JUSTIFICATION: u16 = 0x0823;
