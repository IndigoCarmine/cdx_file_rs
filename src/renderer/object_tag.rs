use crate::cdx::object_tag::ObjectTag;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ObjectTag {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // ObjectTag is a metadata object
        // The actual rendering is done by contained Text objects
    }
}
