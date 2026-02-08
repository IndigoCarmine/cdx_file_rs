use crate::cdx::reaction_step::ReactionStep;
use crate::renderer::Drawable;

impl Drawable for ReactionStep {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // ReactionStep is a container/metadata object
        // The actual rendering is done by its child objects (reactants, products, arrows, etc.)
    }
}
