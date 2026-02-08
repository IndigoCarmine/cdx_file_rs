use crate::cdx::named_alternative_group::NamedAlternativeGroup;
use crate::cdx::values::Rectangle;
use crate::renderer::Drawable;

impl Drawable for NamedAlternativeGroup {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // NamedAlternativeGroup is a container object for alternative molecular representations
        // The actual rendering is handled by child Fragment objects
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
