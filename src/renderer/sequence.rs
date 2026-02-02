use crate::cdx::sequence::Sequence;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Sequence {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Sequence objects represent biological sequences (DNA, RNA, proteins)
        // TODO: Implement sequence rendering if visual representation is needed
        // Typically sequences are rendered by child objects or external renderers
    }
}
