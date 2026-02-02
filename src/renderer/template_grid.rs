use crate::cdx::template_grid::TemplateGrid;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TemplateGrid {
    fn draw(&self, _ctx: &RenderContext) {
        // TemplateGrid is a UI element for template organization
        // May not need rendering in a document viewer context
    }
}
