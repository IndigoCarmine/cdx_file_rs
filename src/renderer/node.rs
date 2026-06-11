use crate::cdx::file::NodePayload;
use crate::cdx::node::Node;
use crate::renderer::backend::Color;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for Node {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, _ctx: &RenderContext<P>) {
        // Atom rendering is handled by draw_with_node (needs child inspection).
    }

    /// Draw a white background circle at the atom position when a TextObject child exists.
    /// Called in layer 1 (between bonds and text labels) to occlude bond lines at junctions.
    fn draw_with_node<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        node: &dendron::Node<NodePayload>,
    ) {
        // Find the first TextObject child to determine font size for circle radius
        let font_size_pts = node.children().find_map(|child| {
            if let NodePayload::TextObject(t) = &*child.borrow_data() {
                // Prefer style run font size, then label_size, then caption_size
                if let Some(ref cdx_str) = t.text {
                    if let Some(run) = cdx_str.style_runs.first() {
                        return Some(run.font_size as f32 / 20.0);
                    }
                }
                t.label_size
                    .map(|s| s as f32 / 20.0)
                    .or_else(|| t.caption_size.map(|s| s as f32 / 20.0))
            } else {
                None
            }
        });

        let Some(font_size_pts) = font_size_pts else {
            return; // No text label → no circle needed
        };

        if let Some(ref pos) = self.position_2d {
            let cdx_pos = pos.to_backend_point();
            let screen_pos = ctx.cdx_to_screen(&cdx_pos);
            // Radius covers the capital-letter height (≈ 0.7× font size) plus a small margin
            let radius = font_size_pts * ctx.zoom * ctx.auto_scale * 0.75;
            ctx.painter.circle_filled(screen_pos, radius, Color::WHITE);
        }
    }
}
