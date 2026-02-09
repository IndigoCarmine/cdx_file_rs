use crate::cdx::bond::Bond;
use crate::cdx::values::Rectangle;
use crate::renderer::{
    Drawable, RenderContext,
    backend::{Point2d as BackendPoint2d, Stroke},
};

impl Drawable for Bond {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        _ctx: &crate::renderer::RenderContext<P>,
    ) {
        if let Some(false) = self.visible {
            return;
        }

        let begin_id = match self.begin {
            Some(id) => id,
            None => return,
        };
        let end_id = match self.end {
            Some(id) => id,
            None => return,
        };

        let start = match _ctx.node_position(begin_id) {
            Some(pos) => pos,
            None => return,
        };

        let end = match _ctx.node_position(end_id) {
            Some(pos) => pos,
            None => return,
        };

        let p1 = _ctx.cdx_to_screen(start);
        let p2 = _ctx.cdx_to_screen(end);

        let color = _ctx.resolve_color(self.foreground_color, _ctx.default_foreground_color());

        let line_width = self.line_width.unwrap_or(_ctx.default_line_width()) as f32;
        let stroke = Stroke::new(line_width.max(0.5), color);

        let order = self.bond_order.unwrap_or(1);

        if order <= 1 {
            _ctx.painter.line_segment(p1, p2, stroke);
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

        // Calculate the actual spacing between double bond lines
        // BondSpacing is a RELATIVE value (percentage of bond length)
        // - In CDX format: stored as (10 * percent), so default 18 means 1.8%
        // - In CDXML format: stored directly as percent, so 18 means 18%
        // The spacing is measured between the line segments (not from center of each line)
        let spacing_percent_raw = self.bond_spacing.unwrap_or(_ctx.default_bond_spacing()) as f32;

        // BondSpacingAbs takes precedence if available (per CDX specification)
        let spacing_screen = if let Some(abs_spacing) = self.bond_spacing_abs {
            // Use absolute spacing directly, scaled to screen coordinates
            _ctx.cdx_length_to_screen(abs_spacing)
        } else {
            // Use relative spacing: spacing_percent_raw is in CDX units (10 * percent)
            // For CDX: 180 means 18%, so divide by 1000 (= 10 * 100)
            // Calculate as percentage of bond length on screen
            len * (spacing_percent_raw / 1000.0)
        };

        if order == 2 {
            // Double bond: two parallel lines with position determined by double_position property
            // Position values: 0/256=Center, 1/257=Right, 2/258=Left
            let position = self.double_position.unwrap_or(1) % 256;

            let (a1, a2, b1, b2) = match position {
                0 => {
                    // Center: two lines symmetrically offset from the center
                    let half_offset = spacing_screen * 0.5;
                    let ox = nx * half_offset;
                    let oy = ny * half_offset;
                    (
                        BackendPoint2d::new(p1.x + ox, p1.y + oy),
                        BackendPoint2d::new(p2.x + ox, p2.y + oy),
                        BackendPoint2d::new(p1.x - ox, p1.y - oy),
                        BackendPoint2d::new(p2.x - ox, p2.y - oy),
                    )
                }
                2 => {
                    // Right: center line + one line on the right side
                    let offset = spacing_screen;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    (
                        p1, // Center line start
                        p2, // Center line end
                        BackendPoint2d::new(p1.x + ox, p1.y + oy),
                        BackendPoint2d::new(p2.x + ox, p2.y + oy),
                    )
                }
                1 => {
                    // Left: center line + one line on the left side
                    let offset = spacing_screen;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    (
                        p1, // Center line start
                        p2, // Center line end
                        BackendPoint2d::new(p1.x - ox, p1.y - oy),
                        BackendPoint2d::new(p2.x - ox, p2.y - oy),
                    )
                }
                _ => {
                    // Default to center for unknown values
                    let half_offset = spacing_screen * 0.5;
                    let ox = nx * half_offset;
                    let oy = ny * half_offset;
                    (
                        BackendPoint2d::new(p1.x + ox, p1.y + oy),
                        BackendPoint2d::new(p2.x + ox, p2.y + oy),
                        BackendPoint2d::new(p1.x - ox, p1.y - oy),
                        BackendPoint2d::new(p2.x - ox, p2.y - oy),
                    )
                }
            };

            _ctx.painter.line_segment(a1, a2, stroke);
            _ctx.painter.line_segment(b1, b2, stroke);
        } else {
            // Triple bond: center line + two outer lines with full spacing
            _ctx.painter.line_segment(p1, p2, stroke);
            let offset = spacing_screen;
            let ox = nx * offset;
            let oy = ny * offset;

            let a1 = BackendPoint2d::new(p1.x + ox, p1.y + oy);
            let a2 = BackendPoint2d::new(p2.x + ox, p2.y + oy);
            let b1 = BackendPoint2d::new(p1.x - ox, p1.y - oy);
            let b2 = BackendPoint2d::new(p2.x - ox, p2.y - oy);
            _ctx.painter.line_segment(a1, a2, stroke);
            _ctx.painter.line_segment(b1, b2, stroke);
        }
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        // I should calculate the bounding box of the bond. But I cannot ref node position here.
        // I will implement it later.
        None
    }
}
