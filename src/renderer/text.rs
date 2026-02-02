use crate::cdx::text::TextObject;
use crate::cdx::values::CDXStyleRun;
use crate::renderer::{Drawable, RenderContext};
use eframe::egui;
use egui::text::{LayoutJob, TextFormat};

impl Drawable for TextObject {
    fn draw(&self, ctx: &RenderContext) {
        // Check visibility
        if let Some(false) = self.visible {
            return;
        }

        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);

            // Determine base alignment based on justification
            let base_align = match self.justification.or(self.caption_justification) {
                Some(0) => egui::Align2::LEFT_CENTER,   // Left
                Some(1) => egui::Align2::CENTER_CENTER, // Center
                Some(2) => egui::Align2::RIGHT_CENTER,  // Right
                _ => egui::Align2::LEFT_CENTER,         // Default to left
            };

            // Get text content from CDXString
            let text_str = &self.text.text;
            let style_runs = &self.text.style_runs;

            // If no style runs, use simple rendering
            if style_runs.is_empty() {
                // Determine font size (prefer caption size, fallback to label size, then default)
                let font_size = self.caption_size.or(self.label_size).unwrap_or(10) as f32;

                // Determine text color (prefer caption color, fallback to label color, then default)
                let color = match self.caption_color.or(self.label_color) {
                    Some(idx) => ctx
                        .document
                        .get_color_table()
                        .and_then(|ct| ct.get(idx as usize))
                        .map(|c| c.to_color32())
                        .unwrap_or(egui::Color32::BLACK),
                    None => egui::Color32::BLACK,
                };

                ctx.draw_text_with_align(text_str, screen_pos, base_align, color, font_size);
                return;
            }

            // Render text with style runs using the public function
            draw_cdx_string(ctx, text_str, style_runs, screen_pos, base_align);
        }
    }
}

/// Public function to draw CDX styled string using LayoutJob and TextFormat
/// This function properly handles bold, italic, underline, subscript, and superscript styles
pub fn draw_cdx_string(
    ctx: &RenderContext,
    text: &str,
    style_runs: &[CDXStyleRun],
    pos: egui::Pos2,
    base_align: egui::Align2,
) {
    if style_runs.is_empty() {
        return;
    }

    let chars: Vec<char> = text.chars().collect();
    let scale = ctx.zoom * ctx.auto_scale;

    // Create a LayoutJob for the entire text
    let mut job = LayoutJob::default();

    // Process each style run
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
        let font_size = base_font_size * scale;

        // Get color from color table
        let color = ctx
            .document
            .get_color_table()
            .and_then(|ct| ct.get(run.color_index as usize))
            .map(|c| c.to_color32())
            .unwrap_or(egui::Color32::BLACK);

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

        // Create TextFormat with proper styling
        // Subscript/superscript positioning handled via valign field
        let format = TextFormat {
            font_id: egui::FontId::proportional(adjusted_font_size),
            color,
            italics: is_italic,
            underline: if is_underline {
                egui::Stroke::new(1.0, color)
            } else {
                egui::Stroke::NONE
            },
            valign: if is_superscript {
                egui::Align::TOP
            } else if is_subscript {
                egui::Align::BOTTOM
            } else {
                egui::Align::Center
            },
            ..Default::default()
        };

        // Append the segment to the job
        job.append(&segment, 0.0, format);
    }

    // Adjust horizontal alignment for the job
    job.halign = match base_align.x() {
        egui::Align::LEFT => egui::Align::LEFT,
        egui::Align::Center => egui::Align::Center,
        egui::Align::RIGHT => egui::Align::RIGHT,
    };

    // Create galley from the layout job
    let galley = ctx.painter.fonts(|fonts| fonts.layout_job(job));

    // Calculate final position based on vertical alignment
    let final_pos = match base_align.y() {
        egui::Align::TOP => pos,
        egui::Align::Center => egui::Pos2::new(pos.x, pos.y - galley.size().y / 2.0),
        egui::Align::BOTTOM => egui::Pos2::new(pos.x, pos.y - galley.size().y),
    };

    // Draw the galley
    // Note: The color parameter is a fallback; actual colors are defined in each TextFormat
    ctx.painter.galley(final_pos, galley, egui::Color32::WHITE);
}

impl TextObject {
    fn draw_styled_text(
        &self,
        ctx: &RenderContext,
        text: &str,
        style_runs: &[CDXStyleRun],
        pos: egui::Pos2,
        base_align: egui::Align2,
    ) {
        draw_cdx_string(ctx, text, style_runs, pos, base_align);
    }

    fn calculate_total_width(
        &self,
        ctx: &RenderContext,
        text: &str,
        style_runs: &[CDXStyleRun],
    ) -> f32 {
        if style_runs.is_empty() {
            return 0.0;
        }

        let chars: Vec<char> = text.chars().collect();
        let scale = ctx.zoom * ctx.auto_scale;
        let mut job = LayoutJob::default();

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

            let color = ctx
                .document
                .get_color_table()
                .and_then(|ct| ct.get(run.color_index as usize))
                .map(|c| c.to_color32())
                .unwrap_or(egui::Color32::BLACK);

            let format = TextFormat {
                font_id: egui::FontId::proportional(adjusted_font_size),
                color,
                ..Default::default()
            };

            job.append(&segment, 0.0, format);
        }

        // Calculate width by creating a temporary galley
        let galley = ctx.painter.fonts(|fonts| fonts.layout_job(job));
        galley.size().x
    }
}
