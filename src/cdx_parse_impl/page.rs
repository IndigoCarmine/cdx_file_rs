use crate::cdx::page::Page;
use crate::cdx_tags::page_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(Page, CDXOBJ_PAGE, {
    bounding_box: CDXPROP_BOUNDING_BOX,
    z_order: CDXPROP_Z_ORDER,
    ignore_warnings: CDXPROP_IGNORE_WARNINGS,
    chemical_warning: CDXPROP_CHEMICAL_WARNING,
    visible: CDXPROP_VISIBLE,
    foreground_color: CDXPROP_FOREGROUND_COLOR,
    background_color: CDXPROP_BACKGROUND_COLOR,
    width_pages: CDXPROP_WIDTH_PAGES,
    height_pages: CDXPROP_HEIGHT_PAGES,
    drawing_space_type: CDXPROP_DRAWING_SPACE_TYPE,
    width: CDXPROP_WIDTH,
    height: CDXPROP_HEIGHT,
    page_overlap: CDXPROP_PAGE_OVERLAP,
    header: CDXPROP_HEADER,
    header_position: CDXPROP_HEADER_POSITION,
    footer: CDXPROP_FOOTER,
    footer_position: CDXPROP_FOOTER_POSITION,
    print_trim_marks: CDXPROP_PRINT_TRIM_MARKS,
    splitter_positions: CDXPROP_SPLITTER_POSITIONS,
    page_definition: CDXPROP_PAGE_DEFINITION,
    bounds_in_parent: CDXPROP_BOUNDS_IN_PARENT,
});
