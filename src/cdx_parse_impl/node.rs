use crate::cdx::binary_codec::BinaryCodec;
use crate::cdx::node::Node;
use crate::cdx::values::*;
use crate::cdx_parse_impl::raw_nodes::{RawCdxObject, RawCdxProperty};
use crate::cdx_parse_impl::tagged_object::TaggedObject;
use crate::cdx_tags::node_tags::*;
use crate::error::CdxError;

impl TaggedObject for Node {
    const TAG: u16 = CDXOBJ_NODE;

    fn from_raw(raw: RawCdxObject) -> Result<Self, CdxError> {
        // Z-order using BinaryCodec
        let z_order = raw
            .get_property(CDXPROP_Z_ORDER)
            .and_then(|v| i16::decode(v).ok());

        // Ignore warnings
        let ignore_warnings = raw
            .get_property(CDXPROP_IGNORE_WARNINGS)
            .and_then(|v| bool::decode(v).ok());

        // Chemical warning
        let chemical_warning = raw
            .get_property(CDXPROP_CHEMICAL_WARNING)
            .and_then(|v| CDXString::decode(v).ok());

        // Visible
        let visible = raw
            .get_property(CDXPROP_VISIBLE)
            .and_then(|v| bool::decode(v).ok());

        // 2D Position
        let position_2d = raw
            .get_property(CDXPROP_2D_POSITION)
            .and_then(|v| Point2d::decode(v).ok());

        // 3D Position
        let position_3d = raw
            .get_property(CDXPROP_3D_POSITION)
            .and_then(|v| Point3d::decode(v).ok());

        // Foreground color
        let foreground_color = raw
            .get_property(CDXPROP_FOREGROUND_COLOR)
            .and_then(|v| u16::decode(v).ok());

        // Background color
        let background_color = raw
            .get_property(CDXPROP_BACKGROUND_COLOR)
            .and_then(|v| i16::decode(v).ok());

        // Node type
        let node_type = raw
            .get_property(CDXPROP_NODE_TYPE)
            .and_then(|v| i16::decode(v).ok());

        // Label display
        let label_display = raw
            .get_property(CDXPROP_NODE_LABEL_DISPLAY)
            .and_then(|v| i8::decode(v).ok());

        // Element (atomic number)
        let element = raw
            .get_property(CDXPROP_NODE_ELEMENT)
            .and_then(|v| i16::decode(v).ok());

        // Isotope
        let isotope = raw
            .get_property(CDXPROP_ATOM_ISOTOPE)
            .and_then(|v| i16::decode(v).ok());

        // Charge
        let charge = raw
            .get_property(CDXPROP_ATOM_CHARGE)
            .and_then(|v| i8::decode(v).ok());

        // Radical
        let radical = raw
            .get_property(CDXPROP_ATOM_RADICAL)
            .and_then(|v| u8::decode(v).ok());

        // Restrict free sites
        let restrict_free_sites = raw
            .get_property(CDXPROP_ATOM_RESTRICT_FREE_SITES)
            .and_then(|v| u8::decode(v).ok());

        // Restrict implicit H
        let restrict_implicit_h = raw
            .get_property(CDXPROP_ATOM_RESTRICT_IMPLICIT_H)
            .and_then(|v| bool::decode(v).ok());

        // Restrict ring bond count
        let restrict_ring_bond_count = raw
            .get_property(CDXPROP_ATOM_RESTRICT_RING_BOND_COUNT)
            .and_then(|v| i8::decode(v).ok());

        // Restrict unsaturated bonds
        let restrict_unsaturated_bonds = raw
            .get_property(CDXPROP_ATOM_RESTRICT_UNSATURATED_BONDS)
            .and_then(|v| i8::decode(v).ok());

        // Restrict rxn change
        let restrict_rxn_change = raw
            .get_property(CDXPROP_ATOM_RESTRICT_RXN_CHANGE)
            .and_then(|v| bool::decode(v).ok());

        // Restrict rxn stereo
        let restrict_rxn_stereo = raw
            .get_property(CDXPROP_ATOM_RESTRICT_RXN_STEREO)
            .and_then(|v| i8::decode(v).ok());

        // Abnormal valence
        let abnormal_valence = raw
            .get_property(CDXPROP_ATOM_ABNORMAL_VALENCE)
            .and_then(|v| bool::decode(v).ok());

        // Num hydrogens
        let num_hydrogens = raw
            .get_property(CDXPROP_ATOM_NUM_HYDROGENS)
            .and_then(|v| u16::decode(v).ok());

        // H dot
        let h_dot = raw
            .get_property(CDXPROP_ATOM_H_DOT)
            .and_then(|v| bool::decode(v).ok());

        // H dash
        let h_dash = raw
            .get_property(CDXPROP_ATOM_H_DASH)
            .and_then(|v| bool::decode(v).ok());

        // Geometry
        let geometry = raw
            .get_property(CDXPROP_ATOM_GEOMETRY)
            .and_then(|v| i8::decode(v).ok());

        // Bond ordering
        let bond_ordering = raw
            .get_property(CDXPROP_ATOM_BOND_ORDERING)
            .and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());

