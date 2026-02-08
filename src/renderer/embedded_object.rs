use crate::cdx::embedded_object::EmbeddedObject;
use crate::cdx::values::Rectangle;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for EmbeddedObject {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // EmbeddedObject represents external embedded data (OLE objects, images, etc.)
        // TODO: Implement rendering for embedded images and objects
        // This would require decoding embedded data and rendering it at the specified position
    }
    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
