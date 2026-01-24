use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FontStyle {
    pub font_id: u16,
    pub face: u16,
    pub size: u16,
    pub color: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StyleRun {
    pub start: u16,
    pub style: FontStyle,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StyledString {
    pub text: String,
    pub runs: Vec<StyleRun>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FontEntry {
    pub id: u16,
    pub charset: u16,
    pub name: String,
}
