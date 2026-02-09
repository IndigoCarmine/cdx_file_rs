use crate::cdx::constraint::Constraint;
use crate::cdx_tags::constraint_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Constraint, CDXOBJ_CONSTRAINT, {
    name: CDXPROP_NAME,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    bond_length: CDXPROP_BOND_LENGTH,
    line_width: CDXPROP_LINE_WIDTH,
    hash_spacing: CDXPROP_HASH_SPACING,
    label_style_font: CDXPROP_LABEL_STYLE_FONT,
    label_style_size: CDXPROP_LABEL_STYLE_SIZE,
    label_style_face: CDXPROP_LABEL_STYLE_FACE,
    label_style_color: CDXPROP_LABEL_STYLE_COLOR,
    basis_objects: CDXPROP_BASIS_OBJECTS,
    constraint_type: CDXPROP_CONSTRAINT_TYPE,
    constraint_min: CDXPROP_CONSTRAINT_MIN,
    constraint_max: CDXPROP_CONSTRAINT_MAX,
    ignore_unconnected_atoms: CDXPROP_IGNORE_UNCONNECTED_ATOMS,
    dihedral_is_chiral: CDXPROP_DIHEDRAL_IS_CHIRAL,
    visible: CDXPROP_VISIBLE,
    z_order: CDXPROP_Z_ORDER,
    bounding_box: CDXPROP_BOUNDING_BOX,
});
