use crate::cdx::sequence::Sequence;
use crate::cdx_tags::sequence_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Sequence, CDXOBJ_SEQUENCE, {
    sequence_identifier: CDXPROP_SEQUENCE_IDENTIFIER,
});
