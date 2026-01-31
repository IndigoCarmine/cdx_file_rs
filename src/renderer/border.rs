use crate::cdx::border::Border;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Border {
    fn draw(&self, _ctx: &RenderContext) {
        // Border rendering - placeholder implementation
        // Borders are metadata objects that don't need rendering
    }
}
