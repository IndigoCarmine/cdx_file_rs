use crate::cdx::registry_number::RegistryNumber;
use crate::renderer::Drawable;

impl Drawable for RegistryNumber {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        // RegistryNumber is a metadata object storing registry/CAS numbers
        // It has no visual representation
    }
}
