// =======================
// Named Alternative Group Object
// =======================

pub const CDXOBJ_NAMED_ALTERNATIVE_GROUP: u16 = 0x800A; // kCDXObj_NamedAlternativeGroup: Container for alternative substituents (R-Group/G-Group).

// =======================
// Named Alternative Group Subobjects
// =======================

pub const CDXOBJ_GROUP: u16 = 0x8002; // kCDXObj_Group: Logical collection of objects.
pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // kCDXObj_Fragment: Fragment definition.
pub const CDXOBJ_TEXT: u16 = 0x8006; // kCDXObj_Text: Required: group name text.
pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Named Alternative Group Properties
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
// Named Alternative Group-Specific Properties
// =======================

pub const CDXPROP_NAMED_ALTERNATIVE_GROUP_TEXT_FRAME: u16 = 0x0B00; // Upper portion bounding box (group name).
pub const CDXPROP_NAMED_ALTERNATIVE_GROUP_GROUP_FRAME: u16 = 0x0B01; // Lower portion bounding box (group definition).
pub const CDXPROP_NAMED_ALTERNATIVE_GROUP_VALENCE: u16 = 0x0B02; // Number of attachment points per alternative.
