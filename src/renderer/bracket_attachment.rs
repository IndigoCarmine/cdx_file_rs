use crate::cdx::bracket_attachment::BracketAttachment;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for BracketAttachment {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // BracketAttachment is a metadata object that references a Graphic object
        // The actual bracket visual is drawn by the Graphic object referenced in bracket_graphic_id
        // This object mainly serves to establish the relationship between the bracketed group
        // and the graphical bracket representation
    }
}
