// =======================
// Group Object
// =======================

pub const CDXOBJ_GROUP: u16 = 0x8002; // kCDXObj_Group: Logical collection of ChemDraw objects.

// =======================
// Group Subobjects
// =======================

pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // Chemically meaningful fragment.
pub const CDXOBJ_TEXT: u16 = 0x8006; // Text object.
pub const CDXOBJ_GRAPHIC: u16 = 0x8007; // Graphic primitive.
pub const CDXOBJ_CURVE: u16 = 0x8008; // Bézier curve.
pub const CDXOBJ_NAMED_ALTERNATIVE_GROUP: u16 = 0x800A; // Alternative substituent container.
pub const CDXOBJ_REACTION_STEP: u16 = 0x800E; // Reaction step description.
pub const CDXOBJ_SPECTRUM: u16 = 0x8010; // Spectral plot.
pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // Arbitrary metadata tag.

// =======================
// Group Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle.
pub const CDXPROP_GROUP_INTEGRAL: u16 = 0x1100; // Integral (non-subdivisible) group flag.
