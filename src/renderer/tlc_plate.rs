use crate::cdx::tlc_plate::TLCPlate;
use crate::cdx::values::Rectangle;
use crate::renderer::backend::AbstractPainter;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TLCPlate {
    fn draw<P: AbstractPainter>(&self, ctx: &RenderContext<P>) {
        use crate::cdx::values::Point2d;
        use crate::renderer::backend::{Point2d as BackendPoint2d, Stroke};

        if matches!(self.visible, Some(false)) {
            return;
        }

        let (top_left, top_right, bottom_right, bottom_left) = match (
            &self.top_left,
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
        ) {
            (Some(tl), Some(tr), Some(br), Some(bl)) => {
                (tl.clone(), tr.clone(), br.clone(), bl.clone())
            }
            _ => match &self.bounding_box {
                Some(bbox) => (
                    Point2d {
                        x: bbox.left,
                        y: bbox.top,
                    },
                    Point2d {
                        x: bbox.right,
                        y: bbox.top,
                    },
                    Point2d {
                        x: bbox.right,
                        y: bbox.bottom,
                    },
                    Point2d {
                        x: bbox.left,
                        y: bbox.bottom,
                    },
                ),
                None => return,
            },
        };

        let color = ctx.resolve_color(self.foreground_color, ctx.default_foreground_color());
        let line_width = self.line_width.unwrap_or_else(|| ctx.default_line_width());
        let scale = ctx.zoom * ctx.auto_scale;
        let stroke = Stroke::new((line_width * scale as f64) as f32, color);

        let to_screen = |p: &Point2d| -> BackendPoint2d { ctx.cdx_to_screen(p) };

        if self.tlc_show_borders.unwrap_or(false) {
            let tl = to_screen(&top_left);
            let tr = to_screen(&top_right);
            let br = to_screen(&bottom_right);
            let bl = to_screen(&bottom_left);
            ctx.painter.line_segment(tl, tr, stroke);
            ctx.painter.line_segment(tr, br, stroke);
            ctx.painter.line_segment(br, bl, stroke);
            ctx.painter.line_segment(bl, tl, stroke);
        }

        let lerp = |a: &Point2d, b: &Point2d, t: f64| -> Point2d {
            Point2d {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
            }
        };

        if self.tlc_show_origin.unwrap_or(false) {
            if let Some(fraction) = self.tlc_origin_fraction {
                if (0.0..=1.0).contains(&fraction) {
                    let left = lerp(&bottom_left, &top_left, fraction);
                    let right = lerp(&bottom_right, &top_right, fraction);
                    ctx.painter
                        .line_segment(to_screen(&left), to_screen(&right), stroke);
                }
            }
        }

        if self.tlc_show_solvent_front.unwrap_or(false) {
            if let Some(fraction) = self.tlc_solvent_front_fraction {
                if (0.0..=1.0).contains(&fraction) {
                    let left = lerp(&top_left, &bottom_left, fraction);
                    let right = lerp(&top_right, &bottom_right, fraction);
                    ctx.painter
                        .line_segment(to_screen(&left), to_screen(&right), stroke);
                }
            }
        }
    }

    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}
