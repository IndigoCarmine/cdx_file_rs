use crate::cdx::curve::Curve;
use crate::cdx_tags::curve_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Curve, CDXOBJ_CURVE, {
    z_order: CDXPROP_Z_ORDER,
    ignore_warnings: CDXPROP_IGNORE_WARNINGS,
    chemical_warning: CDXPROP_CHEMICAL_WARNING,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    curve_type: CDXPROP_CURVE_TYPE,
    arrowhead_size: CDXPROP_ARROWHEAD_SIZE,
    curve_points: CDXPROP_CURVE_POINTS,
    curve_points_3d: CDXPROP_CURVE_POINTS3D,
    arrowhead_type: CDXPROP_ARROWHEAD_TYPE,
    arrowhead_center_size: CDXPROP_ARROWHEAD_CENTER_SIZE,
    arrowhead_width: CDXPROP_ARROWHEAD_WIDTH,
    arrow_arrowhead_head: CDXPROP_ARROW_ARROWHEAD_HEAD,
    arrow_arrowhead_tail: CDXPROP_ARROW_ARROWHEAD_TAIL,
    fill_type: CDXPROP_FILL_TYPE,
    closed: CDXPROP_CLOSED,
    curve_spacing: CDXPROP_CURVE_SPACING,
});
