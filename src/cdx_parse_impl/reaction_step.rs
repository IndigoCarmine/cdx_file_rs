use crate::cdx::reaction_step::ReactionStep;
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_parse_impl::raw_nodes::RawCdxObject;
use crate::error::CdxError;
use crate::cdx::values::*;
use crate::cdx::binary_codec::BinaryCodec;

pub const CDXOBJ_REACTION_STEP: u16 = 0x800E;

// Property tags
const CDXPROP_ZORDER: u16 = 0x000A;
const CDXPROP_VISIBLE: u16 = 0x0011;
const CDXPROP_BOUNDINGBOX: u16 = 0x0204;
const CDXPROP_REACTIONSTEP_REACTANTS: u16 = 0x0400;
const CDXPROP_REACTIONSTEP_PRODUCTS: u16 = 0x0401;
const CDXPROP_REACTIONSTEP_PLUSSES: u16 = 0x0402;
const CDXPROP_REACTIONSTEP_ARROWS: u16 = 0x0403;
const CDXPROP_REACTIONSTEP_OBJECTSABOVEARROW: u16 = 0x0404;
const CDXPROP_REACTIONSTEP_OBJECTSBELOWARROW: u16 = 0x0405;

impl TaggedObject for ReactionStep {
    const TAG: u16 = CDXOBJ_REACTION_STEP;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        let mut step = ReactionStep::new(raw.id);

        step.z_order = raw.get_property(CDXPROP_ZORDER)
            .and_then(|v| i16::decode(v).ok());
        
        step.visible = raw.get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());
        
        step.bounding_box = raw.get_property(CDXPROP_BOUNDINGBOX)
            .and_then(|v| Rectangle::decode(v).ok());
        
        // Parse object ID lists (each ID is u32, 4 bytes)
        step.reaction_step_reactants = raw.get_property(CDXPROP_REACTIONSTEP_REACTANTS)
            .map(|v| parse_id_list(v));
        
        step.reaction_step_products = raw.get_property(CDXPROP_REACTIONSTEP_PRODUCTS)
            .map(|v| parse_id_list(v));
        
        step.reaction_step_plusses = raw.get_property(CDXPROP_REACTIONSTEP_PLUSSES)
            .map(|v| parse_id_list(v));
        
        step.reaction_step_arrows = raw.get_property(CDXPROP_REACTIONSTEP_ARROWS)
            .map(|v| parse_id_list(v));
        
        step.reaction_step_objectsabovearrow = raw.get_property(CDXPROP_REACTIONSTEP_OBJECTSABOVEARROW)
            .map(|v| parse_id_list(v));
        
        step.reaction_step_objectsbelowarrow = raw.get_property(CDXPROP_REACTIONSTEP_OBJECTSBELOWARROW)
            .map(|v| parse_id_list(v));

        Ok(step)
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        use crate::cdx_parse_impl::raw_nodes::RawCdxProperty;
        
        let mut properties = Vec::new();

        if let Some(v) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ZORDER,
                value: v.encode()?,
            });
        }
        
        if let Some(v) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: v.encode()?,
            });
        }
        
        if let Some(ref v) = self.bounding_box {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BOUNDINGBOX,
                value: v.encode()?,
            });
        }
        
        if let Some(ref ids) = self.reaction_step_reactants {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_REACTANTS,
                value: encode_id_list(ids),
            });
        }
        
        if let Some(ref ids) = self.reaction_step_products {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_PRODUCTS,
                value: encode_id_list(ids),
            });
        }
        
        if let Some(ref ids) = self.reaction_step_plusses {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_PLUSSES,
                value: encode_id_list(ids),
            });
        }
        
        if let Some(ref ids) = self.reaction_step_arrows {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_ARROWS,
                value: encode_id_list(ids),
            });
        }
        
        if let Some(ref ids) = self.reaction_step_objectsabovearrow {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_OBJECTSABOVEARROW,
                value: encode_id_list(ids),
            });
        }
        
        if let Some(ref ids) = self.reaction_step_objectsbelowarrow {
            properties.push(RawCdxProperty {
                tag: CDXPROP_REACTIONSTEP_OBJECTSBELOWARROW,
                value: encode_id_list(ids),
            });
        }

        Ok(RawCdxObject {
            tag: Self::TAG,
            id: self.id,
            properties,
            children: Vec::new(),
        })
    }
}

/// Parse a list of u32 IDs from raw bytes
fn parse_id_list(data: &[u8]) -> Vec<u32> {
    data.chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

/// Encode a list of u32 IDs to bytes
fn encode_id_list(ids: &[u32]) -> Vec<u8> {
    ids.iter()
        .flat_map(|id| id.to_le_bytes())
        .collect()
}
