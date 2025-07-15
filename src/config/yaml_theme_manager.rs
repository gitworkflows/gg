use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde_yaml;
use log::{info, error};

use super::yaml_theme::YamlTheme;
use super::theme::WarpTheme; // Import WarpTheme

pub struct ThemeManager {
    themes: HashMap<String, YamlTheme>,
    active_theme_name: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            themes: HashMap::new(),
            active_theme_name: "Default Dark".to_string(), // Default fallback
        };
        manager.load_builtin_themes();
        manager.load_user_themes();
        manager
    }

    fn load_builtin_themes(&mut self) {
        info!("Loading built-in themes...");
        // Example built-in themes (you would embed these or load from a known path)
        let default_dark_yaml = r#"
            name: Default Dark
            author: Warp
            colors:
              primary:
                background: "#1E1E1E"
                foreground: "#D4D4D4"
                cursor: "#D4D4D4"
                selection_background: "#264F78"
                selection_foreground: "#FFFFFF"
                black: "#000000"
                red: "#CD3131"
                green: "#0DBC79"
                yellow: "#E5E510"
                blue: "#2472C8"
                magenta: "#BC3FBC"
                cyan: "#0598BC"
                white: "#E5E5E5"
              normal:
                background: "#1E1E1E"
                foreground: "#D4D4D4"
                cursor: "#D4D4D4"
                selection_background: "#264F78"
                selection_foreground: "#FFFFFF"
                black: "#000000"
                red: "#CD3131"
                green: "#0DBC79"
                yellow: "#E5E510"
                blue: "#2472C8"
                magenta: "#BC3FBC"
                cyan: "#0598BC"
                white: "#E5E5E5"
              bright:
                background: "#1E1E1E"
                foreground: "#D4D4D4"
                cursor: "#D4D4D4"
                selection_background: "#264F78"
                selection_foreground: "#FFFFFF"
                black: "#666666"
                red: "#F14C4C"
                green: "#23D18B"
                yellow: "#F5F543"
                blue: "#3B8EEA"
                magenta: "#D670D6"
                cyan: "#29B8DB"
                white: "#FFFFFF"
        "#;
        if let Ok(theme) = serde_yaml::from_str::<YamlTheme>(default_dark_yaml) {
            self.themes.insert(theme.name.clone(), theme);
            info!("Loaded built-in theme: Default Dark");
        } else {
            error!("Failed to parse built-in Default Dark theme.");
        }

        let default_light_yaml = r#"
            name: Default Light
            author: Warp
            colors:
              primary:
                background: "#FFFFFF"
                foreground: "#000000"
                cursor: "#000000"
                selection_background: "#ADD6FF"
                selection_foreground: "#000000"
                black: "#000000"
                red: "#CD3131"
                green: "#0DBC79"
                yellow: "#E5E510"
                blue: "#2472C8"
                magenta: "#BC3FBC"
                cyan: "#0598BC"
                white: "#E5E5E5"
              normal:
                background: "#FFFFFF"
                foreground: "#000000"
                cursor: "#000000"
                selection_background: "#ADD6FF"
                selection_foreground: "#000000"
                black: "#000000"
                red: "#CD3131"
                green: "#0DBC79"
                yellow: "#E5E510"
                blue: "#2472C8"
                magenta: "#BC3FBC"
                cyan: "#0598BC"
                white: "#E5E5E5"
              bright:
                background: "#FFFFFF"
                foreground: "#000000"
                cursor: "#000000"
                selection_background: "#ADD6FF"
                selection_foreground: "#000000"
                black: "#666666"
                red: "#F14C4C"
                green: "#23D18B"
                yellow: "#F5F543"
                blue: "#3B8EEA"
                magenta: "#D670D6"
                cyan: "#29B8DB"
                white: "#FFFFFF"
        "#;
        if let Ok(theme) = serde_yaml::from_str::<YamlTheme>(default_light_yaml) {
            self.themes.insert(theme.name.clone(), theme);
            info!("Loaded built-in theme: Default Light");
        } else {
            error!("Failed to parse built-in Default Light theme.");
        }
    }

    fn get_themes_dir() -> PathBuf {
        directories::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("warp-terminal-clone")
            .join("themes")
    }

    fn load_user_themes(&mut self) {
        let themes_dir = Self::get_themes_dir();
        info!("Loading user themes from: {:?}", themes_dir);
        if themes_dir.exists() {
            for entry in fs::read_dir(&themes_dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            match serde_yaml::from_str::<YamlTheme>(&content) {
                                Ok(theme) => {
                                    info!("Loaded user theme: {}", theme.name);
                                    self.themes.insert(theme.name.clone(), theme);
                                }
                                Err(e) => {
                                    error!("Failed to parse theme file {:?}: {}", path, e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to read theme file {:?}: {}", path, e);
                        }
                    }
                }
            }
        } else {
            info!("User themes directory does not exist: {:?}", themes_dir);
            // Create the directory if it doesn't exist
            if let Err(e) = fs::create_dir_all(&themes_dir) {
                error!("Failed to create user themes directory {:?}: {}", themes_dir, e);
            }
        }
    }

    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    pub fn get_active_theme(&self) -> WarpTheme {
        self.themes.get(&self.active_theme_name)
            .map(|yaml_theme| yaml_theme.to_iced_theme())
            .unwrap_or_else(WarpTheme::default_dark) // Fallback to default dark
    }

    pub fn set_active_theme(&mut self, name: &str) {
        if self.themes.contains_key(name) {
            self.active_theme_name = name.to_string();
            info!("Active theme set to: {}", name);
        } else {
            error!("Theme '{}' not found.", name);
        }
    }

    pub fn get_theme_by_name(&self, name: &str) -> Option<WarpTheme> {
        self.themes.get(name).map(|yaml_theme| yaml_theme.to_iced_theme())
    }

    pub fn import_theme(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let theme: YamlTheme = serde_yaml::from_str(&content)?;
        let target_path = Self::get_themes_dir().join(format!("{}.yaml", theme.name.replace(" ", "_").to_lowercase()));
        fs::copy(path, &target_path)?;
        self.themes.insert(theme.name.clone(), theme);
        info!("Successfully imported theme from {:?} to {:?}", path, target_path);
        Ok(())
    }

    pub fn export_theme(&self, theme_name: &str, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(theme) = self.themes.get(theme_name) {
            let yaml_content = serde_yaml::to_string(theme)?;
            let file_name = format!("{}.yaml", theme.name.replace(" ", "_").to_lowercase());
            let target_path = target_dir.join(file_name);
            fs::write(&target_path, yaml_content)?;
            info!("Successfully exported theme '{}' to {:?}", theme_name, target_path);
            Ok(())
        } else {
            Err(format!("Theme '{}' not found for export.", theme_name).into())
        }
    }
}
