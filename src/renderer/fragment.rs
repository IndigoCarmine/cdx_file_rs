// Fragment rendering is handled by traversing the tree in CdxRenderer::render_all
// Fragments themselves have no visual representation, only their children (nodes and bonds)

use crate::cdx::fragment::Fragment;
use crate::cdx::values::Rectangle;
use crate::renderer::core::Drawable;

impl Drawable for Fragment {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
    }
    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
