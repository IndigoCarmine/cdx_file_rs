use crate::cdx::sequence::Sequence;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::CDXString;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::sequence_tags::*;
use crate::error::CdxError;


impl TaggedObject for Sequence {
    const TAG: u16 = CDXOBJ_SEQUENCE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut sequence = Sequence::new(raw.id);

        // Parse sequence_identifier (required)
        if let Some(id_data) = raw.get_property(CDXPROP_SEQUENCE_IDENTIFIER) {
            sequence.sequence_identifier = CDXString::decode(id_data).ok();
        }

        Ok(sequence)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject::new(Self::TAG, self.id);

        // Write sequence_identifier
        if let Some(ref identifier) = self.sequence_identifier {
            raw.add_property(CDXPROP_SEQUENCE_IDENTIFIER, identifier.encode()?);
        }

        Ok(raw)
    }
}
