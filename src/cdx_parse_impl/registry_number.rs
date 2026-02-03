use crate::cdx::registry_number::RegistryNumber;
use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::values::CDXString;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::registry_number_tags::*;
use crate::error::CdxError;


impl TaggedObject for RegistryNumber {
    const TAG: u16 = CDXOBJ_REGISTRY_NUMBER;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut reg_num = RegistryNumber::new(raw.id);

        // Parse registry_number (required)
        if let Some(number_data) = raw.get_property(CDXPROP_REGISTRY_NUMBER) {
            reg_num.registry_number = CDXString::decode(number_data).ok();
        }

        // Parse registry_authority (required)
        if let Some(authority_data) = raw.get_property(CDXPROP_REGISTRY_AUTHORITY) {
            reg_num.registry_authority = CDXString::decode(authority_data).ok();
        }

        Ok(reg_num)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut raw = RawCdxObject::new(Self::TAG, self.id);

        // Write registry_number
        if let Some(ref number) = self.registry_number {
            raw.add_property(CDXPROP_REGISTRY_NUMBER, number.encode()?);
        }

        // Write registry_authority
        if let Some(ref authority) = self.registry_authority {
            raw.add_property(CDXPROP_REGISTRY_AUTHORITY, authority.encode()?);
        }

        Ok(raw)
    }
}
