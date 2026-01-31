use crate::cdx::reaction_step::ReactionStep;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ReactionStep {
    fn draw(&self, _ctx: &RenderContext) {
        // ReactionStep is a container/metadata object
        // The actual rendering is done by its child objects (reactants, products, arrows, etc.)
    }
}
