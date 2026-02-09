use crate::cdx::segcomponent::SegComponent;
use crate::cdx::values::Rectangle;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for SegComponent {
    fn draw<P: AbstractPainter>(&self, _ctx: &RenderContext<P>) {
        // SegComponent as a container relies on draw_with_node to access its children in the tree.
    }

    fn draw_with_node<P: AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        node: &dendron::Node<crate::cdx::file::NodePayload>,
    ) {
        let bbox = ctx.accumulate_children_bounding_box(node);
        if let Some(bbox_inner) = bbox {
            let screen_rect = ctx.cdx_rect_to_screen(&bbox_inner);
            ctx.painter
                .rect_stroke(screen_rect, 0.0, ctx.default_stroke());
        }
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        None
    }
}
