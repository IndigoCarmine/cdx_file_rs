use crate::cdx::cross_reference::CrossReference;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for CrossReference {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // CrossReference is a metadata object for referencing external documents
        // It has no visual representation
    }
}
