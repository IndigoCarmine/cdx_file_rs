use crate::cdx::table::Table;
use crate::cdx_tags::table_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Table, CDXOBJ_TABLE, {
    z_order: CDXPROP_Z_ORDER,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    bold_width: CDXPROP_BOLD_WIDTH,
    line_width: CDXPROP_LINE_WIDTH,
    margin_width: CDXPROP_MARGIN_WIDTH,
    label_style_font: CDXPROP_LABEL_STYLE_FONT,
    label_style_size: CDXPROP_LABEL_STYLE_SIZE,
    label_style_face: CDXPROP_LABEL_STYLE_FACE,
});
