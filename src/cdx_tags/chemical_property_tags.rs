// =======================
// Chemical Property Object
// =======================

pub const CDXOBJ_CHEMICAL_PROPERTY: u16 = 0x8026; // kCDXObj_ChemicalProperty: Physical/chemical property annotation.

// =======================
// Chemical Property Subobjects
// =======================

// (none)

// =======================
// Chemical Property Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_NAME: u16 = 0x0008; // Object name.

pub const CDXPROP_BASIS_OBJECTS: u16 = 0x0B82; // Objects defining the property.
pub const CDXPROP_CHEMICAL_PROPERTY_TYPE: u16 = 0x0BB0; // Property type (name, formula, MW, etc.) (enumerated).
pub const CDXPROP_CHEMICAL_PROPERTY_DISPLAY_ID: u16 = 0x0BB1; // ID of display object.
pub const CDXPROP_CHEMICAL_PROPERTY_IS_ACTIVE: u16 = 0x0BB2; // Auto-update flag.

pub const CDXPROP_POSITIONING: u16 = 0x0D06; // Positioning type (enumerated).
pub const CDXPROP_POSITIONING_ANGLE: u16 = 0x0D07; // Angular positioning (degrees * 65536).
pub const CDXPROP_POSITIONING_OFFSET: u16 = 0x0D08; // Offset positioning.
