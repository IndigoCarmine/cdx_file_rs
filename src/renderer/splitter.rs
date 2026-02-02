use crate::cdx::splitter::Splitter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Splitter {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Splitter is a UI element for dividing panes
        // May not need rendering in a document viewer context
    }
}
