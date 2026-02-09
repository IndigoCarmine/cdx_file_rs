use crate::cdx::tlc_spot::TLCSpot;
use crate::cdx_tags::tlc_spot_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(TLCSpot, CDXOBJ_TLC_SPOT, {
    visible: CDXPROP_VISIBLE,
    width: CDXPROP_WIDTH,
    height: CDXPROP_HEIGHT,
    curve_type: CDXPROP_CURVE_TYPE,
    tlc_rf: CDXPROP_TLC_RF,
    tlc_tail: CDXPROP_TLC_TAIL,
    tlc_show_rf: CDXPROP_TLC_SHOW_RF,
});
