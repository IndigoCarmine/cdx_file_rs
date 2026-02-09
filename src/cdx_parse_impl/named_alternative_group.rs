use crate::cdx::named_alternative_group::NamedAlternativeGroup;
use crate::cdx_tags::named_alternative_group_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(NamedAlternativeGroup, CDXOBJ_NAMED_ALTERNATIVE_GROUP, {
    z_order: CDXPROP_Z_ORDER,
    ignore_warnings: CDXPROP_IGNORE_WARNINGS,
    chemical_warning: CDXPROP_CHEMICAL_WARNING,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    named_alternative_group_text_frame: CDXPROP_NAMED_ALTERNATIVE_GROUP_TEXT_FRAME,
    named_alternative_group_group_frame: CDXPROP_NAMED_ALTERNATIVE_GROUP_GROUP_FRAME,
    named_alternative_group_valence: CDXPROP_NAMED_ALTERNATIVE_GROUP_VALENCE,
});
