use crate::cdx::spectrum::Spectrum;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Spectrum {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &crate::renderer::RenderContext<P>) {
        // Spectrum objects represent spectroscopic data
        // TODO: Implement spectrum rendering (NMR, IR, MS plots)
        // This would require plotting spectral data points
    }
}
