use crate::cdx::text::TextObject;
use crate::cdx::values::CDXStyleRun;
use crate::renderer::{Drawable, RenderContext};
use crate::renderer::backend::Align2;

impl Drawable for TextObject {
    fn draw<P: crate::renderer::backend::AbstractPainter>(&self, ctx: &crate::renderer::RenderContext<P>) {
        // Check visibility
        if let Some(false) = self.visible {
            return;
        }

        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);

            // Determine base alignment based on justification
            let base_align = match self.justification.or(self.caption_justification) {
                Some(0) => Align2::LEFT_CENTER,   // Left
                Some(1) => Align2::CENTER_CENTER, // Center
                Some(2) => Align2::RIGHT_CENTER,  // Right
                _ => Align2::LEFT_CENTER,         // Default to left
            };

            // Get text content from CDXString
            let text_str = &self.text.text;
            let style_runs = &self.text.style_runs;

            // If no style runs, use simple rendering
            if style_runs.is_empty() {
                // Determine font size (prefer caption size, fallback to label size, then document default)
                let font_size = if let Some(size) = self.caption_size {
                    size as f32
                } else if let Some(size) = self.label_size {
                    size as f32
                } else {
                    ctx.default_caption_size()
                };

                // Determine text color (prefer caption color, fallback to label color, then document default)
                let color = if let Some(idx) = self.caption_color {
                    ctx.resolve_color_i16(Some(idx), ctx.default_caption_color())
                } else if let Some(idx) = self.label_color {
                    ctx.resolve_color_i16(Some(idx), ctx.default_label_color())
                } else {
                    ctx.default_caption_color()
                };

                ctx.draw_text_with_align(text_str, screen_pos, base_align, color, font_size);
                return;
            }

            // Render text with style runs
            self.draw_styled_text(ctx, text_str, style_runs, screen_pos, base_align);
        }
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
        use crate::renderer::backend::{Point2d as BackendPoint2d, Align2, Align, FontId, FontFamily, Stroke};
        
        if style_runs.is_empty() {
            return;
        }

        let chars: Vec<char> = text.chars().collect();
        let current_y = pos.y;

        // Calculate total width to determine starting position based on alignment
        let total_width = self.calculate_total_width(ctx, text, style_runs);

        // Calculate starting X position based on alignment
        let start_x = match base_align.x {
            Align::Left => pos.x,
            Align::Center => pos.x - total_width / 2.0,
            Align::Right => pos.x - total_width,
        };

        let mut current_x = start_x;

        // All segments rendered with LEFT alignment from the calculated start position
        let segment_align = Align2::LEFT_CENTER;

        // Render each style run
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
            let color =
                ctx.resolve_color(Some(run.color_index), ctx.default_caption_color());

            // Parse font_face to determine style
            let _is_bold = (run.font_face & 0x01) != 0;
            let _is_italic = (run.font_face & 0x02) != 0;
            let is_underline = (run.font_face & 0x04) != 0;
            // Subscript (0x20), superscript (0x40), and formula (0x60) are mutually exclusive
            let script_style = run.font_face & 0x60;
            let is_subscript = script_style == 0x20;
            let is_superscript = script_style == 0x40;

            // Calculate Y offset for subscript/superscript
            let y_offset = if is_superscript {
                -font_size * 0.4 // Move up for superscript
            } else if is_subscript {
                font_size * 0.3 // Move down for subscript
            } else {
                0.0
            };

            // Adjust font size for sub/superscript
            let adjusted_font_size = if is_superscript || is_subscript {
                font_size * 0.7
            } else {
                font_size
            };

            // Create font with appropriate style
            // egui doesn't have direct bold/italic control in FontFamily
            // We use Proportional as base for all styles
            let adjusted_font_id =
                FontId::new(adjusted_font_size, FontFamily::Proportional);

            // Calculate segment position
            let segment_pos = BackendPoint2d::new(current_x, current_y + y_offset);

            // Draw the text segment with left alignment
            ctx.painter.text(
                segment_pos,
                segment_align,
                &segment,
                adjusted_font_id.clone(),
                color,
            );

            // Draw underline if needed
            if is_underline {
                let galley =
                    ctx.painter
                        .layout_no_wrap(segment.clone(), adjusted_font_id.clone(), color);
                let underline_y = segment_pos.y + adjusted_font_size * 0.6;
                let underline_start = BackendPoint2d::new(segment_pos.x, underline_y);
                let underline_end = BackendPoint2d::new(segment_pos.x + galley.size.0, underline_y);
                ctx.painter.line_segment(
                    underline_start,
                    underline_end,
                    Stroke::new(1.0, color),
                );
            }

            // Calculate the width of this segment to advance cursor
            let galley = ctx.painter.layout_no_wrap(segment, adjusted_font_id, color);
            current_x += galley.size.0;
        }
    }

    fn calculate_total_width<P: crate::renderer::backend::AbstractPainter>(
        &self,
        ctx: &RenderContext<P>,
        text: &str,
        style_runs: &[CDXStyleRun],
    ) -> f32 {
        use crate::renderer::backend::{FontId, FontFamily};
        
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

            let color =
                ctx.resolve_color(Some(run.color_index), ctx.default_caption_color());

            let galley = ctx.painter.layout_no_wrap(segment, font_id, color);
            total_width += galley.size.0;
        }

        total_width
    }
}
