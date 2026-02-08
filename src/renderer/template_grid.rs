use crate::cdx::template_grid::TemplateGrid;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TemplateGrid {
    fn draw<P: AbstractPainter>(&self, _ctx: &RenderContext<P>) {
        // TemplateGrid is a UI element for template organization
        // May not need rendering in a document viewer context
    }
}
