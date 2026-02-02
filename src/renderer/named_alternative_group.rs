use crate::cdx::named_alternative_group::NamedAlternativeGroup;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for NamedAlternativeGroup {
    fn draw(&self, _ctx: &RenderContext) {
        // NamedAlternativeGroup is a container object for alternative molecular representations
        // The actual rendering is handled by child Fragment objects
    }
}
