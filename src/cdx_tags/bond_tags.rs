// =======================
// Bond Object
// =======================

pub const CDXOBJ_BOND: u16 = 0x8005; // kCDXObj_Bond: Chemical bond connecting two Node objects.

// =======================
// Bond Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrarily named property attached to objects.

// =======================
// Bond Properties (Common)
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.
pub const CDXPROP_IGNORE_WARNINGS: u16 = 0x000F; // Suppress chemical warnings.
pub const CDXPROP_CHEMICAL_WARNING: u16 = 0x0010; // Chemical warning text.
pub const CDXPROP_VISIBLE: u16 = 0x0011; // Visibility flag.

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Bond-Specific Properties
// =======================

pub const CDXPROP_BOND_ORDER: u16 = 0x0600; // Bond order (bit-encoded).
pub const CDXPROP_BOND_DISPLAY: u16 = 0x0601; // Primary bond display type.
pub const CDXPROP_BOND_DISPLAY2: u16 = 0x0602; // Secondary display for double bonds.
pub const CDXPROP_BOND_DOUBLE_POSITION: u16 = 0x0603; // Double bond line position.

pub const CDXPROP_BOND_BEGIN: u16 = 0x0604; // Required: begin Node object ID.
pub const CDXPROP_BOND_END: u16 = 0x0605; // Required: end Node object ID.

pub const CDXPROP_BOND_RESTRICT_TOPOLOGY: u16 = 0x0606; // Query topology restriction.
pub const CDXPROP_BOND_RESTRICT_RXN_PARTICIPATION: u16 = 0x0607; // Reaction participation restriction.

pub const CDXPROP_BOND_BEGIN_ATTACH: u16 = 0x0608; // Attachment point on begin node.
pub const CDXPROP_BOND_END_ATTACH: u16 = 0x0609; // Attachment point on end node.

pub const CDXPROP_BOND_CIP_STEREOCHEMISTRY: u16 = 0x060A; // Cahn–Ingold–Prelog stereochemistry.
pub const CDXPROP_BOND_CIRCULAR_ORDERING: u16 = 0x060B; // Ordered list of attached bonds.

pub const CDXPROP_BOND_SHOW_QUERY: u16 = 0x060C; // Show query indicator.
pub const CDXPROP_BOND_SHOW_STEREO: u16 = 0x060D; // Show stereochemistry indicator.
pub const CDXPROP_BOND_CROSSING_BONDS: u16 = 0x060E; // Bonds crossing this bond.
pub const CDXPROP_BOND_SHOW_RXN: u16 = 0x060F; // Show reaction-change indicator.

// =======================
// Geometry / Style Overrides
// =======================

pub const CDXPROP_BOND_SPACING: u16 = 0x0804; // Relative spacing of multiple bonds.
pub const CDXPROP_BOND_LENGTH: u16 = 0x0805; // Default bond length.
pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806; // Bold bond width.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.
pub const CDXPROP_MARGIN_WIDTH: u16 = 0x0808; // Margin around atom labels.
pub const CDXPROP_HASH_SPACING: u16 = 0x0809; // Hashed bond spacing.

pub const CDXPROP_LABEL_STYLE: u16 = 0x080A; // Atom label font style (unused).

pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A; // Atom label font family.
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C; // Atom label font size.
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E; // Atom label font face.

pub const CDXPROP_BOND_SPACING_ABS: u16 = 0x0822; // Absolute spacing of multiple bonds.
