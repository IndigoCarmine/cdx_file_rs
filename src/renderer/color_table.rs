use crate::cdx::color_table::ColorTable;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ColorTable {
    fn draw(&self, _ctx: &RenderContext) {
        // ColorTable is a metadata object containing the document's color palette
        // It has no visual representation - colors are referenced by other objects
    }
}
