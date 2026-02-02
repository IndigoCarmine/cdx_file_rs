use crate::cdx::cross_reference::CrossReference;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for CrossReference {
    fn draw(&self, _ctx: &RenderContext) {
        // CrossReference is a metadata object for referencing external documents
        // It has no visual representation
    }
}
