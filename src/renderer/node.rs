use crate::cdx::node::Node;
use crate::renderer::{Drawable, RenderContext, element_to_symbol};
use eframe::egui;

impl Drawable for Node {
    fn draw(&self, ctx: &RenderContext) {
        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);
            let radius = 10.0; // Default atom radius in pixels

            // Determine color
            let color = match self.foreground_color {
                Some(color_idx) => ctx
                    .document
                    .get_color_table()
                    .and_then(|ct| ct.get(color_idx as usize))
                    .map(|c| c.to_color32())
                    .unwrap_or(egui::Color32::GREEN),
                None => egui::Color32::YELLOW,
            };

            // Draw circle for atom
            ctx.painter
                .add(egui::Shape::circle_filled(screen_pos, radius, color));

            // Draw element label
            if let Some(element) = self.element {
                let label = element_to_symbol(element);
                ctx.draw_text(
                    &label,
                    screen_pos,
                    egui::Color32::WHITE,
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
