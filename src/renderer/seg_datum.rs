use crate::cdx::seg_datum::SegDatum;
use crate::renderer::{Drawable, RenderContext, backend::AbstractPainter};

impl Drawable for SegDatum {
    fn draw<P: AbstractPainter>(&self, _ctx: &RenderContext<P>) {
        // SegDatum is mostly metadata, not visual
    }
}
