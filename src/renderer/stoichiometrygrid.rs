use crate::cdx::stoichiometrygrid::StoichiometryGrid;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for StoichiometryGrid {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>) {
        // if let Some(pos) = &self.position_2d {
        //     let screen_pos = ctx.cdx_to_screen(pos);
        //     // Draw a simple marker for the grid (customize as needed)
        //     ctx.painter.circle_stroke(screen_pos, 10.0, ctx.default_stroke());
        // }
    }
}
