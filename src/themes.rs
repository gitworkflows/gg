use iced::{Color, Theme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTheme {
    pub name: String,
    pub is_dark: bool,
    pub colors: HashMap<String, [f32; 3]>, // RGB values 0.0-1.0
    pub accent_color: [f32; 3],
    pub border_color: [f32; 3],
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
        let mut colors = HashMap::new();
        colors.insert("background".to_string(), [0.1, 0.1, 0.1]);
        colors.insert("foreground".to_string(), [0.9, 0.9, 0.9]);
        colors.insert("black".to_string(), [0.0, 0.0, 0.0]);
        colors.insert("red".to_string(), [0.8, 0.2, 0.2]);
        colors.insert("green".to_string(), [0.2, 0.8, 0.2]);
        colors.insert("yellow".to_string(), [0.8, 0.8, 0.2]);
        colors.insert("blue".to_string(), [0.2, 0.2, 0.8]);
        colors.insert("magenta".to_string(), [0.8, 0.2, 0.8]);
        colors.insert("cyan".to_string(), [0.2, 0.8, 0.8]);
        colors.insert("white".to_string(), [0.7, 0.7, 0.7]);
        colors.insert("bright_black".to_string(), [0.3, 0.3, 0.3]);
        colors.insert("bright_red".to_string(), [1.0, 0.0, 0.0]);
        colors.insert("bright_green".to_string(), [0.0, 1.0, 0.0]);
        colors.insert("bright_yellow".to_string(), [1.0, 1.0, 0.0]);
        colors.insert("bright_blue".to_string(), [0.0, 0.0, 1.0]);
        colors.insert("bright_magenta".to_string(), [1.0, 0.0, 1.0]);
        colors.insert("bright_cyan".to_string(), [0.0, 1.0, 1.0]);
        colors.insert("bright_white".to_string(), [1.0, 1.0, 1.0]);
        colors.insert("block_background_dark".to_string(), [0.15, 0.15, 0.15]);
        colors.insert("block_background_light".to_string(), [0.85, 0.85, 0.85]);


        WarpTheme {
            name: "Default Dark".to_string(),
            is_dark: true,
            colors,
            accent_color: [0.0, 0.6, 1.0], // A vibrant blue
            border_color: [0.3, 0.3, 0.3],
        }
    }

    pub fn default_light() -> Self {
        let mut colors = HashMap::new();
        colors.insert("background".to_string(), [0.9, 0.9, 0.9]);
        colors.insert("foreground".to_string(), [0.1, 0.1, 0.1]);
        colors.insert("black".to_string(), [0.0, 0.0, 0.0]);
        colors.insert("red".to_string(), [0.8, 0.2, 0.2]);
        colors.insert("green".to_string(), [0.2, 0.8, 0.2]);
        colors.insert("yellow".to_string(), [0.8, 0.8, 0.2]);
        colors.insert("blue".to_string(), [0.2, 0.2, 0.8]);
        colors.insert("magenta".to_string(), [0.8, 0.2, 0.8]);
        colors.insert("cyan".to_string(), [0.2, 0.8, 0.8]);
        colors.insert("white".to_string(), [0.7, 0.7, 0.7]);
        colors.insert("bright_black".to_string(), [0.3, 0.3, 0.3]);
        colors.insert("bright_red".to_string(), [1.0, 0.0, 0.0]);
        colors.insert("bright_green".to_string(), [0.0, 1.0, 0.0]);
        colors.insert("bright_yellow".to_string(), [1.0, 1.0, 0.0]);
        colors.insert("bright_blue".to_string(), [0.0, 0.0, 1.0]);
        colors.insert("bright_magenta".to_string(), [1.0, 0.0, 1.0]);
        colors.insert("bright_cyan".to_string(), [0.0, 1.0, 1.0]);
        colors.insert("bright_white".to_string(), [1.0, 1.0, 1.0]);
        colors.insert("block_background_dark".to_string(), [0.15, 0.15, 0.15]);
        colors.insert("block_background_light".to_string(), [0.85, 0.85, 0.85]);

        WarpTheme {
            name: "Default Light".to_string(),
            is_dark: false,
            colors,
            accent_color: [0.0, 0.6, 1.0], // A vibrant blue
            border_color: [0.7, 0.7, 0.7],
        }
    }

    pub fn get_color(&self, name: &str) -> Color {
        let rgb = self.colors.get(name).unwrap_or(&[0.0, 0.0, 0.0]);
        Color::from_rgb(rgb[0], rgb[1], rgb[2])
    }

    pub fn get_background_color(&self) -> Color {
        self.get_color("background")
    }

    pub fn get_foreground_color(&self) -> Color {
        self.get_color("foreground")
    }

    pub fn get_accent_color(&self) -> Color {
        Color::from_rgb(self.accent_color[0], self.accent_color[1], self.accent_color[2])
    }

    pub fn get_border_color(&self) -> Color {
        Color::from_rgb(self.border_color[0], self.border_color[1], self.border_color[2])
    }

    pub fn get_terminal_color(&self, name: &str, bright: bool) -> Color {
        let key = if bright {
            format!("bright_{}", name)
        } else {
            name.to_string()
        };
        self.get_color(&key)
    }

    pub fn get_block_background_color(&self, is_dark_theme: bool) -> Color {
        if is_dark_theme {
            self.get_color("block_background_dark")
        } else {
            self.get_color("block_background_light")
        }
    }

    pub fn is_dark_theme(&self) -> bool {
        self.is_dark
    }
}

