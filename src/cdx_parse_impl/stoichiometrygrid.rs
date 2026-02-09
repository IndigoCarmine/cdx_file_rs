use crate::cdx::stoichiometrygrid::StoichiometryGrid;
use crate::cdx_tags::stoichiometrygrid_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(StoichiometryGrid, CDXOBJ_STOICHIOMETRYGRID, {
    position_2d: CDXPROP_STOICHIOMETRYGRID_POSITION,
    raw_data: CDXPROP_STOICHIOMETRYGRID_RAW,
});
