// =======================
// Embedded Object
// =======================

pub const CDXOBJ_EMBEDDED_OBJECT: u16 = 0x8009; // kCDXObj_EmbeddedObject: PICT, Metafile, or OLE object.

// =======================
// Embedded Object Subobjects
// =======================

pub const CDXOBJ_OBJECT_TAG: u16 = 0x8011; // kCDXObj_ObjectTag: Arbitrary metadata tag.

// =======================
// Embedded Object Properties
// =======================

// NOTE: `id` is implicit (UINT16) and not a CDX property constant.

pub const CDXPROP_Z_ORDER: u16 = 0x000A; // Back-to-front drawing order.

pub const CDXPROP_BOUNDING_BOX: u16 = 0x0204; // Required: bounding rectangle.
pub const CDXPROP_ROTATION_ANGLE: u16 = 0x0205; // Angular orientation (degrees * 65536).

pub const CDXPROP_FOREGROUND_COLOR: u16 = 0x0301; // Foreground color index.
pub const CDXPROP_BACKGROUND_COLOR: u16 = 0x0302; // Background color index.

// =======================
// Picture/Image Properties
// =======================

pub const CDXPROP_PICTURE_EDITION: u16 = 0x0A60; // Macintosh Publish & Subscribe edition.
pub const CDXPROP_PICTURE_EDITION_ALIAS: u16 = 0x0A61; // Macintosh edition alias.
pub const CDXPROP_MAC_PICT: u16 = 0x0A62; // Macintosh PICT data.
pub const CDXPROP_WINDOWS_METAFILE: u16 = 0x0A63; // Windows Metafile data.
pub const CDXPROP_OLE_OBJECT: u16 = 0x0A64; // OLE object data.
pub const CDXPROP_ENHANCED_METAFILE: u16 = 0x0A65; // Windows Enhanced Metafile data.
pub const CDXPROP_GIF: u16 = 0x0A6E; // GIF image data.
pub const CDXPROP_TIFF: u16 = 0x0A6F; // TIFF image data.
pub const CDXPROP_PNG: u16 = 0x0A70; // PNG image data.
pub const CDXPROP_JPEG: u16 = 0x0A71; // JPEG image data.
pub const CDXPROP_BMP: u16 = 0x0A72; // BMP image data.
