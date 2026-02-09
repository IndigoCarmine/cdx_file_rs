use crate::cdx::group::Group;
use crate::cdx_tags::group_tag::*;
use crate::impl_tagged_object;

impl_tagged_object!(Group, CDXOBJ_GROUP, {
    bounding_box: CDXPROP_BOUNDING_BOX,
    integral: CDXPROP_GROUP_INTEGRAL,
});