pub struct ThemeManager {
    themes: HashMap<String, WarpTheme>,
    current_theme_name: String,
    themes_dir: PathBuf,
}

impl ThemeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let themes_dir = PathBuf::from("themes");
        let mut manager = Self {
            themes: HashMap::new(),
            current_theme_name: "Default Dark".to_string(),
            themes_dir,
        };
        manager.load_themes_from_directory()?;
        Ok(manager)
    }

    pub fn load_themes_from_directory(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.themes.clear();
        self.themes.insert("Default Dark".to_string(), WarpTheme::default_dark());
        self.themes.insert("Default Light".to_string(), WarpTheme::default_light());

        if self.themes_dir.exists() && self.themes_dir.is_dir() {
            for entry in fs::read_dir(&self.themes_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                    let theme_str = fs::read_to_string(&path)?;
                    let theme: WarpTheme = serde_yaml::from_str(&theme_str)?;
                    self.themes.insert(theme.name.clone(), theme);
                }
            }
        } else {
            fs::create_dir_all(&self.themes_dir)?;
            // Save default themes to files if directory was just created
            self.save_theme("Default Dark", &WarpTheme::default_dark())?;
            self.save_theme("Default Light", &WarpTheme::default_light())?;
        }
        Ok(())
    }

    pub fn get_current_theme(&self) -> &WarpTheme {
        self.themes.get(&self.current_theme_name).unwrap_or(&WarpTheme::default_dark())
    }

    pub fn set_current_theme(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.current_theme_name = name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found.", name))
        }
    }

    pub fn get_available_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    pub fn save_theme(&self, name: &str, theme: &WarpTheme) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{}.yaml", name.replace(" ", "_").to_lowercase());
        let path = self.themes_dir.join(file_name);
        let yaml_string = serde_yaml::to_string(theme)?;
        fs::write(path, yaml_string)?;
        Ok(())
    }
}

// This file is now deprecated and its content has been moved to:
// - src/config/theme.rs (for WarpTheme struct and color parsing)
// - src/config/yaml_theme.rs (for YamlTheme and related structs)
// - src/config/yaml_theme_manager.rs (for ThemeManager logic)

// You should remove this file from your project's module declarations in main.rs
// and update any imports that were pointing here.
