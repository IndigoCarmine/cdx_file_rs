use crate::cdx::bond::Bond;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;

impl Drawable for Bond {
    fn draw(&self, _ctx: &RenderContext) {
        if let Some(false) = self.visible {
            return;
        }

        let start = match _ctx.node_position(self.begin) {
            Some(pos) => pos,
            None => return,
        };

        let end = match _ctx.node_position(self.end) {
            Some(pos) => pos,
            None => return,
        };

        let p1 = _ctx.cdx_to_screen(start);
        let p2 = _ctx.cdx_to_screen(end);

        let color = match self.foreground_color {
            Some(idx) => _ctx
                .document
                .get_color_table()
                .and_then(|ct| ct.get(idx as usize))
                .map(|c| c.to_color32())
                .unwrap_or(egui::Color32::BLACK),
            None => egui::Color32::BLACK,
        };

        let line_width = self.line_width.unwrap_or(_ctx.default_line_width()) as f32;
        let stroke = egui::Stroke::new(line_width.max(0.5), color);

        let order = self.bond_order.unwrap_or(1);

        if order <= 1 {
            _ctx.painter.line_segment([p1, p2], stroke);
            return;
        }

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return;
        }

        let nx = -dy / len;
        let ny = dx / len;

        let spacing = self.bond_spacing.unwrap_or(10) as f32;

        if order == 2 {
            let ox = nx * spacing * 0.5;
            let oy = ny * spacing * 0.5;
            let a1 = egui::Pos2 {
                x: p1.x + ox,
                y: p1.y + oy,
            };
            let a2 = egui::Pos2 {
                x: p2.x + ox,
                y: p2.y + oy,
            };
            let b1 = egui::Pos2 {
                x: p1.x - ox,
                y: p1.y - oy,
            };
            let b2 = egui::Pos2 {
                x: p2.x - ox,
                y: p2.y - oy,
            };
            _ctx.painter.line_segment([a1, a2], stroke);
            _ctx.painter.line_segment([b1, b2], stroke);
        } else {
            _ctx.painter.line_segment([p1, p2], stroke);
            let ox = nx * spacing;
            let oy = ny * spacing;
            let a1 = egui::Pos2 {
                x: p1.x + ox,
                y: p1.y + oy,
            };
            let a2 = egui::Pos2 {
                x: p2.x + ox,
                y: p2.y + oy,
            };
            let b1 = egui::Pos2 {
                x: p1.x - ox,
                y: p1.y - oy,
            };
            let b2 = egui::Pos2 {
                x: p2.x - ox,
                y: p2.y - oy,
            };
            _ctx.painter.line_segment([a1, a2], stroke);
            _ctx.painter.line_segment([b1, b2], stroke);
        }
    }
}
