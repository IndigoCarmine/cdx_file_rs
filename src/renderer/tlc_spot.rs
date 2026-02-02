use crate::cdx::tlc_spot::TLCSpot;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TLCSpot {
    fn draw(&self, _ctx: &RenderContext) {
        // TLCSpot represents a spot on a TLC plate
        // TODO: Implement TLC spot rendering based on position and RF value
        // Typically rendered as a circle or ellipse at the appropriate position
    }
}
