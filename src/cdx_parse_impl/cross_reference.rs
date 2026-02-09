use crate::cdx::cross_reference::CrossReference;
use crate::cdx_tags::cross_reference_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(CrossReference, CDXOBJ_CROSS_REFERENCE, {
    cross_reference_container: CDXPROP_CROSS_REFERENCE_CONTAINER,
    cross_reference_document: CDXPROP_CROSS_REFERENCE_DOCUMENT,
    cross_reference_identifier: CDXPROP_CROSS_REFERENCE_IDENTIFIER,
    cross_reference_sequence: CDXPROP_CROSS_REFERENCE_SEQUENCE,
});
