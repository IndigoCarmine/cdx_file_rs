use crate::cdx::crossing_bond::CrossingBond;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for CrossingBond {
    fn draw(&self, _ctx: &RenderContext) {
        // CrossingBond is a metadata object that references a Bond that crosses bracket boundaries
        // The actual bond rendering is handled by the Bond object referenced in crossing_bond_id
        // This object provides metadata about which end of the bond is inside/outside the bracket
    }
}
