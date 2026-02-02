use crate::cdx::chemical_property::ChemicalProperty;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for ChemicalProperty {
    fn draw(&self, _ctx: &RenderContext) {
        // ChemicalProperty is a metadata object storing chemical information
        // It has no visual representation
    }
}
