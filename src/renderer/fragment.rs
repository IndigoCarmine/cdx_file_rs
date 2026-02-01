// Fragment rendering is handled by traversing the tree in CdxRenderer::render_all
// Fragments themselves have no visual representation, only their children (nodes and bonds)

use crate::cdx::fragment::Fragment;
use crate::renderer::core::RenderContext;
use crate::renderer::core::Drawable;

impl Drawable for Fragment {
    fn draw(&self, _ctx: &RenderContext) {
    }
}
