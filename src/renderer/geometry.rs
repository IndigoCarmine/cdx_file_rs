use crate::cdx::geometry::Geometry;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Geometry {
    fn draw(&self, _ctx: &RenderContext) {
        // Geometry is a metadata object representing relationships
        // It doesn't have visual representation itself
    }
}
