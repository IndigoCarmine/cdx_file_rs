use crate::cdx::embedded_object::EmbeddedObject;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for EmbeddedObject {
    fn draw(&self, _ctx: &RenderContext) {
        // EmbeddedObject represents external embedded data (OLE objects, images, etc.)
        // TODO: Implement rendering for embedded images and objects
        // This would require decoding embedded data and rendering it at the specified position
    }
}
