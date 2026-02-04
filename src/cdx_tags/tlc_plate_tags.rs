// =======================
// TLC Plate Object
// =======================


// In the document, this is 0x8023. But actual TLCPlate objects seem to use 0x801D (probably?)
// pub const CDXOBJ_TLC_PLATE: u16 = 0x8023; // kCDXObj_TLCPlate: Thin Layer Chromatography plate.
pub const CDXOBJ_TLC_PLATE: u16 = 0x801D; // kCDXObj_TLCPlate: Thin Layer Chromatography plate.

// =======================
// TLC Plate Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.
pub const CDXOBJ_TLC_LANE: u16 = 0x8024; // TLC lane (vertical series of spots).

// =======================
// TLC Plate Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle.

pub const CDXPROP_TOP_LEFT: u16 = 0x0209; // Top-left corner (2D point).
pub const CDXPROP_TOP_RIGHT: u16 = 0x020A; // Top-right corner (2D point).
pub const CDXPROP_BOTTOM_RIGHT: u16 = 0x020B; // Bottom-right corner (2D point).
pub const CDXPROP_BOTTOM_LEFT: u16 = 0x020C; // Bottom-left corner (2D point).

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Style Properties
// =======================

pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806; // Bold line width.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.
pub const CDXPROP_MARGIN_WIDTH: u16 = 0x0808; // Margin width.

pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A; // Default font family.
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C; // Default font size.
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E; // Default font face.

// =======================
// TLC-Specific Properties
// =======================

pub const CDXPROP_TLC_ORIGIN_FRACTION: u16 = 0x0AA0; // Origin line position (fraction of height from bottom).
pub const CDXPROP_TLC_SOLVENT_FRONT_FRACTION: u16 = 0x0AA1; // Solvent front position (fraction of height from top).
pub const CDXPROP_TLC_SHOW_ORIGIN: u16 = 0x0AA2; // Show origin line.
pub const CDXPROP_TLC_SHOW_SOLVENT_FRONT: u16 = 0x0AA3; // Show solvent front line.
pub const CDXPROP_TLC_SHOW_BORDERS: u16 = 0x0AA4; // Show plate borders.
