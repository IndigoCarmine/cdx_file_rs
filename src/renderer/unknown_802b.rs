use crate::cdx::unknown_802b::UnknownObject802B;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for UnknownObject802B {
    fn draw(&self, _ctx: &RenderContext) {
        // Unknown object - no rendering
    }
}
