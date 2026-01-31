use crate::cdx::object_tag::ObjectTag;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ObjectTag {
    fn draw(&self, _ctx: &RenderContext) {
        // ObjectTag is a metadata object
        // The actual rendering is done by contained Text objects
    }
}
