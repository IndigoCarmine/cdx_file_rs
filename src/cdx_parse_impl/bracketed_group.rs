use crate::cdx::bracketed_group::BracketedGroup;
use crate::cdx_tags::bracketed_group_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(BracketedGroup, CDXOBJ_BRACKETED_GROUP, {
    bracket_usage: CDXPROP_BRACKET_USAGE,
    polymer_repeat_pattern: CDXPROP_POLYMER_REPEAT_PATTERN,
    polymer_flip_type: CDXPROP_POLYMER_FLIP_TYPE,
    bracketed_objects: CDXPROP_BRACKETED_OBJECTS,
    bracket_repeat_count: CDXPROP_BRACKET_REPEAT_COUNT,
    bracket_component_order: CDXPROP_BRACKET_COMPONENT_ORDER,
    bracket_sru_label: CDXPROP_BRACKET_SRU_LABEL,
});
