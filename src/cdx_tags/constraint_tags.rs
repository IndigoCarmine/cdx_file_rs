// =======================
// Constraint Object
// =======================

pub const CDXOBJ_CONSTRAINT: u16 = 0x8022; // kCDXObj_Constraint: Distance or angle constraint.

// =======================
// Constraint Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Constraint Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_NAME: u16 = 0x0008; // Object name.

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.

pub const CDXPROP_BOND_LENGTH: u16 = 0x0805; // Default bond length.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Line width.
pub const CDXPROP_HASH_SPACING: u16 = 0x0809; // Hash spacing.

pub const CDXPROP_LABEL_STYLE_FONT: u16 = 0x081A; // Label font family.
pub const CDXPROP_LABEL_STYLE_SIZE: u16 = 0x081C; // Label font size.
pub const CDXPROP_LABEL_STYLE_FACE: u16 = 0x081E; // Label font face.
pub const CDXPROP_LABEL_STYLE_COLOR: u16 = 0x0820; // Label color.

// =======================
// Constraint-Specific Properties
// =======================

pub const CDXPROP_BASIS_OBJECTS: u16 = 0x0B82; // Required: ordered list of defining objects.
pub const CDXPROP_CONSTRAINT_TYPE: u16 = 0x0B83; // Constraint type (distance/angle/exclusion) (enumerated).
pub const CDXPROP_CONSTRAINT_MIN: u16 = 0x0B84; // Minimum constraint value.
pub const CDXPROP_CONSTRAINT_MAX: u16 = 0x0B85; // Maximum constraint value.
pub const CDXPROP_IGNORE_UNCONNECTED_ATOMS: u16 = 0x0B86; // Ignore unconnected atoms in exclusion sphere.
pub const CDXPROP_DIHEDRAL_IS_CHIRAL: u16 = 0x0B87; // Dihedral signed/unsigned flag.
