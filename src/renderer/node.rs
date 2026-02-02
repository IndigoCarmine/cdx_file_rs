use crate::cdx::node::Node;
use crate::renderer::{Drawable, RenderContext, element_to_symbol, backend::Color};
use crate::renderer::backend::Point2d as BackendPoint2d;

impl Drawable for Node {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &RenderContext<P>) {
        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);
            let radius = ctx.style.default_atom_radius;

            // Determine color - use object color or fallback to document foreground color
            let forground_color =
                ctx.resolve_color(self.foreground_color, ctx.default_foreground_color());
            let background_color =
                ctx.resolve_color_i16(self.background_color, ctx.default_background_color());

            // Draw circle for atom
            ctx.painter.circle_filled(screen_pos, radius, background_color);

            // Draw element label
            if let Some(element) = self.element {
                let label = element_to_symbol(element);
                ctx.draw_text(
                    &label,
                    screen_pos,
                    forground_color,
                    ctx.default_label_size(),
                );
            }

            // Draw charge if present
            if let Some(charge) = self.charge {
                if charge != 0 {
                    let charge_str = format!("{:+}", charge);
                    let charge_pos = BackendPoint2d::new(
                        screen_pos.x + radius + ctx.style.charge_label_offset,
                        screen_pos.y - radius - ctx.style.charge_label_offset,
                    );
                    ctx.draw_text(
                        &charge_str,
                        charge_pos,
                        Color::RED,
                        ctx.style.charge_label_size,
                    );
                }
            }
        }
    }
}
