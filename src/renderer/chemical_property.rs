use crate::cdx::chemical_property::ChemicalProperty;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ChemicalProperty {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // ChemicalProperty is a metadata object storing chemical information
        // It has no visual representation
    }
}
