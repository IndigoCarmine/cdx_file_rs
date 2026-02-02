use crate::cdx::unknown_802b::UnknownObject802B;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for UnknownObject802B {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Unknown object - no rendering
    }
}
