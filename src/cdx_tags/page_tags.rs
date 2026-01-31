// =======================
// Page Object
// =======================

pub const CDXOBJ_PAGE: u16 = 0x8001; // kCDXObj_Page: Drawing space corresponding to a page.

// =======================
// Page Subobjects
// =======================

pub const CDXOBJ_GROUP: u16 = 0x8002; // Logical collection of objects.
pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // Chemically meaningful fragment.
pub const CDXOBJ_TEXT: u16 = 0x8006; // Text object.
pub const CDXOBJ_GRAPHIC: u16 = 0x8007; // Graphic primitive.
pub const CDXOBJ_BRACKETED_GROUP: u16 = 0x8017; // Bracketed collection of objects.
pub const CDXOBJ_CURVE: u16 = 0x8008; // Bézier curve.
pub const CDXOBJ_EMBEDDED_OBJECT: u16 = 0x8009; // PICT, Metafile, or OLE object.
pub const CDXOBJ_TABLE: u16 = 0x8016; // Grid-like arrangement of drawing spaces.
pub const CDXOBJ_NAMED_ALTERNATIVE_GROUP: u16 = 0x800A; // Alternative substituent container.
pub const CDXOBJ_REACTION_SCHEME: u16 = 0x800D; // Single- or multi-step reaction scheme.
pub const CDXOBJ_REACTION_STEP: u16 = 0x800E; // One step of a reaction.
pub const CDXOBJ_SPECTRUM: u16 = 0x8010; // Spectral plot.
pub const CDXOBJ_SEQUENCE: u16 = 0x8013; // Member of an ordered sequence.
pub const CDXOBJ_CROSS_REFERENCE: u16 = 0x8014; // Link to a sequence.
pub const CDXOBJ_BORDER: u16 = 0x8020; // Border definition.
pub const CDXOBJ_GEOMETRY: u16 = 0x8021; // Geometrical relationship.
pub const CDXOBJ_CONSTRAINT: u16 = 0x8022; // Distance or angle constraint.
pub const CDXOBJ_TLC_PLATE: u16 = 0x8023; // TLC plate object.
pub const CDXOBJ_SPLITTER: u16 = 0x8015; // Horizontal page splitter.
pub const CDXOBJ_CHEMICAL_PROPERTY: u16 = 0x8026; // Chemical/physical property annotation.
pub const CDXOBJ_ARROW: u16 = 0x8027; // Line or arc with optional arrowheads.

// =======================
// Page Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

pub const CDXPROP_WIDTH_PAGES: u16 = 0x080F; // Width in pages.
pub const CDXPROP_HEIGHT_PAGES: u16 = 0x0810; // Height in pages.
pub const CDXPROP_DRAWING_SPACE_TYPE: u16 = 0x0811; // Drawing space type (enumerated).

pub const CDXPROP_WIDTH: u16 = 0x0812; // Page width in CDX units.
pub const CDXPROP_HEIGHT: u16 = 0x0813; // Page height in CDX units.
pub const CDXPROP_PAGE_OVERLAP: u16 = 0x0814; // Overlap when tiling pages.

pub const CDXPROP_HEADER: u16 = 0x0815; // Header text.
pub const CDXPROP_HEADER_POSITION: u16 = 0x0816; // Header vertical offset.
pub const CDXPROP_FOOTER: u16 = 0x0817; // Footer text.
pub const CDXPROP_FOOTER_POSITION: u16 = 0x0818; // Footer vertical offset.

pub const CDXPROP_PRINT_TRIM_MARKS: u16 = 0x0819; // Print trim marks flag.

pub const CDXPROP_SPLITTER_POSITIONS: u16 = 0x1FF0; // Page splitter positions.
pub const CDXPROP_PAGE_DEFINITION: u16 = 0x1FF1; // Page formatting definition.

pub const CDXPROP_BOUNDS_IN_PARENT: u16 = 0x0206; // Page bounds in parent coordinate space.
