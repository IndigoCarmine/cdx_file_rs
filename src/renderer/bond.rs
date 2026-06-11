use crate::cdx::bond::Bond;
use crate::cdx::values::Rectangle;
use crate::renderer::to_points::{self, ToBackendF32};
use crate::renderer::{
    Drawable,
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
            Some(pos) => pos.to_backend_point(),
            None => return,
        };

        let end = match _ctx.node_position(end_id) {
            Some(pos) => pos.to_backend_point(),
            None => return,
        };

        let p1 = _ctx.cdx_to_screen(&start);
        let p2 = _ctx.cdx_to_screen(&end);

        let color = _ctx.resolve_color(self.foreground_color, _ctx.default_foreground_color());

        // line_width is stored as raw CDX fixed-point (÷65536 = CDX pts); apply auto_scale→px
        let line_width = _ctx.cdx_length_to_screen(self.line_width.unwrap_or(_ctx.default_line_width()) / 65536.0);
        let stroke = Stroke::new(line_width, color);


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
        let spacing_percent_raw = self.bond_spacing.unwrap_or(_ctx.default_bond_spacing()) as f32 / 10.0;

        // BondSpacingAbs takes precedence if available (per CDX specification)
        let spacing_screen = if let Some(abs_spacing) = self.bond_spacing_abs {
            // Use absolute spacing directly, scaled to screen coordinates
            _ctx.cdx_length_to_screen(abs_spacing)
        } else {
            // Use relative spacing: spacing_percent_raw is in CDX units (10 * percent)
            // For CDX: 180 means 18%, so divide by 1000 (= 10 * 100)
            // Calculate as percentage of bond length on screen
            len * (spacing_percent_raw / 100.0)
        };

        if order == 2 {
            // Double bond: two parallel lines with position determined by double_position property
            // Position values: 0/256=Center, 1/257=Right, 2/258=Left
            let position = self.double_position.unwrap_or(1) % 256;

            // For non-centered double bonds the secondary (inner) line is shortened ~15% from each
            // end to match ChemDraw's ring-bond drawing style.
            const INNER_START: f32 = 0.15;
            const INNER_END: f32 = 0.85;

            let truncate = |b1: BackendPoint2d, b2: BackendPoint2d| -> (BackendPoint2d, BackendPoint2d) {
                (
                    BackendPoint2d::new(
                        b1.x + (b2.x - b1.x) * INNER_START,
                        b1.y + (b2.y - b1.y) * INNER_START,
                    ),
                    BackendPoint2d::new(
                        b1.x + (b2.x - b1.x) * INNER_END,
                        b1.y + (b2.y - b1.y) * INNER_END,
                    ),
                )
            };

            match position {
                0 => {
                    // Center: two lines symmetrically offset — no truncation
                    let half_offset = spacing_screen * 0.5;
                    let ox = nx * half_offset;
                    let oy = ny * half_offset;
                    let a1 = BackendPoint2d::new(p1.x + ox, p1.y + oy);
                    let a2 = BackendPoint2d::new(p2.x + ox, p2.y + oy);
                    let b1 = BackendPoint2d::new(p1.x - ox, p1.y - oy);
                    let b2 = BackendPoint2d::new(p2.x - ox, p2.y - oy);
                    _ctx.painter.line_segment(a1, a2, stroke);
                    _ctx.painter.line_segment(b1, b2, stroke);
                }
                2 => {
                    // Right: full center line + truncated secondary on the right side
                    let offset = spacing_screen;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    let b1 = BackendPoint2d::new(p1.x + ox, p1.y + oy);
                    let b2 = BackendPoint2d::new(p2.x + ox, p2.y + oy);
                    let (b1t, b2t) = truncate(b1, b2);
                    _ctx.painter.line_segment(p1, p2, stroke);
                    _ctx.painter.line_segment(b1t, b2t, stroke);
                }
                1 | _ => {
                    // Left (default): full center line + truncated secondary on the left side
                    let offset = spacing_screen;
                    let ox = nx * offset;
                    let oy = ny * offset;
                    let b1 = BackendPoint2d::new(p1.x - ox, p1.y - oy);
                    let b2 = BackendPoint2d::new(p2.x - ox, p2.y - oy);
                    let (b1t, b2t) = truncate(b1, b2);
                    _ctx.painter.line_segment(p1, p2, stroke);
                    _ctx.painter.line_segment(b1t, b2t, stroke);
                }
            };
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
