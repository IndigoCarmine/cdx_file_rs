use crate::cdx::registry_number::RegistryNumber;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for RegistryNumber {
    fn draw(&self, _ctx: &RenderContext) {
        // RegistryNumber is a metadata object storing registry/CAS numbers
        // It has no visual representation
    }
}
