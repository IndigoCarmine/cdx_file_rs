use crate::cdx::group::Group;
use crate::renderer::renderer::RenderContext;
use crate::renderer::renderer::Drawable;

impl Drawable for Group {
    fn draw(&self, _ctx: &RenderContext) {
    }
}
