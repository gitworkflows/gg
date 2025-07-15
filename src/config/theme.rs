use iced::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WarpTheme {
    pub name: String,
    pub is_dark: bool,
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub border: String,
    pub command_background: String,
    pub output_background: String,
    // Add more theme-specific colors as needed
}

impl Default for WarpTheme {
    fn default() -> Self {
        Self::from_name("Default Dark")
    }
}

impl WarpTheme {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Default Dark" => Self::default_dark(),
            "Default Light" => Self::default_light(),
            _ => Self::default_dark(), // Fallback
        }
    }

    pub fn default_dark() -> Self {
        WarpTheme {
            name: "Default Dark".to_string(),
            is_dark: true,
            background: "#1E1E1E".to_string(),
            foreground: "#D4D4D4".to_string(),
            accent: "#007ACC".to_string(),
            border: "#333333".to_string(),
            command_background: "#252525".to_string(),
            output_background: "#1E1E1E".to_string(),
        }
    }

    pub fn default_light() -> Self {
        WarpTheme {
            name: "Default Light".to_string(),
            is_dark: false,
            background: "#FFFFFF".to_string(),
            foreground: "#000000".to_string(),
            accent: "#007ACC".to_string(),
            border: "#CCCCCC".to_string(),
            command_background: "#F5F5F5".to_string(),
            output_background: "#FFFFFF".to_string(),
        }
    }

    pub fn get_background_color(&self) -> Color {
        parse_hex_color(&self.background).unwrap_or(Color::BLACK)
    }

    pub fn get_foreground_color(&self) -> Color {
        parse_hex_color(&self.foreground).unwrap_or(Color::WHITE)
    }

    pub fn get_accent_color(&self) -> Color {
        parse_hex_color(&self.accent).unwrap_or(Color::from_rgb(0.0, 0.5, 1.0))
    }

    pub fn get_border_color(&self) -> Color {
        parse_hex_color(&self.border).unwrap_or(Color::from_rgb(0.2, 0.2, 0.2))
    }

    pub fn get_command_background_color(&self, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            parse_hex_color(&self.command_background).unwrap_or(Color::from_rgb(0.15, 0.15, 0.15))
        } else {
            parse_hex_color(&self.command_background).unwrap_or(Color::from_rgb(0.9, 0.9, 0.9))
        }
    }

    pub fn get_output_background_color(&self, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            parse_hex_color(&self.output_background).unwrap_or(Color::BLACK)
        } else {
            parse_hex_color(&self.output_background).unwrap_or(Color::WHITE)
        }
    }

    pub fn is_dark_theme(&self) -> bool {
        self.is_dark
    }
}

// Helper function to parse hex color strings
fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::from_rgb8(r, g, b))
    } else {
        None
    }
}
