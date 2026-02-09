use crate::cdx::template_grid::TemplateGrid;
use crate::cdx_tags::template_grid_tags::*;
use crate::impl_tagged_object;

impl_tagged_object!(TemplateGrid, CDXOBJ_TEMPLATE_GRID, {
    extent_2d: CDXPROP_2D_EXTENT,
    template_pane_height: CDXPROP_TEMPLATE_PANE_HEIGHT,
    template_num_rows: CDXPROP_TEMPLATE_NUM_ROWS,
    template_num_columns: CDXPROP_TEMPLATE_NUM_COLUMNS,
});
