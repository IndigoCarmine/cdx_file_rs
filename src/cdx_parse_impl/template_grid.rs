use crate::cdx::template_grid::TemplateGrid;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::error::CdxError;

pub const CDXOBJ_TEMPLATE_GRID: u16 = 0x800B;

impl TaggedObject for TemplateGrid {
    const TAG: u16 = CDXOBJ_TEMPLATE_GRID;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let template = TemplateGrid::new(raw.id);
        // TODO: Parse template grid properties when needed
        Ok(template)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties: Vec::new(),
            children: Vec::new(),
        })
    }
}
