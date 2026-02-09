use crate::cdx::reaction_step::ReactionStep;
use crate::cdx_tags::reaction_step_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(ReactionStep, CDXOBJ_REACTION_STEP, {
    z_order: CDXPROP_Z_ORDER,
    visible: CDXPROP_VISIBLE,
    bounding_box: CDXPROP_BOUNDING_BOX,
    reaction_step_reactants: CDXPROP_REACTION_STEP_REACTANTS,
    reaction_step_products: CDXPROP_REACTION_STEP_PRODUCTS,
    reaction_step_plusses: CDXPROP_REACTION_STEP_PLUSSES,
    reaction_step_arrows: CDXPROP_REACTION_STEP_ARROWS,
    reaction_step_objectsabovearrow: CDXPROP_REACTION_STEP_OBJECTS_ABOVE_ARROW,
    reaction_step_objectsbelowarrow: CDXPROP_REACTION_STEP_OBJECTS_BELOW_ARROW,
});
