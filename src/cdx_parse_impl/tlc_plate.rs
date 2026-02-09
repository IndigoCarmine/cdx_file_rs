use crate::cdx::tlc_plate::TLCPlate;
use crate::cdx_tags::tlc_plate_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(TLCPlate, CDXOBJ_TLC_PLATE, {
    z_order: CDXPROP_Z_ORDER,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    top_left: CDXPROP_TOP_LEFT,
    top_right: CDXPROP_TOP_RIGHT,
    bottom_right: CDXPROP_BOTTOM_RIGHT,
    bottom_left: CDXPROP_BOTTOM_LEFT,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    bold_width: CDXPROP_BOLD_WIDTH,
    line_width: CDXPROP_LINE_WIDTH,
    margin_width: CDXPROP_MARGIN_WIDTH,
    label_font: CDXPROP_LABEL_STYLE_FONT,
    label_size: CDXPROP_LABEL_STYLE_SIZE,
    label_face: CDXPROP_LABEL_STYLE_FACE,
    tlc_origin_fraction: CDXPROP_TLC_ORIGIN_FRACTION,
    tlc_solvent_front_fraction: CDXPROP_TLC_SOLVENT_FRONT_FRACTION,
    tlc_show_origin: CDXPROP_TLC_SHOW_ORIGIN,
    tlc_show_solvent_front: CDXPROP_TLC_SHOW_SOLVENT_FRONT,
    tlc_show_borders: CDXPROP_TLC_SHOW_BORDERS,
});
