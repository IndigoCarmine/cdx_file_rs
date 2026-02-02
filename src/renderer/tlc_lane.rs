use crate::cdx::tlc_lane::TlcLane;
use crate::renderer::core::Drawable;
use crate::renderer::core::RenderContext;

impl Drawable for TlcLane {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {}
}
