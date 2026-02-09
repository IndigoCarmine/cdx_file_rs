use crate::cdx::object_tag::ObjectTag;
use crate::cdx_tags::object_tag_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(ObjectTag, CDXOBJ_OBJECT_TAG, {
    tag_name: CDXPROP_TAG_NAME,
    z_order: CDXPROP_Z_ORDER,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    position: CDXPROP_2D_POSITION,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    tag_type: CDXPROP_TAG_TYPE,
    tag_value: CDXPROP_TAG_VALUE,
});
