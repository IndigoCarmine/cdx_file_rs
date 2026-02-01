// Document rendering is handled by traversing the tree in CdxRenderer::render_all
// Documents are the root node and don't have direct visual representation
use crate::cdx::document::Document;
use crate::renderer::core::RenderContext;
use crate::renderer::core::Drawable;

impl Drawable for Document {
    fn draw(&self, _ctx: &RenderContext) {
    }
}
