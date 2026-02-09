use crate::cdx::seg_datum::SegDatum;
use crate::cdx_tags::segment_datum_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(SegDatum, CDXOBJ_SEGMENTDATUM, {
    sg_data_type: CDXPROP_SEGMENTDATUM_TYPE,
    sg_data_value: CDXPROP_SEGMENTDATUM_VALUE,
    sg_property_type: CDXPROP_SEGMENTDATUM_PROPERTY_TYPE,
    is_read_only: CDXPROP_SEGMENTDATUM_IS_READONLY,
});
