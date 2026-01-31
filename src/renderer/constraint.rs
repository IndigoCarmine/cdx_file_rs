use crate::cdx::constraint::Constraint;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Constraint {
    fn draw(&self, _ctx: &RenderContext) {
        // Constraint is a metadata object representing constraints between objects
        // It doesn't have visual representation itself
    }
}
