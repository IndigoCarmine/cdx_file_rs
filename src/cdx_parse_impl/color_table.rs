use crate::cdx::color_table::ColorTable;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

// ColorTable is stored as a property (0x0300) in CDX files, not as an object.
// According to CDX Format Specification (ColorTable.md):
// "This object is used only in CDXML files. In CDX files, a kCDXProp_ColorTable property is used instead."
// But for compatibility with the NodePayload enum, we provide a TAG.
pub const CDXOBJ_COLOR_TABLE: u16 = 0x0300;

// The property tag for color table data within the raw object
const CDXPROP_COLOR_TABLE_DATA: u16 = 0x0300;

impl TaggedObject for ColorTable {
    const TAG: u16 = CDXOBJ_COLOR_TABLE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Try to decode from embedded property data if present
        if let Some(data) = raw.get_property(CDXPROP_COLOR_TABLE_DATA) {
            return ColorTable::decode(data);
        }

        // If no property data, return default color table
        Ok(ColorTable::default())
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Encode color table as a property
        let encoded_data = self.encode()?;
        properties.push(RawCdxProperty {
            tag: CDXPROP_COLOR_TABLE_DATA,
            value: encoded_data,
        });

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: 0,
            properties,
            children: Vec::new(),
        })
    }
}
