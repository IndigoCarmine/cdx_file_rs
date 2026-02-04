/// Font loader module for loading system fonts into egui
/// 
/// This module provides functionality to load local system fonts
/// and register them with egui's font system.

use eframe::egui;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Font style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

/// Font family key for lookup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontKey {
    pub family: String,
    pub style: FontStyle,
}

impl FontKey {
    pub fn new(family: impl Into<String>, style: FontStyle) -> Self {
        Self {
            family: family.into(),
            style,
        }
    }
}

/// Loaded font data cache
static FONT_CACHE: OnceLock<HashMap<FontKey, Vec<u8>>> = OnceLock::new();

/// Get the system fonts directory
fn get_system_font_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    
    // Windows fonts directory
    #[cfg(target_os = "windows")]
    {
        if let Some(windir) = std::env::var_os("WINDIR") {
            dirs.push(PathBuf::from(windir).join("Fonts"));
        }
        // Also check user fonts directory
        if let Some(localappdata) = std::env::var_os("LOCALAPPDATA") {
            dirs.push(PathBuf::from(localappdata).join("Microsoft").join("Windows").join("Fonts"));
        }
    }
    
    // macOS fonts directories
    #[cfg(target_os = "macos")]
    {
        dirs.push(PathBuf::from("/System/Library/Fonts"));
        dirs.push(PathBuf::from("/Library/Fonts"));
        if let Some(home) = std::env::var_os("HOME") {
            dirs.push(PathBuf::from(home).join("Library/Fonts"));
        }
    }
    
    // Linux fonts directories
    #[cfg(target_os = "linux")]
    {
        dirs.push(PathBuf::from("/usr/share/fonts"));
        dirs.push(PathBuf::from("/usr/local/share/fonts"));
        if let Some(home) = std::env::var_os("HOME") {
            dirs.push(PathBuf::from(home).join(".fonts"));
            dirs.push(PathBuf::from(home).join(".local/share/fonts"));
        }
    }
    
    dirs
}

/// Font file patterns to search for each family
fn get_font_file_patterns() -> Vec<(&'static str, &'static str, FontStyle)> {
    vec![
        // Arial variants
        ("Arial", "arial.ttf", FontStyle::Regular),
        ("Arial", "arialbd.ttf", FontStyle::Bold),
        ("Arial", "ariali.ttf", FontStyle::Italic),
        ("Arial", "arialbi.ttf", FontStyle::BoldItalic),
        
        // Times New Roman variants
        ("Times New Roman", "times.ttf", FontStyle::Regular),
        ("Times New Roman", "timesbd.ttf", FontStyle::Bold),
        ("Times New Roman", "timesi.ttf", FontStyle::Italic),
        ("Times New Roman", "timesbi.ttf", FontStyle::BoldItalic),
        
        // Symbol
        ("Symbol", "symbol.ttf", FontStyle::Regular),
        
        // Segoe UI (Windows default)
        ("Segoe UI", "segoeui.ttf", FontStyle::Regular),
        ("Segoe UI", "segoeuib.ttf", FontStyle::Bold),
        ("Segoe UI", "segoeuii.ttf", FontStyle::Italic),
        ("Segoe UI", "segoeuiz.ttf", FontStyle::BoldItalic),
        
        // Consolas (monospace)
        ("Consolas", "consola.ttf", FontStyle::Regular),
        ("Consolas", "consolab.ttf", FontStyle::Bold),
        ("Consolas", "consolai.ttf", FontStyle::Italic),
        ("Consolas", "consolaz.ttf", FontStyle::BoldItalic),
        
        // Courier New (monospace)
        ("Courier New", "cour.ttf", FontStyle::Regular),
        ("Courier New", "courbd.ttf", FontStyle::Bold),
        ("Courier New", "couri.ttf", FontStyle::Italic),
        ("Courier New", "courbi.ttf", FontStyle::BoldItalic),
        
        // DejaVu Sans (Linux common)
        ("DejaVu Sans", "DejaVuSans.ttf", FontStyle::Regular),
        ("DejaVu Sans", "DejaVuSans-Bold.ttf", FontStyle::Bold),
        ("DejaVu Sans", "DejaVuSans-Oblique.ttf", FontStyle::Italic),
        ("DejaVu Sans", "DejaVuSans-BoldOblique.ttf", FontStyle::BoldItalic),
        
        // Liberation Sans (Linux common, Arial-compatible)
        ("Liberation Sans", "LiberationSans-Regular.ttf", FontStyle::Regular),
        ("Liberation Sans", "LiberationSans-Bold.ttf", FontStyle::Bold),
        ("Liberation Sans", "LiberationSans-Italic.ttf", FontStyle::Italic),
        ("Liberation Sans", "LiberationSans-BoldItalic.ttf", FontStyle::BoldItalic),
    ]
}

