use crate::cdx::geometry::Geometry;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Geometry {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Geometry is a metadata object representing relationships
        // It doesn't have visual representation itself
    }
}
