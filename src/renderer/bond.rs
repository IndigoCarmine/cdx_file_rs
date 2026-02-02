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
                .unwrap_or_else(|| _ctx.default_foreground_color()),
            None => _ctx.default_foreground_color(),
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
        let spacing_default = _ctx.default_bond_spacing() ;
        let bond_length_default = _ctx.default_bond_length();
        let spacing = self.bond_spacing.unwrap_or(spacing_default) as f32;
        let scale = _ctx.zoom * _ctx.auto_scale;

        if order == 2 {
            // Double bond: two parallel lines with position determined by double_position property
            // Position values: 0/256=Center, 1/257=Right, 2/258=Left
            let position = self.double_position.unwrap_or(1) % 256;

            let (a1, a2, b1, b2) = match position {
                0 => {
                    // Center: two lines symmetrically offset from the center
                    let half_offset = spacing * 0.5 * scale;
                    let ox = nx * half_offset;
                    let oy = ny * half_offset;
                    (
                        egui::Pos2 { x: p1.x + ox, y: p1.y + oy },
                        egui::Pos2 { x: p2.x + ox, y: p2.y + oy },
                        egui::Pos2 { x: p1.x - ox, y: p1.y - oy },
                        egui::Pos2 { x: p2.x - ox, y: p2.y - oy },
                    )
                },
                1 => {
                    // Right: center line + one line on the right side
                    let offset = spacing * scale;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    (
                        p1,  // Center line start
                        p2,  // Center line end
                        egui::Pos2 { x: p1.x + ox, y: p1.y + oy },
                        egui::Pos2 { x: p2.x + ox, y: p2.y + oy },
                    )
                },
                2 => {
                    // Left: center line + one line on the left side
                    let offset = spacing * scale;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    (
                        p1,  // Center line start
                        p2,  // Center line end
                        egui::Pos2 { x: p1.x - ox, y: p1.y - oy },
                        egui::Pos2 { x: p2.x - ox, y: p2.y - oy },
                    )
                },
                _ => {
                    // Default to center for unknown values
                    let half_offset = spacing * 0.5 * scale;
                    let ox = nx * half_offset;
                    let oy = ny * half_offset;
                    (
                        egui::Pos2 { x: p1.x + ox, y: p1.y + oy },
                        egui::Pos2 { x: p2.x + ox, y: p2.y + oy },
                        egui::Pos2 { x: p1.x - ox, y: p1.y - oy },
                        egui::Pos2 { x: p2.x - ox, y: p2.y - oy },
                    )
                }
            };

            _ctx.painter.line_segment([a1, a2], stroke);
            _ctx.painter.line_segment([b1, b2], stroke);
        } else {
            // Triple bond: center line + two outer lines with full spacing
            _ctx.painter.line_segment([p1, p2], stroke);
            let offset = spacing * scale;
            let ox = nx * offset;
            let oy = ny * offset;
            
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