        // Attachments
        let attachments = raw
            .get_property(CDXPROP_NODE_ATTACHMENTS)
            .and_then(|v| crate::cdx::binary_codec::decode_u32_array(v).ok());

        // Generic nickname
        let generic_nickname = raw
            .get_property(CDXPROP_ATOM_GENERIC_NICKNAME)
            .and_then(|v| CDXString::decode(v).ok());

        // Alt group ID
        let alt_group_id = raw
            .get_property(CDXPROP_ATOM_ALT_GROUP_ID)
            .and_then(|v| u32::decode(v).ok());

        // Restrict substituents up to
        let restrict_substituents_up_to = raw
            .get_property(CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_UP_TO)
            .and_then(|v| u8::decode(v).ok());

        // Restrict substituents exactly
        let restrict_substituents_exactly = raw
            .get_property(CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_EXACTLY)
            .and_then(|v| u8::decode(v).ok());

        // CIP stereochemistry
        let cip_stereochemistry = raw
            .get_property(CDXPROP_ATOM_CIP_STEREOCHEMISTRY)
            .and_then(|v| i8::decode(v).ok());

        // Atom translation
        let atom_translation = raw
            .get_property(CDXPROP_ATOM_TRANSLATION)
            .and_then(|v| i8::decode(v).ok());

        // Atom number
        let atom_number = raw
            .get_property(CDXPROP_ATOM_NUMBER)
            .and_then(|v| u16::decode(v).ok());

        // Show query
        let show_query = raw
            .get_property(CDXPROP_ATOM_SHOW_QUERY)
            .and_then(|v| bool::decode(v).ok());

        // Show stereo
        let show_stereo = raw
            .get_property(CDXPROP_ATOM_SHOW_STEREO)
            .and_then(|v| bool::decode(v).ok());

        // Show atom number
        let show_atom_number = raw
            .get_property(CDXPROP_ATOM_SHOW_ATOM_NUMBER)
            .and_then(|v| bool::decode(v).ok());

        // Link count low
        let link_count_low = raw
            .get_property(CDXPROP_ATOM_LINK_COUNT_LOW)
            .and_then(|v| i16::decode(v).ok());

        // Link count high
        let link_count_high = raw
            .get_property(CDXPROP_ATOM_LINK_COUNT_HIGH)
            .and_then(|v| i16::decode(v).ok());

        // Isotopic abundance
        let isotopic_abundance = raw
            .get_property(CDXPROP_ATOM_ISOTOPIC_ABUNDANCE)
            .and_then(|v| f64::decode(v).ok());

        // External connection type
        let external_connection_type = raw
            .get_property(CDXPROP_ATOM_EXTERNAL_CONNECTION_TYPE)
            .and_then(|v| i8::decode(v).ok());

        // Generic list
        let generic_list = raw
            .get_property(CDXPROP_ATOM_GENERIC_LIST)
            .and_then(|v| CDXString::decode(v).ok());

        // Show enhanced stereo
        let show_enhanced_stereo = raw
            .get_property(CDXPROP_ATOM_SHOW_ENHANCED_STEREO)
            .and_then(|v| bool::decode(v).ok());

        // Enhanced stereo type
        let enhanced_stereo_type = raw
            .get_property(CDXPROP_ATOM_ENHANCED_STEREO_TYPE)
            .and_then(|v| i8::decode(v).ok());

        // Enhanced stereo group num
        let enhanced_stereo_group_num = raw
            .get_property(CDXPROP_ATOM_ENHANCED_STEREO_GROUP_NUM)
            .and_then(|v| u16::decode(v).ok());

        // Line width
        let line_width = raw
            .get_property(CDXPROP_LINE_WIDTH)
            .and_then(|v| f64::decode(v).ok());

        // Label font
        let label_font = raw
            .get_property(CDXPROP_LABEL_FONT)
            .and_then(|v| i16::decode(v).ok());

        // Label size
        let label_size = raw
            .get_property(CDXPROP_LABEL_SIZE)
            .and_then(|v| i16::decode(v).ok());

        // Label face
        let label_face = raw
            .get_property(CDXPROP_LABEL_FACE)
            .and_then(|v| i16::decode(v).ok());

