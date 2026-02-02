use crate::cdx::node::Node;
use crate::renderer::{Drawable, RenderContext, element_to_symbol};
use eframe::egui;

impl Drawable for Node {
    fn draw(&self, ctx: &RenderContext) {
        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);
            let radius = 10.0; // Default atom radius in pixels

            // Determine color - use object color or fallback to document foreground color
            let forground_color = match self.foreground_color {
                Some(color_idx) => ctx
                    .document
                    .get_color_table()
                    .and_then(|ct| ct.get(color_idx as usize))
                    .map(|c| c.to_color32())
                    .unwrap_or_else(||ctx.default_foreground_color()),
                None => ctx.default_foreground_color(),
            };
            let background_color = match self.background_color {
                Some(color_idx) => ctx
                    .document
                    .get_color_table()
                    .and_then(|ct| ct.get(color_idx as usize))
                    .map(|c| c.to_color32())
                    .unwrap_or_else(||ctx.default_background_color()),
                None => ctx.default_background_color(),
            };

            // Draw circle for atom
            ctx.painter
                .add(egui::Shape::circle_filled(screen_pos, radius, background_color));

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
                    let charge_pos = egui::Pos2 {
                        x: screen_pos.x + radius + 8.0,
                        y: screen_pos.y - radius - 8.0,
                    };
                    ctx.draw_text(&charge_str, charge_pos, egui::Color32::RED, 8.0);
                }
            }
        }
    }
}
