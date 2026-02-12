use crate::cdx::text::TextObject;
use crate::cdx::values::{CDXStyleRun, Rectangle};
use crate::renderer::backend::{Align2, TextSpan, TextStyle};
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TextObject {
    fn draw<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &crate::renderer::RenderContext<P>,
    ) {
        // Check visibility
        if let Some(false) = self.visible {
            return;
        }

        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(&pos.to_backend_point());

            // Determine base alignment based on justification
            let base_align = match self.justification.or(self.caption_justification) {
                Some(0) => Align2::LEFT_CENTER,   // Left
                Some(1) => Align2::CENTER_CENTER, // Center
                Some(2) => Align2::RIGHT_CENTER,  // Right
                _ => Align2::LEFT_CENTER,         // Default to left
            };

            // Get text content from CDXString
            if let Some(ref cdx_str) = self.text {
                let text_str = &cdx_str.text;
                let style_runs = &cdx_str.style_runs;

                // If no style runs, use simple rendering with rich_text
                if style_runs.is_empty() {
                    // Determine font size (prefer caption size, fallback to label size, then document default)
                    let font_size = if let Some(size) = self.caption_size {
                        size as f32
                    } else if let Some(size) = self.label_size {
                        size as f32
                    } else {
                        ctx.default_caption_size()
                    };

                    // // Determine text color (prefer caption color, fallback to label color, then document default)
                    // let color = if let Some(idx) = self.caption_color {
                    //     ctx.resolve_color_i16(Some(idx), ctx.default_caption_color())
                    // } else if let Some(idx) = self.label_color {
                    //     ctx.resolve_color_i16(Some(idx), ctx.default_label_color())
                    // } else {
                    //     ctx.default_caption_color()
                    // };

                    let color = super::Color::from_rgb(255,255,255,);
                    let span = TextSpan::new(text_str.clone(), font_size, color);
                    ctx.painter.rich_text(screen_pos, base_align, &[span]);
                    return;
                }

                // Render text with style runs using rich_text
                self.draw_styled_text(ctx, text_str, style_runs, screen_pos, base_align);
            }
        }
    }
    fn get_bounding_box(&self) -> Option<Rectangle> {
        self.bounding_box.clone()
    }
}

impl TextObject {
    fn draw_styled_text<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        text: &str,
        style_runs: &[CDXStyleRun],
        pos: crate::renderer::backend::Point2d,
        base_align: crate::renderer::backend::Align2,
    ) {
        if style_runs.is_empty() {
            return;
        }

        let chars: Vec<char> = text.chars().collect();

        // Build TextSpan array from style runs
        let mut spans: Vec<TextSpan> = Vec::new();

        for (i, run) in style_runs.iter().enumerate() {
            let start_idx = run.char_index as usize;
            let end_idx = if i + 1 < style_runs.len() {
                style_runs[i + 1].char_index as usize
            } else {
                chars.len()
            };

            if start_idx >= chars.len() {
                continue;
            }

            let segment: String = chars[start_idx..end_idx.min(chars.len())].iter().collect();

            // Font size: 20ths of a point -> points
            let base_font_size = (run.font_size as f32) / 20.0;
            // Apply scale for consistent sizing
            let scale = ctx.zoom * ctx.auto_scale;
            let font_size = base_font_size * scale;

            // Get color from color table or use document default
            let color = ctx.resolve_color(Some(run.color_index), ctx.default_caption_color());

            // Parse font_face to determine style
            let is_bold = (run.font_face & 0x01) != 0;
            let is_italic = (run.font_face & 0x02) != 0;
            let is_underline = (run.font_face & 0x04) != 0;
            // Subscript (0x20), superscript (0x40), and formula (0x60) are mutually exclusive
            let script_style = run.font_face & 0x60;
            let is_subscript = script_style == 0x20;
            let is_superscript = script_style == 0x40;

            // Adjust font size for sub/superscript
            let adjusted_font_size = if is_superscript || is_subscript {
                font_size * 0.7
            } else {
                font_size
            };

            // Build TextStyle
            let mut style = TextStyle::new();
            if is_bold {
                style = style.bold();
            }
            if is_italic {
                style = style.italic();
            }
            if is_underline {
                style = style.underline();
            }
            if is_subscript {
                style = style.subscript();
            }
            if is_superscript {
                style = style.superscript();
            }

            spans.push(TextSpan::new(segment, adjusted_font_size, color).with_style(style));
        }

        // Draw all spans using rich_text
        ctx.painter.rich_text(pos, base_align, &spans);
    }

    fn calculate_total_width<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        text: &str,
        style_runs: &[CDXStyleRun],
    ) -> f32 {
        use crate::renderer::backend::{FontFamily, FontId};

        if style_runs.is_empty() {
            return 0.0;
        }

        let chars: Vec<char> = text.chars().collect();
        let mut total_width = 0.0;

        for (i, run) in style_runs.iter().enumerate() {
            let start_idx = run.char_index as usize;
            let end_idx = if i + 1 < style_runs.len() {
                style_runs[i + 1].char_index as usize
            } else {
                chars.len()
            };

            if start_idx >= chars.len() {
                continue;
            }

            let segment: String = chars[start_idx..end_idx.min(chars.len())].iter().collect();
            let base_font_size = (run.font_size as f32) / 20.0;
            let scale = ctx.zoom * ctx.auto_scale;
            let font_size = base_font_size * scale;

            // Subscript (0x20), superscript (0x40), and formula (0x60) are mutually exclusive
            let script_style = run.font_face & 0x60;
            let is_subscript = script_style == 0x20;
            let is_superscript = script_style == 0x40;
            let adjusted_font_size = if is_superscript || is_subscript {
                font_size * 0.7
            } else {
                font_size
            };

            let font_id = FontId::new(adjusted_font_size, FontFamily::Proportional);

            let color = ctx.resolve_color(Some(run.color_index), ctx.default_caption_color());

            let galley = ctx.painter.layout_no_wrap(segment, font_id, color);
            total_width += galley.size.0;
        }

        total_width
    }
}
