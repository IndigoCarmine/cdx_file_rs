use crate::cdx::segcomponent::SegComponent;
use crate::cdx::values::Rectangle;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for SegComponent {
    fn draw<P: AbstractPainter>(&self, _ctx: &RenderContext<P>) {}

    fn draw_with_node<P: AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        node: &dendron::Node<crate::cdx::file::NodePayload>,
    ) {
        if let Some(mut rect) = ctx.accumulate_children_bounding_box(node) {
            // Normalize height: use max bottom / min top across all sibling SegComponents
            if let Some(parent) = node.parent() {
                for sibling in parent.children() {
                    if let Some(sib_bbox) = ctx.accumulate_children_bounding_box(&sibling) {
                        if sib_bbox.bottom > rect.bottom { rect.bottom = sib_bbox.bottom; }
                        if sib_bbox.top < rect.top { rect.top = sib_bbox.top; }
                    }
                }
            }
            // Use CDX width property (0x0812) for correct column width
            if let Some(raw_width) = self.width {
                rect.right = rect.left + raw_width as f64;
            }
            let bbox = rect.to_backend_rect();
            let screen_rect = ctx.cdx_rect_to_screen(&bbox);
            ctx.painter.rect_stroke(screen_rect, 0.0, ctx.default_stroke());
        }
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        None
    }
}
