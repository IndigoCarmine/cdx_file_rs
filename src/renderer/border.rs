use crate::cdx::border::Border;
use crate::cdx::values::Rectangle;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Border {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // Border rendering - placeholder implementation
        // Borders are metadata objects that don't need rendering
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        None
    }
}
