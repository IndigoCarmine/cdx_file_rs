use crate::cdx::spectrum::Spectrum;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Spectrum {
    fn draw(&self, _ctx: &RenderContext) {
        // Spectrum objects represent spectroscopic data
        // TODO: Implement spectrum rendering (NMR, IR, MS plots)
        // This would require plotting spectral data points
    }
}
