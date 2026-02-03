use crate::cdx::values::Rectangle;
use serde::{Deserialize, Serialize};

/// Embedded Object
/// Represents PICT, Metafile, or OLE object embedded in the document
/// CDX ID: 0x8009
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddedObject {
    pub id: u32,

    // Common properties
    /// Back-to-front ordering index in 2D drawing (Optional)
    pub z_order: Option<i16>,

    // Geometry
    /// REQUIRED: Bounding rectangle for the embedded object
    pub bounding_box: Option<Rectangle>,
    /// Angular orientation in degrees * 65536 (Optional)
    pub rotation_angle: Option<i32>,

    // Color
    /// Foreground color index (Optional)
    pub foreground_color: Option<u16>,
    /// Background color index (Optional)
    pub background_color: Option<i16>,

    // Image data - mutually exclusive, store one
    /// Macintosh Publish & Subscribe edition (Optional)
    pub picture_edition: Option<Vec<u8>>,
    /// Macintosh edition alias (Optional)
    pub picture_edition_alias: Option<Vec<u8>>,
    /// Macintosh PICT data (Optional)
    pub mac_pict: Option<Vec<u8>>,
    /// Windows Metafile data (Optional)
    pub windows_metafile: Option<Vec<u8>>,
    /// OLE object data (Optional)
    pub ole_object: Option<Vec<u8>>,
    /// Windows Enhanced Metafile data (Optional)
    pub enhanced_metafile: Option<Vec<u8>>,
    /// GIF image data (Optional)
    pub gif: Option<Vec<u8>>,
    /// TIFF image data (Optional)
    pub tiff: Option<Vec<u8>>,
    /// PNG image data (Optional)
    pub png: Option<Vec<u8>>,
    /// JPEG image data (Optional)
    pub jpeg: Option<Vec<u8>>,
    /// BMP image data (Optional)
    pub bmp: Option<Vec<u8>>,
}

impl EmbeddedObject {
    /// Create a new EmbeddedObject with just an ID
    pub fn new(id: u32) -> Self {
        EmbeddedObject {
            id,
            z_order: None,
            bounding_box: None,
            rotation_angle: None,
            foreground_color: None,
            background_color: None,
            picture_edition: None,
            picture_edition_alias: None,
            mac_pict: None,
            windows_metafile: None,
            ole_object: None,
            enhanced_metafile: None,
            gif: None,
            tiff: None,
            png: None,
            jpeg: None,
            bmp: None,
        }
    }
}
