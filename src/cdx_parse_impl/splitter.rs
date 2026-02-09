use crate::cdx::splitter::Splitter;
use crate::cdx_tags::splitter_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Splitter, CDXOBJ_SPLITTER, {
    position_2d: CDXPROP_2D_POSITION,
    page_definition: CDXPROP_PAGE_DEFINITION,
});
