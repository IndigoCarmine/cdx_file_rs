use crate::cdx::fragment::Fragment;
use crate::cdx_tags::fragment_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Fragment, CDXOBJ_FRAGMENT, {
    bounding_box: CDXPROP_BOUNDING_BOX,
    mole_racemic: CDXPROP_MOLE_RACEMIC,
    mole_absolute: CDXPROP_MOLE_ABSOLUTE,
    mole_relative: CDXPROP_MOLE_RELATIVE,
    mole_weight: CDXPROP_MOLE_WEIGHT,
    frag_connection_order: CDXPROP_FRAG_CONNECTION_ORDER,
});
