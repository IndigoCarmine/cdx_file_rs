// Document rendering is handled by traversing the tree in CdxRenderer::render_all
// Documents are the root node and don't have direct visual representation
use crate::cdx::document::Document;
use crate::renderer::core::Drawable;
use crate::renderer::core::RenderContext;

impl Drawable for Document {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &RenderContext<P>) {}
}