/// Load fonts from system directories
fn load_system_fonts() -> HashMap<FontKey, Vec<u8>> {
    let mut fonts = HashMap::new();
    let font_dirs = get_system_font_dirs();
    let patterns = get_font_file_patterns();
    
    for dir in &font_dirs {
        if !dir.exists() {
            continue;
        }
        
        for (family, filename, style) in &patterns {
            let font_path = dir.join(filename);
            if font_path.exists() {
                if let Ok(data) = std::fs::read(&font_path) {
                    let key = FontKey::new(*family, *style);
                    fonts.entry(key).or_insert(data);
                }
            }
            
            // Also try lowercase filename
            let font_path_lower = dir.join(filename.to_lowercase());
            if font_path_lower.exists() {
                if let Ok(data) = std::fs::read(&font_path_lower) {
                    let key = FontKey::new(*family, *style);
                    fonts.entry(key).or_insert(data);
                }
            }
        }
        
        // Try to load any .ttf files from the directory for broader coverage
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "ttf" {
                        if let Some(filename) = path.file_name() {
                            let filename_str = filename.to_string_lossy().to_lowercase();
                            // Try to identify font family from filename
                            let (family, style) = identify_font_from_filename(&filename_str);
                            if !family.is_empty() {
                                if let Ok(data) = std::fs::read(&path) {
                                    let key = FontKey::new(family, style);
                                    fonts.entry(key).or_insert(data);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    fonts
}

/// Try to identify font family and style from filename
fn identify_font_from_filename(filename: &str) -> (String, FontStyle) {
    let name = filename.trim_end_matches(".ttf").trim_end_matches(".TTF");
    
    let style = if name.contains("bolditalic") || name.contains("boldoblique") || name.ends_with("bi") || name.ends_with("z") {
        FontStyle::BoldItalic
    } else if name.contains("bold") || name.ends_with("bd") || name.ends_with("b") {
        FontStyle::Bold
    } else if name.contains("italic") || name.contains("oblique") || name.ends_with("i") {
        FontStyle::Italic
    } else {
        FontStyle::Regular
    };
    
    // Extract family name (very simplified)
    let family = name
        .replace("bolditalic", "")
        .replace("boldoblique", "")
        .replace("bold", "")
        .replace("italic", "")
        .replace("oblique", "")
        .replace("regular", "")
        .trim_end_matches(|c: char| c == '-' || c == '_')
        .to_string();
    
    (family, style)
}

/// Get or initialize the font cache
pub fn get_font_cache() -> &'static HashMap<FontKey, Vec<u8>> {
    FONT_CACHE.get_or_init(load_system_fonts)
}

/// Configure egui fonts with system fonts
/// 
/// This function should be called once during app initialization,
/// typically in the `setup` or first `update` call.
pub fn configure_egui_fonts(ctx: &egui::Context) {
    let font_cache = get_font_cache();
    
    let mut fonts = egui::FontDefinitions::default();
    
    // Register each font family with egui
    for (key, data) in font_cache {
        let font_name = format!("{}-{:?}", key.family, key.style);
        
        fonts.font_data.insert(
            font_name.clone(),
            egui::FontData::from_owned(data.clone()).into(),
        );
        
        // Add to appropriate font families
        match key.style {
            FontStyle::Regular => {
                // Add as primary font for proportional
                if key.family == "Arial" || key.family == "Segoe UI" || key.family == "Liberation Sans" {
                    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                        family.insert(0, font_name.clone());
                    }
                }
                // Add monospace fonts
                if key.family == "Consolas" || key.family == "Courier New" {
                    if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                        family.insert(0, font_name.clone());
                    }
                }
            }
            _ => {
                // Bold/Italic variants are added but egui doesn't have built-in style support
                // They need to be accessed via custom font families
            }
        }
        
        // Create custom font family for each loaded font
        let custom_family = egui::FontFamily::Name(font_name.clone().into());
        fonts.families.insert(custom_family, vec![font_name]);
    }
    
    ctx.set_fonts(fonts);
}

/// Get the egui FontFamily for a given RichTextFontFamily and style
pub fn get_egui_font_family(
    family: &super::backend::RichTextFontFamily,
    bold: bool,
    italic: bool,
) -> egui::FontFamily {
    use super::backend::RichTextFontFamily;
    
    let family_name = match family {
        RichTextFontFamily::Arial => "Arial",
        RichTextFontFamily::TimesNewRoman => "Times New Roman",
        RichTextFontFamily::Symbol => "Symbol",
    };
    
    let style = match (bold, italic) {
        (true, true) => FontStyle::BoldItalic,
        (true, false) => FontStyle::Bold,
        (false, true) => FontStyle::Italic,
        (false, false) => FontStyle::Regular,
    };
    
    let font_name = format!("{}-{:?}", family_name, style);
    
    // Check if this font was loaded
    let cache = get_font_cache();
    let key = FontKey::new(family_name, style);
    
    if cache.contains_key(&key) {
        egui::FontFamily::Name(font_name.into())
    } else {
        // Fallback to Regular style if styled version not found
        let regular_key = FontKey::new(family_name, FontStyle::Regular);
        if cache.contains_key(&regular_key) {
            let regular_name = format!("{}-{:?}", family_name, FontStyle::Regular);
            egui::FontFamily::Name(regular_name.into())
        } else {
            // Final fallback to Proportional
            egui::FontFamily::Proportional
        }
    }
}

/// Get information about loaded fonts (for debugging)
pub fn get_loaded_font_info() -> Vec<String> {
    let cache = get_font_cache();
    let mut info: Vec<String> = cache
        .keys()
        .map(|k| format!("{} ({:?})", k.family, k.style))
        .collect();
    info.sort();
    info
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_font_dirs_exist() {
        let dirs = get_system_font_dirs();
        println!("Font directories:");
        for dir in &dirs {
            println!("  {} (exists: {})", dir.display(), dir.exists());
        }
        assert!(!dirs.is_empty());
    }
    
    #[test]
    fn test_load_fonts() {
        let fonts = load_system_fonts();
        println!("Loaded {} fonts:", fonts.len());
        for key in fonts.keys() {
            println!("  {} ({:?})", key.family, key.style);
        }
    }
}
