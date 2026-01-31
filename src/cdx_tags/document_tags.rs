// =======================
// Subobjects
// =======================

pub const CDXOBJ_PAGE: u16 = 0x8001; // kCDXObj_Page: A drawing space that can contain other objects.
pub const CDXOBJ_TEMPLATE_GRID: u16 = 0x800B; // kCDXObj_TemplateGrid: Layout definition for template documents.

pub const CDXPROP_COLOR_TABLE: u16 = 0x0300; // kCDXProp_ColorTable: Document-wide color palette.
pub const CDXPROP_FONT_TABLE: u16 = 0x0100; // kCDXProp_FontTable: List of fonts used in the document.

// =======================
// Properties (Document / Common)
// =======================

pub const CDXPROP_CREATION_USER_NAME: u16 = 0x0001; // Creator user name.
pub const CDXPROP_CREATION_DATE: u16 = 0x0002; // Object creation date.
pub const CDXPROP_CREATION_PROGRAM: u16 = 0x0003; // Program that created the object.
pub const CDXPROP_MODIFICATION_USER_NAME: u16 = 0x0004; // Last modifier user name.
pub const CDXPROP_MODIFICATION_DATE: u16 = 0x0005; // Last modification date.
pub const CDXPROP_MODIFICATION_PROGRAM: u16 = 0x0006; // Program that last modified the object.
pub const CDXPROP_NAME: u16 = 0x0008; // Object name.
pub const CDXPROP_COMMENT: u16 = 0x0009; // User comment.

// =======================
// Geometry / Appearance
// =======================

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Bounding rectangle.
pub const CDXPROP_ATOM_SHOW_QUERY: u16 = 0x043A; // Show atom query indicator.
pub const CDXPROP_ATOM_SHOW_STEREO: u16 = 0x043B; // Show atom stereochemistry indicator.
pub const CDXPROP_ATOM_SHOW_ATOM_NUMBER: u16 = 0x043C; // Show atom number.

pub const CDXPROP_BOND_SHOW_QUERY: u16 = 0x060C; // Show bond query indicator.
pub const CDXPROP_BOND_SHOW_STEREO: u16 = 0x060D; // Show bond stereochemistry indicator.
pub const CDXPROP_BOND_SHOW_RXN: u16 = 0x060F; // Show reaction-change indicator.

pub const CDXPROP_LABEL_LINE_HEIGHT: u16 = 0x0706; // Atom label text line height.
pub const CDXPROP_CAPTION_LINE_HEIGHT: u16 = 0x0707; // Caption text line height.
pub const CDXPROP_INTERPRET_CHEMICALLY: u16 = 0x0708; // Interpret text chemically.

// =======================
// Printing / Layout
// =======================

pub const CDXPROP_MAC_PRINT_INFO: u16 = 0x0800; // Macintosh TPrint data.
pub const CDXPROP_WIN_PRINT_INFO: u16 = 0x0801; // Windows DEVMODE data.
pub const CDXPROP_PRINT_MARGINS: u16 = 0x0802; // Document print margins.

pub const CDXPROP_CHAIN_ANGLE: u16 = 0x0803; // Default chain angle (deg * 65536).
pub const CDXPROP_BOND_SPACING: u16 = 0x0804; // Multiple bond spacing.
pub const CDXPROP_BOND_LENGTH: u16 = 0x0805; // Default bond length.
pub const CDXPROP_BOLD_WIDTH: u16 = 0x0806; // Bold bond width.
pub const CDXPROP_LINE_WIDTH: u16 = 0x0807; // Default line width.
pub const CDXPROP_MARGIN_WIDTH: u16 = 0x0808; // Space around atom labels.
pub const CDXPROP_HASH_SPACING: u16 = 0x0809; // Hashed bond spacing.

pub const CDXPROP_LABEL_STYLE: u16 = 0x080A; // Default atom label font style (unused).
pub const CDXPROP_CAPTION_STYLE: u16 = 0x080B; // Default caption font style (unused).
pub const CDXPROP_CAPTION_JUSTIFICATION: u16 = 0x080C; // Caption justification.
pub const CDXPROP_FRACTIONAL_WIDTHS: u16 = 0x080D; // Use fractional font widths.
pub const CDXPROP_MAGNIFICATION: u16 = 0x080E; // View magnification factor.

// =======================
// Font Defaults
// =======================

pub const CDXPROP_LABEL_FONT: u16 = 0x081A; // Atom label font family.
pub const CDXPROP_CAPTION_FONT: u16 = 0x081B; // Caption font family.
pub const CDXPROP_LABEL_SIZE: u16 = 0x081C; // Atom label font size.
pub const CDXPROP_CAPTION_SIZE: u16 = 0x081D; // Caption font size.
pub const CDXPROP_LABEL_FACE: u16 = 0x081E; // Atom label font face.
pub const CDXPROP_CAPTION_FACE: u16 = 0x081F; // Caption font face.
pub const CDXPROP_LABEL_COLOR: u16 = 0x0820; // Atom label color.
pub const CDXPROP_CAPTION_COLOR: u16 = 0x0821; // Caption color.
pub const CDXPROP_LABEL_JUSTIFICATION: u16 = 0x0823; // Atom label justification.

// =======================
// OLE / External Data
// =======================

pub const CDXPROP_FIX_INPLACE_EXTENT: u16 = 0x0824; // OLE in-place size.
pub const CDXPROP_FIX_INPLACE_GAP: u16 = 0x0826; // OLE in-place padding.
pub const CDXPROP_CARTRIDGE_DATA: u16 = 0x0827; // Oracle Cartridge transient data.

// =======================
// Window State
// =======================

pub const CDXPROP_WINDOW_IS_ZOOMED: u16 = 0x0900; // Window maximized state.
pub const CDXPROP_WINDOW_POSITION: u16 = 0x0901; // Window top-left position.
pub const CDXPROP_WINDOW_SIZE: u16 = 0x0902; // Window size (width, height).
