use crate::cdx::arrow::Arrow;
use crate::cdx_tags::arrow_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Arrow, CDXOBJ_ARROW, {
    bounding_box: CDXPROP_2D_BOUNDS,
    z_order: CDXPROP_Z_ORDER,
    fill_type: CDXPROP_FILL_TYPE,
    arrowhead_head: CDXPROP_ARROWHEAD_HEAD,
    arrowhead_type: CDXPROP_ARROWHEAD_TYPE,
    head_size: CDXPROP_HEAD_SIZE,
    head_3d: CDXPROP_3D_HEAD,
    tail_3d: CDXPROP_3D_TAIL,
    center_3d: CDXPROP_3D_CENTER,
    major_axis_end_3d: CDXPROP_3D_MAJOR_AXIS_END,
    minor_axis_end_3d: CDXPROP_3D_MINOR_AXIS_END,
    foreground_color: CDXPROP_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    line_width: CDXPROP_LINE_WIDTH,
});
