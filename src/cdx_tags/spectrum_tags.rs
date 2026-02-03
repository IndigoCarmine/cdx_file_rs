// =======================
// Spectrum Object
// =======================

pub const CDXOBJ_SPECTRUM: u16 = 0x8010; // kCDXObj_Spectrum: NMR, MS, IR or other spectral plot.

// =======================
// Spectrum Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Spectrum Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Required: bounding rectangle.

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Styling Properties
// =======================

pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806; // Bold width.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.

pub const CDXPROP_LABEL_STYLE: u16 = 0x080A; // Label font style (unused).
pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A; // Label font family.
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C; // Label font size.
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E; // Label font face.

// =======================
// Spectrum-Specific Properties
// =======================

pub const CDXPROP_SPECTRUM_X_SPACING: u16 = 0x0A80; // Required: X-axis spacing (ppm, Hz, etc.).
pub const CDXPROP_SPECTRUM_X_LOW: u16 = 0x0A81; // Required: first X-axis data point.
pub const CDXPROP_SPECTRUM_X_TYPE: u16 = 0x0A82; // X-axis unit type (enumerated).
pub const CDXPROP_SPECTRUM_Y_TYPE: u16 = 0x0A83; // Y-axis unit type (enumerated).
pub const CDXPROP_SPECTRUM_X_AXIS_LABEL: u16 = 0x0A84; // X-axis label text.
pub const CDXPROP_SPECTRUM_Y_AXIS_LABEL: u16 = 0x0A85; // Y-axis label text.
pub const CDXPROP_SPECTRUM_DATA_POINT: u16 = 0x0A86; // Required: Y-axis values array.
pub const CDXPROP_SPECTRUM_CLASS: u16 = 0x0A87; // Spectrum type (NMR, IR, etc.) (enumerated).
pub const CDXPROP_SPECTRUM_Y_LOW: u16 = 0x0A88; // Y offset for XML storage.
pub const CDXPROP_SPECTRUM_Y_SCALE: u16 = 0x0A89; // Y scaling for XML storage.
