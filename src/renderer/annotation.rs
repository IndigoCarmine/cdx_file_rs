use crate::cdx::annotation::Annotation;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Annotation {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Border rendering - placeholder implementation
        // Borders are metadata objects that don't need rendering
    }
}
