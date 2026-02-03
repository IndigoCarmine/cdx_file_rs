use crate::cdx::color_table::ColorTable;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ColorTable {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // ColorTable is a metadata object containing the document's color palette
        // It has no visual representation - colors are referenced by other objects
    }
}
