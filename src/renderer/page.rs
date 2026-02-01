// Page rendering is handled by traversing the tree in CdxRenderer::render_all
// Pages themselves have no visual representation, only their children

use crate::cdx::page::Page;
use crate::renderer::core::Drawable;
use crate::renderer::core::RenderContext;

impl Drawable for Page {
    fn draw(&self, _ctx: &RenderContext) {}
}
