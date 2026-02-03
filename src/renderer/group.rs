use crate::cdx::group::Group;
use crate::renderer::core::Drawable;
use crate::renderer::core::RenderContext;

impl Drawable for Group {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {}
}
