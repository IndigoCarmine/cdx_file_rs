use crate::cdx::tlc_lane::TlcLane;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TlcLane {
    fn draw<P: AbstractPainter>(&self, _ctx: &RenderContext<P>) {}
}
