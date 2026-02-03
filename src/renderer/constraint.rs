use crate::cdx::constraint::Constraint;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Constraint {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Constraint is a metadata object representing constraints between objects
        // It doesn't have visual representation itself
    }
}
