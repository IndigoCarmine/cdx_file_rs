use crate::cdx::geometry::Geometry;
use crate::cdx_tags::geometry_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Geometry, CDXOBJ_GEOMETRY, {
    visible: CDXPROP_VISIBLE,
    z_order: CDXPROP_Z_ORDER,
    bounding_box: CDXPROP_BOUNDING_BOX,
    position: CDXPROP_2D_POSITION,
    rotation_angle: CDXPROP_ROTATION_ANGLE,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
});
