use crate::cdx::splitter::Splitter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Splitter {
    fn draw(&self, _ctx: &RenderContext) {
        // Splitter is a UI element for dividing panes
        // May not need rendering in a document viewer context
    }
}
