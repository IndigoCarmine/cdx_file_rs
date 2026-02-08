use crate::cdx::geometry::Geometry;
use crate::cdx::values::Rectangle;
use crate::renderer::core::Drawable;

impl Drawable for Geometry {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // Geometry is a metadata object representing relationships
        // It doesn't have visual representation itself
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
