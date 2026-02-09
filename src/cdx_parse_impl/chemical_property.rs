use crate::cdx::chemical_property::ChemicalProperty;
use crate::cdx_tags::chemical_property_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(ChemicalProperty, CDXOBJ_CHEMICAL_PROPERTY, {
    name: CDXPROP_NAME,
    basis_objects: CDXPROP_BASIS_OBJECTS,
    chemical_property_type: CDXPROP_CHEMICAL_PROPERTY_TYPE,
    chemical_property_display_id: CDXPROP_CHEMICAL_PROPERTY_DISPLAY_ID,
    chemical_property_is_active: CDXPROP_CHEMICAL_PROPERTY_IS_ACTIVE,
    positioning: CDXPROP_POSITIONING,
    positioning_angle: CDXPROP_POSITIONING_ANGLE,
    positioning_offset: CDXPROP_POSITIONING_OFFSET,
});
