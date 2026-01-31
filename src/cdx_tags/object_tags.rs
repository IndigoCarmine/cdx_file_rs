// =======================
// Predefined CDX Objects
// =======================

// Top-level / structural objects
pub const CDXOBJ_DOCUMENT: u16 = 0x8000; // kCDXObj_Document
pub const CDXOBJ_PAGE: u16 = 0x8001; // kCDXObj_Page
pub const CDXOBJ_GROUP: u16 = 0x8002; // kCDXObj_Group
pub const CDXOBJ_FRAGMENT: u16 = 0x8003; // kCDXObj_Fragment

// Core chemical objects
pub const CDXOBJ_NODE: u16 = 0x8004; // kCDXObj_Node
pub const CDXOBJ_BOND: u16 = 0x8005; // kCDXObj_Bond

// Text and graphics
pub const CDXOBJ_TEXT: u16 = 0x8006; // kCDXObj_Text
pub const CDXOBJ_GRAPHIC: u16 = 0x8007; // kCDXObj_Graphic
pub const CDXOBJ_CURVE: u16 = 0x8008; // kCDXObj_Curve
pub const CDXOBJ_EMBEDDED_OBJECT: u16 = 0x8009; // kCDXObj_EmbeddedObject

// Grouping / query-related objects
pub const CDXOBJ_NAMED_ALTERNATIVE_GROUP: u16 = 0x800A; // kCDXObj_NamedAlternativeGroup
pub const CDXOBJ_BRACKETED_GROUP: u16 = 0x8017; // kCDXObj_BracketedGroup
pub const CDXOBJ_BRACKET_ATTACHMENT: u16 = 0x8018; // kCDXObj_BracketAttachment
pub const CDXOBJ_CROSSING_BOND: u16 = 0x8019; // kCDXObj_CrossingBond

// Tables and layout
pub const CDXOBJ_TABLE: u16 = 0x8016; // kCDXObj_Table
pub const CDXOBJ_TEMPLATE_GRID: u16 = 0x800B; // kCDXObj_TemplateGrid
pub const CDXOBJ_SPLITTER: u16 = 0x8015; // kCDXObj_Splitter
pub const CDXOBJ_BORDER: u16 = 0x8020; // kCDXObj_Border
pub const CDXOBJ_GEOMETRY: u16 = 0x8021; // kCDXObj_Geometry
pub const CDXOBJ_CONSTRAINT: u16 = 0x8022; // kCDXObj_Constraint

// Reaction-related objects
pub const CDXOBJ_REACTION_SCHEME: u16 = 0x800D; // kCDXObj_ReactionScheme
pub const CDXOBJ_REACTION_STEP: u16 = 0x800E; // kCDXObj_ReactionStep
pub const CDXOBJ_ARROW: u16 = 0x8027; // kCDXObj_Arrow

// Spectra and analytical objects
pub const CDXOBJ_SPECTRUM: u16 = 0x8010; // kCDXObj_Spectrum
pub const CDXOBJ_CHEMICAL_PROPERTY: u16 = 0x8026; // kCDXObj_ChemicalProperty

// TLC-related objects
pub const CDXOBJ_TLC_PLATE: u16 = 0x8023; // kCDXObj_TLCPlate
pub const CDXOBJ_TLC_LANE: u16 = 0x8024; // kCDXObj_TLCLane
pub const CDXOBJ_TLC_SPOT: u16 = 0x8025; // kCDXObj_TLCSpot

// Reference and annotation objects
pub const CDXOBJ_REGISTRY_NUMBER: u16 = 0x800C; // kCDXObj_RegistryNumber
pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag
pub const CDXOBJ_SEQUENCE: u16 = 0x8013; // kCDXObj_Sequence
pub const CDXOBJ_CROSS_REFERENCE: u16 = 0x8014; // kCDXObj_CrossReference

// =======================
// Non-object structural elements (no object IDs)
// =======================

// Color table (property-level container)
pub const CDXPROP_COLOR_TABLE: u16 = 0x0300; // kCDXProp_ColorTable

// Font table (property-level container)
pub const CDXPROP_FONT_TABLE: u16 = 0x0100; // kCDXProp_FontTable

// Represents-property marker
pub const CDXPROP_REPRESENTS_PROPERTY: u16 = 0x000E; // kCDXProp_RepresentsProperty

// NOTE:
// - `color`, `font`, and `s` (style run) are structural elements, not CDX objects,
//   and therefore have no object constant values.
