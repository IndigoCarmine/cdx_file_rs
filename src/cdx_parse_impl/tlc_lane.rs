use crate::cdx::tlc_lane::TlcLane;
use crate::cdx_tags::tlc_lane_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(TlcLane, CDXOBJ_TLC_LANE, {
    visible: CDXPROP_VISIBLE,
});