        Ok(Node {
            id: raw.id,
            z_order,
            ignore_warnings,
            chemical_warning,
            visible,
            position_2d,
            position_3d,
            foreground_color,
            background_color,
            node_type,
            label_display,
            element,
            isotope,
            charge,
            radical,
            restrict_free_sites,
            restrict_implicit_h,
            restrict_ring_bond_count,
            restrict_unsaturated_bonds,
            restrict_rxn_change,
            restrict_rxn_stereo,
            abnormal_valence,
            num_hydrogens,
            h_dot,
            h_dash,
            geometry,
            bond_ordering,
            attachments,
            generic_nickname,
            alt_group_id,
            restrict_substituents_up_to,
            restrict_substituents_exactly,
            cip_stereochemistry,
            atom_translation,
            atom_number,
            show_query,
            show_stereo,
            show_atom_number,
            link_count_low,
            link_count_high,
            isotopic_abundance,
            external_connection_type,
            generic_list,
            show_enhanced_stereo,
            enhanced_stereo_type,
            enhanced_stereo_group_num,
            line_width,
            label_font,
            label_size,
            label_face,
        })
    }

    fn to_raw(&self) -> Result<RawCdxObject, CdxError> {
        let mut properties = Vec::new();

        // Optional properties
        if let Some(val) = self.z_order {
            properties.push(RawCdxProperty {
                tag: CDXPROP_Z_ORDER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.ignore_warnings {
            properties.push(RawCdxProperty {
                tag: CDXPROP_IGNORE_WARNINGS,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.chemical_warning {
            properties.push(RawCdxProperty {
                tag: CDXPROP_CHEMICAL_WARNING,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.visible {
            properties.push(RawCdxProperty {
                tag: CDXPROP_VISIBLE,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.position_2d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_2D_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.position_3d {
            properties.push(RawCdxProperty {
                tag: CDXPROP_3D_POSITION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.foreground_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_FOREGROUND_COLOR,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.background_color {
            properties.push(RawCdxProperty {
                tag: CDXPROP_BACKGROUND_COLOR,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.node_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NODE_TYPE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_display {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NODE_LABEL_DISPLAY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.element {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NODE_ELEMENT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.isotope {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ISOTOPE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.charge {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_CHARGE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.radical {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RADICAL,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_free_sites {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_FREE_SITES,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_implicit_h {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_IMPLICIT_H,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_ring_bond_count {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_RING_BOND_COUNT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_unsaturated_bonds {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_UNSATURATED_BONDS,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_rxn_change {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_RXN_CHANGE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_rxn_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_RXN_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.abnormal_valence {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ABNORMAL_VALENCE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.num_hydrogens {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_NUM_HYDROGENS,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.h_dot {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_H_DOT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.h_dash {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_H_DASH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.geometry {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_GEOMETRY,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.bond_ordering {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_BOND_ORDERING,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
            });
        }
        if let Some(ref val) = self.attachments {
            properties.push(RawCdxProperty {
                tag: CDXPROP_NODE_ATTACHMENTS,
                value: crate::cdx::binary_codec::encode_u32_array(val)?,
            });
        }
        if let Some(ref val) = self.generic_nickname {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_GENERIC_NICKNAME,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.alt_group_id {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ALT_GROUP_ID,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_substituents_up_to {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_UP_TO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.restrict_substituents_exactly {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_RESTRICT_SUBSTITUENTS_EXACTLY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.cip_stereochemistry {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_CIP_STEREOCHEMISTRY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.atom_translation {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_TRANSLATION,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.atom_number {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_NUMBER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.show_query {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_QUERY,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.show_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.show_atom_number {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_ATOM_NUMBER,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.link_count_low {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_LINK_COUNT_LOW,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.link_count_high {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_LINK_COUNT_HIGH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.isotopic_abundance {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ISOTOPIC_ABUNDANCE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.external_connection_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_EXTERNAL_CONNECTION_TYPE,
                value: val.encode()?,
            });
        }
        if let Some(ref val) = self.generic_list {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_GENERIC_LIST,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.show_enhanced_stereo {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_SHOW_ENHANCED_STEREO,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.enhanced_stereo_type {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ENHANCED_STEREO_TYPE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.enhanced_stereo_group_num {
            properties.push(RawCdxProperty {
                tag: CDXPROP_ATOM_ENHANCED_STEREO_GROUP_NUM,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.line_width {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LINE_WIDTH,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_font {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FONT,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_size {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_SIZE,
                value: val.encode()?,
            });
        }
        if let Some(val) = self.label_face {
            properties.push(RawCdxProperty {
                tag: CDXPROP_LABEL_FACE,
                value: val.encode()?,
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
