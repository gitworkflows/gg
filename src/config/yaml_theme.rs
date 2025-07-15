use serde::{Deserialize, Serialize};
use iced::Color;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminalColors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection_background: String,
    pub selection_foreground: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorPalette {
    pub primary: TerminalColors,
    pub normal: TerminalColors,
    pub bright: TerminalColors,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YamlTheme {
    pub name: String,
    pub author: String,
    pub colors: ColorPalette,
}

impl YamlTheme {
    pub fn to_iced_theme(&self) -> crate::config::theme::WarpTheme {
        crate::config::theme::WarpTheme {
            name: self.name.clone(),
            is_dark: is_dark_hex(&self.colors.primary.background), // Heuristic to determine dark/light
            background: self.colors.primary.background.clone(),
            foreground: self.colors.primary.foreground.clone(),
            accent: self.colors.blue.clone(), // Using blue as a generic accent for now
            border: self.colors.bright_black.clone(), // Using bright_black as border
            command_background: self.colors.normal.background.clone(), // Example
            output_background: self.colors.primary.background.clone(), // Example
        }
    }
}

// Simple heuristic to determine if a hex color is dark
fn is_dark_hex(hex: &str) -> bool {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
        // Calculate luminance (perceived brightness)
        let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
        luminance < 0.5 // Threshold for dark vs light
    } else {
        true // Default to dark if parsing fails
    }
}
