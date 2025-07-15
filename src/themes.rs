use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use iced::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTheme {
    pub accent: String,
    pub background: String,
    pub details: String,
    pub foreground: String,
    pub terminal_colors: TerminalColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColors {
    pub bright: ColorPalette,
    pub normal: ColorPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    current_theme: WarpTheme,
    available_themes: HashMap<String, WarpTheme>,
    themes_directory: PathBuf,
}

impl Default for WarpTheme {
    fn default() -> Self {
        WarpTheme {
            accent: "#009688".to_string(),
            background: "#2f343f".to_string(),
            details: "darker".to_string(),
            foreground: "#d3dae3".to_string(),
            terminal_colors: TerminalColors {
                bright: ColorPalette {
                    black: "#2f343f".to_string(),
                    red: "#d64937".to_string(),
                    green: "#86df5d".to_string(),
                    yellow: "#fdd75a".to_string(),
                    blue: "#0f75bd".to_string(),
                    magenta: "#9e5e83".to_string(),
                    cyan: "#37c3d6".to_string(),
                    white: "#f9f9f9".to_string(),
                },
                normal: ColorPalette {
                    black: "#262b36".to_string(),
                    red: "#9c3528".to_string(),
                    green: "#61bc3b".to_string(),
                    yellow: "#f3b43a".to_string(),
                    blue: "#0d68a8".to_string(),
                    magenta: "#744560".to_string(),
                    cyan: "#288e9c".to_string(),
                    white: "#a2a2a2".to_string(),
                },
            },
        }
    }
}

impl ThemeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let themes_directory = Self::get_themes_directory()?;
        
        // Create themes directory if it doesn't exist
        if !themes_directory.exists() {
            fs::create_dir_all(&themes_directory)?;
        }

        let mut manager = ThemeManager {
            current_theme: WarpTheme::default(),
            available_themes: HashMap::new(),
            themes_directory,
        };

        // Load built-in themes
        manager.load_builtin_themes();
        
        // Load user themes from directory
        manager.load_themes_from_directory()?;

        Ok(manager)
    }

    fn get_themes_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        Ok(config_dir.join("warp-terminal").join("themes"))
    }

    pub fn load_theme_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        let theme: WarpTheme = serde_yaml::from_str(&content)?;
        
        let theme_name = path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.available_themes.insert(theme_name.clone(), theme);
        Ok(theme_name)
    }

    pub fn load_themes_from_directory(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.themes_directory.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.themes_directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") ||
               path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Err(e) = self.load_theme_from_file(&path) {
                    eprintln!("Failed to load theme from {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    pub fn save_theme(&self, name: &str, theme: &WarpTheme) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = self.themes_directory.join(format!("{}.yaml", name));
        let yaml_content = serde_yaml::to_string(theme)?;
        fs::write(file_path, yaml_content)?;
        Ok(())
    }

    pub fn set_current_theme(&mut self, theme_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(theme) = self.available_themes.get(theme_name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name).into())
        }
    }

    pub fn get_current_theme(&self) -> &WarpTheme {
        &self.current_theme
    }

    pub fn get_available_themes(&self) -> Vec<String> {
        self.available_themes.keys().cloned().collect()
    }

    pub fn create_theme_from_yaml(yaml_content: &str) -> Result<WarpTheme, Box<dyn std::error::Error>> {
        let theme: WarpTheme = serde_yaml::from_str(yaml_content)?;
        Ok(theme)
    }

    fn load_builtin_themes(&mut self) {
        // Default dark theme
        let default_theme = WarpTheme::default();
        self.available_themes.insert("default".to_string(), default_theme.clone());
        self.current_theme = default_theme;

        // Light theme
        let light_theme = WarpTheme {
            accent: "#2196F3".to_string(),
            background: "#ffffff".to_string(),
            details: "lighter".to_string(),
            foreground: "#333333".to_string(),
            terminal_colors: TerminalColors {
                bright: ColorPalette {
                    black: "#666666".to_string(),
                    red: "#e53e3e".to_string(),
                    green: "#38a169".to_string(),
                    yellow: "#d69e2e".to_string(),
                    blue: "#3182ce".to_string(),
                    magenta: "#b83280".to_string(),
                    cyan: "#00b5d8".to_string(),
                    white: "#000000".to_string(),
                },
                normal: ColorPalette {
                    black: "#000000".to_string(),
                    red: "#c53030".to_string(),
                    green: "#2f855a".to_string(),
                    yellow: "#b7791f".to_string(),
                    blue: "#2b6cb0".to_string(),
                    magenta: "#97266d".to_string(),
                    cyan: "#0987a0".to_string(),
                    white: "#4a5568".to_string(),
                },
            },
        };
        self.available_themes.insert("light".to_string(), light_theme);

        // Dracula theme
        let dracula_theme = WarpTheme {
            accent: "#bd93f9".to_string(),
            background: "#282a36".to_string(),
            details: "darker".to_string(),
            foreground: "#f8f8f2".to_string(),
            terminal_colors: TerminalColors {
                bright: ColorPalette {
                    black: "#6272a4".to_string(),
                    red: "#ff6b6b".to_string(),
                    green: "#69ff94".to_string(),
                    yellow: "#ffffa5".to_string(),
                    blue: "#d6acff".to_string(),
                    magenta: "#ff92df".to_string(),
                    cyan: "#a4ffff".to_string(),
                    white: "#ffffff".to_string(),
                },
                normal: ColorPalette {
                    black: "#21222c".to_string(),
                    red: "#ff5555".to_string(),
                    green: "#50fa7b".to_string(),
                    yellow: "#f1fa8c".to_string(),
                    blue: "#bd93f9".to_string(),
                    magenta: "#ff79c6".to_string(),
                    cyan: "#8be9fd".to_string(),
                    white: "#f8f8f2".to_string(),
                },
            },
        };
        self.available_themes.insert("dracula".to_string(), dracula_theme);
    }
}

