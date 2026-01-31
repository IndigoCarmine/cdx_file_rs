// =======================
// Text Object
// =======================

pub const CDXOBJ_TEXT: u16 = 0x8006; // kCDXObj_Text: An arbitrary block of (possibly styled) text.

// =======================
// Text Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrarily named property attached to objects.

// =======================
// Text Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_2D_POSITION: u16 = 0x0200; // 2D position (Y, X).

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding box (required until 6.0).
pub const CDXPROP_ROTATION_ANGLE: u16 = 0x0205; // Angular orientation in degrees * 65536.

// =======================
// Text Content & Styling
// =======================

pub const CDXPROP_TEXT: u16 = 0x0700; // Required: the text content.

pub const CDXPROP_JUSTIFICATION: u16 = 0x0701; // Horizontal justification (enumerated).
pub const CDXPROP_LINE_HEIGHT: u16 = 0x0702; // Line height.
pub const CDXPROP_WORD_WRAP_WIDTH: u16 = 0x0703; // Word-wrap width.
pub const CDXPROP_LINE_STARTS: u16 = 0x0704; // Line start positions with count.

pub const CDXPROP_LABEL_ALIGNMENT: u16 = 0x0705; // Alignment with respect to node position (enumerated).
pub const CDXPROP_LABEL_LINE_HEIGHT: u16 = 0x0706; // Text line height for atom labels.
pub const CDXPROP_CAPTION_LINE_HEIGHT: u16 = 0x0707; // Text line height for captions.

pub const CDXPROP_INTERPRET_CHEMICALLY: u16 = 0x0708; // Whether to interpret text chemically.

// =======================
// Font & Style Properties
// =======================

pub const CDXPROP_LABEL_STYLE: u16 = 0x080A; // Default atom label font style (unused).
pub const CDXPROP_CAPTION_STYLE: u16 = 0x080B; // Default caption font style (unused).

pub const CDXPROP_LABEL_FONT: u16 = 0x081A; // Default font family for atom labels.
pub const CDXPROP_CAPTION_FONT: u16 = 0x081B; // Default font family for captions.

pub const CDXPROP_LABEL_SIZE: u16 = 0x081C; // Default font size for atom labels.
pub const CDXPROP_CAPTION_SIZE: u16 = 0x081D; // Default font size for captions.

pub const CDXPROP_LABEL_FACE: u16 = 0x081E; // Default font face for atom labels.
pub const CDXPROP_CAPTION_FACE: u16 = 0x081F; // Default font face for captions.

pub const CDXPROP_LABEL_COLOR: u16 = 0x0820; // Default color for atom labels.
pub const CDXPROP_CAPTION_COLOR: u16 = 0x0821; // Default color for captions.

pub const CDXPROP_CAPTION_JUSTIFICATION: u16 = 0x080C; // Horizontal justification for captions (enumerated).
pub const CDXPROP_LABEL_JUSTIFICATION: u16 = 0x0823; // Default justification for atom labels (enumerated).
