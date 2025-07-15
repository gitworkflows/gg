use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::theme::WarpTheme; // Corrected import path

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpConfig {
    pub preferences: UserPreferences,
    pub font_family: String,
    pub font_size: u16,
    pub shell: String,
    pub theme: String, // Name of the active theme
    pub keybindings: KeyBindings,
    pub prompt: PromptSettings,
}

impl Default for WarpConfig {
    fn default() -> Self {
        Self {
            preferences: UserPreferences::default(),
            font_family: "Fira Code".to_string(),
            font_size: 16,
            shell: "bash".to_string(), // or "powershell.exe" on Windows
            theme: "Default Dark".to_string(),
            keybindings: KeyBindings::default(),
            prompt: PromptSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub enable_fuzzy_search: bool,
    pub enable_collaboration: bool,
    pub show_welcome_message: bool,
    pub max_history_size: usize,
    pub enable_auto_update: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            enable_fuzzy_search: true,
            enable_collaboration: false,
            show_welcome_message: true,
            max_history_size: 1000,
            enable_auto_update: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub submit_input: String,
    pub history_up: String,
    pub history_down: String,
    pub clear_terminal: String,
    pub toggle_fullscreen: String,
    pub open_command_palette: String,
    pub open_preferences: String,
    pub open_theme_customizer: String,
    pub open_profile_manager: String,
    pub open_workflow_browser: String,
    pub open_warp_drive: String, // New keybinding
    // Add more keybindings as needed
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            submit_input: "Enter".to_string(),
            history_up: "Up".to_string(),
            history_down: "Down".to_string(),
            clear_terminal: "Ctrl+L".to_string(),
            toggle_fullscreen: "F11".to_string(),
            open_command_palette: "Ctrl+P".to_string(),
            open_preferences: "Ctrl+, ".to_string(),
            open_theme_customizer: "Ctrl+T".to_string(),
            open_profile_manager: "Ctrl+Shift+P".to_string(),
            open_workflow_browser: "Ctrl+W".to_string(),
            open_warp_drive: "Ctrl+Shift+D".to_string(), // Default value
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSettings {
    pub show_user: bool,
    pub show_host: bool,
    pub show_cwd: bool,
    pub show_git_status: bool,
    pub user_symbol: String,
    pub host_symbol: String,
    pub cwd_symbol: String,
    pub git_symbol: String,
    pub prompt_symbol: String,
}

impl Default for PromptSettings {
    fn default() -> Self {
        Self {
            show_user: true,
            show_host: true,
            show_cwd: true,
            show_git_status: true,
            user_symbol: "👤".to_string(),
            host_symbol: "💻".to_string(),
            cwd_symbol: "📁".to_string(),
            git_symbol: "🌿".to_string(),
            prompt_symbol: "❯".to_string(),
        }
    }
}

pub struct ConfigManager {
    config: WarpConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = directories::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_config_dir = config_dir.join("warp-terminal-clone");
        fs::create_dir_all(&app_config_dir)?;
        let config_path = app_config_dir.join("config.json");

        let config = if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)?;
            serde_json::from_str(&config_str)?
        } else {
            let default_config = WarpConfig::default();
            let json_string = serde_json::to_string_pretty(&default_config)?;
            fs::write(&config_path, json_string)?;
            default_config
        };

        Ok(Self { config, config_path })
    }

    pub fn get_config(&self) -> &WarpConfig {
        &self.config
    }

    pub fn get_preferences(&self) -> &UserPreferences {
        &self.config.preferences
    }

    pub fn get_keybindings(&self) -> &KeyBindings {
        &self.config.keybindings
    }

    pub fn get_prompt_settings(&self) -> &PromptSettings {
        &self.config.prompt
    }

    pub fn update_config(&mut self, new_config: WarpConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = new_config;
        self.save_config()
    }

    pub fn update_preferences(&mut self, new_preferences: UserPreferences) -> Result<(), Box<dyn std::error::Error>> {
        self.config.preferences = new_preferences;
        self.save_config()
    }

    pub fn update_keybindings(&mut self, new_keybindings: KeyBindings) -> Result<(), Box<dyn std::error::Error>> {
        self.config.keybindings = new_keybindings;
        self.save_config()
    }

    pub fn update_prompt_settings(&mut self, new_prompt_settings: PromptSettings) -> Result<(), Box<dyn std::error::Error>> {
        self.config.prompt = new_prompt_settings;
        self.save_config()
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, json_string)?;
        Ok(())
    }

    pub fn default() -> Self {
        Self {
            config: WarpConfig::default(),
            config_path: PathBuf::from("config.json"), // Dummy path for default
        }
    }
}

// Placeholder for main config module
pub struct MainConfig;

impl MainConfig {
    pub fn new() -> Self {
        MainConfig
    }

    pub fn load(&self) {
        println!("Loading main configuration.");
    }
}