impl WarpTheme {
    pub fn get_background_color(&self) -> Color {
        hex_to_color(&self.background)
    }

    pub fn get_foreground_color(&self) -> Color {
        hex_to_color(&self.foreground)
    }

    pub fn get_accent_color(&self) -> Color {
        hex_to_color(&self.accent)
    }

    pub fn get_terminal_color(&self, color_name: &str, bright: bool) -> Color {
        let palette = if bright {
            &self.terminal_colors.bright
        } else {
            &self.terminal_colors.normal
        };

        let hex_color = match color_name {
            "black" => &palette.black,
            "red" => &palette.red,
            "green" => &palette.green,
            "yellow" => &palette.yellow,
            "blue" => &palette.blue,
            "magenta" => &palette.magenta,
            "cyan" => &palette.cyan,
            "white" => &palette.white,
            _ => &self.foreground,
        };

        hex_to_color(hex_color)
    }

    pub fn get_block_background_color(&self, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            // Slightly lighter than background for dark themes
            let bg = hex_to_color(&self.background);
            Color::from_rgb(
                (bg.r + 0.05).min(1.0),
                (bg.g + 0.05).min(1.0),
                (bg.b + 0.05).min(1.0),
            )
        } else {
            // Slightly darker than background for light themes
            let bg = hex_to_color(&self.background);
            Color::from_rgb(
                (bg.r - 0.05).max(0.0),
                (bg.g - 0.05).max(0.0),
                (bg.b - 0.05).max(0.0),
            )
        }
    }

    pub fn get_border_color(&self) -> Color {
        // Create a border color that's between background and foreground
        let bg = hex_to_color(&self.background);
        let fg = hex_to_color(&self.foreground);
        
        Color::from_rgb(
            (bg.r + fg.r) / 2.0,
            (bg.g + fg.g) / 2.0,
            (bg.b + fg.b) / 2.0,
        )
    }

    pub fn is_dark_theme(&self) -> bool {
        let bg = hex_to_color(&self.background);
        // Calculate luminance to determine if theme is dark
        let luminance = 0.299 * bg.r + 0.587 * bg.g + 0.114 * bg.b;
        luminance < 0.5
    }
}

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Color::BLACK; // Fallback for invalid hex
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    Color::from_rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color() {
        let color = hex_to_color("#ff0000");
        assert_eq!(color, Color::from_rgb(1.0, 0.0, 0.0));

        let color = hex_to_color("#00ff00");
        assert_eq!(color, Color::from_rgb(0.0, 1.0, 0.0));

        let color = hex_to_color("#0000ff");
        assert_eq!(color, Color::from_rgb(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_theme_serialization() {
        let theme = WarpTheme::default();
        let yaml = serde_yaml::to_string(&theme).unwrap();
        let deserialized: WarpTheme = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(theme.accent, deserialized.accent);
        assert_eq!(theme.background, deserialized.background);
    }

    #[test]
    fn test_is_dark_theme() {
        let dark_theme = WarpTheme::default();
        assert!(dark_theme.is_dark_theme());

        let light_theme = WarpTheme {
            background: "#ffffff".to_string(),
            ..WarpTheme::default()
        };
        assert!(!light_theme.is_dark_theme());
    }
}
