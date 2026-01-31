use eframe::egui;
use crate::cdx::text::TextObject;
use crate::renderer::{Drawable, RenderContext};

impl Drawable for TextObject {
    fn draw(&self, ctx: &RenderContext) {
        // Check visibility
        if let Some(false) = self.visible {
            return;
        }

        if let Some(ref pos) = self.position_2d {
            let screen_pos = ctx.cdx_to_screen(pos);

            // Determine font size (prefer caption size, fallback to label size, then default)
            let font_size = self.caption_size
                .or(self.label_size)
                .unwrap_or(10) as f32;

            // Determine text color (prefer caption color, fallback to label color, then default)
            let color = match self.caption_color.or(self.label_color) {
                Some(idx) => ctx.document.get_color_table()
                    .and_then(|ct| ct.get(idx as usize))
                    .map(|c| c.to_color32())
                    .unwrap_or(egui::Color32::BLACK),
                None => egui::Color32::BLACK,
            };

            // Get text content from CDXString
            let text_str = &self.text.text;

            // Determine alignment based on justification
            let align = match self.justification.or(self.caption_justification) {
                Some(0) => egui::Align2::LEFT_CENTER,   // Left
                Some(1) => egui::Align2::CENTER_CENTER, // Center
                Some(2) => egui::Align2::RIGHT_CENTER,  // Right
                _ => egui::Align2::LEFT_CENTER,         // Default to left
            };

            ctx.draw_text_with_align(text_str, screen_pos, align, color, font_size);
        }
    }
}

