use crate::cdx::text::TextObject;
use crate::cdx_tags::text_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(TextObject, CDXOBJ_TEXT, {
    text: CDXPROP_TEXT,
    z_order: CDXPROP_Z_ORDER,
    ignore_warnings: CDXPROP_IGNORE_WARNINGS,
    chemical_warning: CDXPROP_CHEMICAL_WARNING,
    visible: CDXPROP_VISIBLE,
    position_2d: CDXPROP_2D_POSITION,
    bounding_box: CDXPROP_BOUNDING_BOX,
    rotation_angle: CDXPROP_ROTATION_ANGLE,
    justification: CDXPROP_JUSTIFICATION,
    line_height: CDXPROP_LINE_HEIGHT,
    word_wrap_width: CDXPROP_WORD_WRAP_WIDTH,
    line_starts: CDXPROP_LINE_STARTS,
    label_alignment: CDXPROP_LABEL_ALIGNMENT,
    label_line_height: CDXPROP_LABEL_LINE_HEIGHT,
    caption_line_height: CDXPROP_CAPTION_LINE_HEIGHT,
    interpret_chemically: CDXPROP_INTERPRET_CHEMICALLY,
    label_font: CDXPROP_LABEL_STYLE_FONT,
    caption_font: CDXPROP_CAPTION_STYLE_FONT,
    label_size: CDXPROP_LABEL_STYLE_SIZE,
    caption_size: CDXPROP_CAPTION_STYLE_SIZE,
    label_face: CDXPROP_LABEL_STYLE_FACE,
    caption_face: CDXPROP_CAPTION_STYLE_FACE,
    label_color: CDXPROP_LABEL_STYLE_COLOR,
    caption_color: CDXPROP_CAPTION_STYLE_COLOR,
    caption_justification: CDXPROP_CAPTION_JUSTIFICATION,
    label_justification: CDXPROP_LABEL_JUSTIFICATION,
});
