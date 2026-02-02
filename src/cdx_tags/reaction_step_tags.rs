// =======================
// Reaction Step Object
// =======================

pub const CDXOBJ_REACTION_STEP: u16 = 0x800E; // kCDXObj_ReactionStep: One step of a reaction.

// =======================
// Reaction Step Subobjects
// =======================

// (none)

// =======================
// Reaction Step Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_REACTION_STEP_ATOM_MAP: u16 = 0x0C00; // Atom mapping pairs (reactant-to-product).
pub const CDXPROP_REACTION_STEP_REACTANTS: u16 = 0x0C01; // Ordered list of reactant IDs.
pub const CDXPROP_REACTION_STEP_PRODUCTS: u16 = 0x0C02; // Ordered list of product IDs.
pub const CDXPROP_REACTION_STEP_PLUSSES: u16 = 0x0C03; // Ordered list of plus sign IDs.
pub const CDXPROP_REACTION_STEP_ARROWS: u16 = 0x0C04; // Ordered list of arrow IDs.
pub const CDXPROP_REACTION_STEP_OBJECTS_ABOVE_ARROW: u16 = 0x0C05; // Objects above reaction arrow.
pub const CDXPROP_REACTION_STEP_OBJECTS_BELOW_ARROW: u16 = 0x0C06; // Objects below reaction arrow.
pub const CDXPROP_REACTION_STEP_ATOM_MAP_MANUAL: u16 = 0x0C07; // Manual atom mapping pairs.
pub const CDXPROP_REACTION_STEP_ATOM_MAP_AUTO: u16 = 0x0C08; // Automatic atom mapping pairs.
