// Page rendering is handled by traversing the tree in CdxRenderer::render_all
// Pages themselves have no visual representation, only their children

use crate::cdx::page::Page;
use crate::cdx::values::Rectangle;
use crate::renderer::core::Drawable;

impl Drawable for Page {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
    }
    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
