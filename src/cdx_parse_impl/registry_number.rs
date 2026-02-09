use crate::cdx::registry_number::RegistryNumber;
use crate::cdx_tags::registry_number_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(RegistryNumber, CDXOBJ_REGISTRY_NUMBER, {
    registry_number: CDXPROP_REGISTRY_NUMBER,
    registry_authority: CDXPROP_REGISTRY_AUTHORITY,
});
