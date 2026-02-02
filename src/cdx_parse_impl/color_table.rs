use crate::cdx::color_table::ColorTable;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

// ColorTable is stored as a property (0x0300) in CDX files, not as an object
// But for compatibility with the NodePayload enum, we provide a TAG
pub const CDXOBJ_COLOR_TABLE: u16 = 0x0300;

impl TaggedObject for ColorTable {
    const TAG: u16 = CDXOBJ_COLOR_TABLE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let color_table = ColorTable { colors: Vec::new() };
        // TODO: Parse color table when needed
        Ok(color_table)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: 0,
            properties: Vec::new(),
            children: Vec::new(),
        })
    }
}
