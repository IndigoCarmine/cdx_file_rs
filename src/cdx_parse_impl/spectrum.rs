use crate::cdx::spectrum::Spectrum;
use crate::cdx_tags::spectrum_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Spectrum, CDXOBJ_SPECTRUM, {
    z_order: CDXPROP_Z_ORDER,
    ignore_warnings: CDXPROP_IGNORE_WARNINGS,
    chemical_warning: CDXPROP_CHEMICAL_WARNING,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    bold_width: CDXPROP_BOLD_WIDTH,
    line_width: CDXPROP_LINE_WIDTH,
    label_style_font: CDXPROP_LABEL_STYLE_FONT,
    label_style_size: CDXPROP_LABEL_STYLE_SIZE,
    label_style_face: CDXPROP_LABEL_STYLE_FACE,
    spectrum_x_spacing: CDXPROP_SPECTRUM_X_SPACING,
    spectrum_x_low: CDXPROP_SPECTRUM_X_LOW,
    spectrum_x_type: CDXPROP_SPECTRUM_X_TYPE,
    spectrum_y_type: CDXPROP_SPECTRUM_Y_TYPE,
    spectrum_x_axis_label: CDXPROP_SPECTRUM_X_AXIS_LABEL,
    spectrum_y_axis_label: CDXPROP_SPECTRUM_Y_AXIS_LABEL,
    spectrum_data_point: CDXPROP_SPECTRUM_DATA_POINT,
    spectrum_class: CDXPROP_SPECTRUM_CLASS,
    spectrum_y_low: CDXPROP_SPECTRUM_Y_LOW,
    spectrum_y_scale: CDXPROP_SPECTRUM_Y_SCALE,
});
