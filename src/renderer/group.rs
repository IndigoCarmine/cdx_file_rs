use crate::cdx::group::Group;
use crate::renderer::core::Drawable;
use crate::renderer::core::RenderContext;

impl Drawable for Group {
    fn draw(&self, _ctx: &RenderContext) {}
}
