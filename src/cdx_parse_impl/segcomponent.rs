use crate::cdx::segcomponent::SegComponent;
use crate::cdx_tags::segcomponent_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(SegComponent, CDXOBJ_SEGCOMPONENT, {
    width: CDXPROP_SEGCOMPONENT_WIDTH,
    component_is_reactant: CDXPROP_SEGCOMPONENT_IS_REACTANT,
    component_is_header: CDXPROP_SEGCOMPONENT_IS_HEADER,
});
