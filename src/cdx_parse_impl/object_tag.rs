use crate::cdx::object_tag::ObjectTag;
use crate::cdx_tags::object_tag_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(ObjectTag, CDXOBJ_OBJECT_TAG, {
    name: CDXPROP_NAME,
    object_type: CDXPROP_OBJECTTAG_TYPE,
    trackig: CDXPROP_OBJECTTAG_TRACKING,
    oersistent: CDXPROP_OBJECTTAG_PERSISTENT,
    value: CDXPROP_OBJECTTAG_VALUE,
    positioning: CDXPROP_POSITIONING,
    position_angle: CDXPROP_POSITION_ANGLE,
    position_offset: CDXPROP_POSITION_ANGLE_OFFSET,
});
