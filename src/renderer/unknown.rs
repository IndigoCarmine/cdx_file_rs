use crate::cdx::unknown::*;
use crate::renderer::{Drawable, RenderContext};

macro_rules! generate_render {
    ( $name:ident ) => {
        impl Drawable for $name {
            fn draw<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
                // Unknown objects have no defined rendering behavior
            }
        }
    };
}

generate_render!(UnknownObject801D);
generate_render!(UnknownObject802B);
generate_render!(UnknownObject801E);
generate_render!(UnknownObject801F);
