use crate::cdx::group::Group;
use crate::cdx::values::Rectangle;
use crate::renderer::core::Drawable;

impl Drawable for Group {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
