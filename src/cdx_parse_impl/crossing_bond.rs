use crate::cdx::crossing_bond::CrossingBond;
use crate::cdx_tags::crossing_bond_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(CrossingBond, CDXOBJ_CROSSING_BOND, {
    crossing_bond_id: CDXPROP_CROSSING_BOND_ID,
    crossing_bond_begin_inside: CDXPROP_CROSSING_BOND_BEGIN_INSIDE,
});
