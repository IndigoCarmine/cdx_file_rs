use crate::cdx::border::Border;
use crate::cdx_tags::border_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Border, CDXOBJ_BORDER, {
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    line_width: CDXPROP_LINE_WIDTH,
    side: CDXPROP_SIDE,
    line_type: CDXPROP_LINE_TYPE,
});
