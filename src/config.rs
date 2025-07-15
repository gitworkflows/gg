use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PromptStyle {
    Warp,
    Shell,
}

impl Default for PromptStyle {
    fn default() -> Self {
        PromptStyle::Warp
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSettings {
    pub style: PromptStyle,
    pub same_line_prompt: bool,
    pub context_chips: Vec<String>, // e.g., "cwd", "git", "kubernetes", "time"
}

impl Default for PromptSettings {
    fn default() -> Self {
        PromptSettings {
            style: PromptStyle::Warp,
            same_line_prompt: false,
            context_chips: vec![
                "cwd".to_string(),
                "git".to_string(),
                "time".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpConfig {
    pub theme: String,
    pub font_size: u16,
    pub font_family: String,
    pub shell: String,
    pub startup_commands: Vec<String>,
    pub keybindings: KeyBindings,
    pub preferences: UserPreferences,
    pub custom_themes: HashMap<String, CustomThemeOverrides>,
    pub prompt: PromptSettings, // Add this line
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub auto_save_session: bool,
    pub show_timestamps: bool,
    pub enable_fuzzy_search: bool,
    pub max_history_size: usize,
    pub scroll_sensitivity: f32,
    pub animation_speed: f32,
    pub blur_background: bool,
    pub transparency: f32,
    pub cursor_style: CursorStyle,
    pub tab_behavior: TabBehavior,
    pub notification_settings: NotificationSettings,
    pub performance: PerformanceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeOverrides {
    pub accent: Option<String>,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub terminal_colors: Option<TerminalColorOverrides>,
    pub ui_elements: Option<UiElementColors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalColorOverrides {
    pub bright: Option<ColorPaletteOverrides>,
    pub normal: Option<ColorPaletteOverrides>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPaletteOverrides {
    pub black: Option<String>,
    pub red: Option<String>,
    pub green: Option<String>,
    pub yellow: Option<String>,
    pub blue: Option<String>,
    pub magenta: Option<String>,
    pub cyan: Option<String>,
    pub white: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiElementColors {
    pub button_background: Option<String>,
    pub button_hover: Option<String>,
    pub input_background: Option<String>,
    pub border_color: Option<String>,
    pub selection_color: Option<String>,
    pub error_color: Option<String>,
    pub warning_color: Option<String>,
    pub success_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub new_tab: String,
    pub close_tab: String,
    pub next_tab: String,
    pub prev_tab: String,
    pub clear_screen: String,
    pub copy: String,
    pub paste: String,
    pub search: String,
    pub preferences: String,
    pub theme_selector: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Underline,
    Beam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabBehavior {
    pub close_on_exit: bool,
    pub confirm_close: bool,
    pub new_tab_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enable_notifications: bool,
    pub command_completion: bool,
    pub error_notifications: bool,
    pub sound_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub max_fps: u32,
    pub gpu_acceleration: bool,
    pub memory_limit_mb: usize,
    pub lazy_rendering: bool,
    pub buffer_size: usize,
}

impl Default for WarpConfig {
    fn default() -> Self {
        WarpConfig {
            theme: "default".to_string(),
            font_size: 14,
            font_family: "JetBrains Mono".to_string(),
            shell: "zsh".to_string(),
            startup_commands: vec![],
            keybindings: KeyBindings::default(),
            preferences: UserPreferences::default(),
            custom_themes: HashMap::new(),
            prompt: PromptSettings::default(), // Add this line
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        UserPreferences {
            auto_save_session: true,
            show_timestamps: true,
            enable_fuzzy_search: true,
            max_history_size: 1000,
            scroll_sensitivity: 1.0,
            animation_speed: 1.0,
            blur_background: false,
            transparency: 1.0,
            cursor_style: CursorStyle::Block,
            tab_behavior: TabBehavior::default(),
            notification_settings: NotificationSettings::default(),
            performance: PerformanceSettings::default(),
        }
    }
}

impl Default for TabBehavior {
    fn default() -> Self {
        TabBehavior {
            close_on_exit: true,
            confirm_close: false,
            new_tab_directory: "~".to_string(),
        }
    }
}

impl Default for NotificationSettings {
    fn default() -> Self {
        NotificationSettings {
            enable_notifications: true,
            command_completion: false,
            error_notifications: true,
            sound_enabled: false,
        }
    }
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        PerformanceSettings {
            max_fps: 60,
            gpu_acceleration: true,
            memory_limit_mb: 512,
            lazy_rendering: true,
            buffer_size: 10000,
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            new_tab: "Ctrl+T".to_string(),
            close_tab: "Ctrl+W".to_string(),
            next_tab: "Ctrl+Tab".to_string(),
            prev_tab: "Ctrl+Shift+Tab".to_string(),
            clear_screen: "Ctrl+L".to_string(),
            copy: "Ctrl+C".to_string(),
            paste: "Ctrl+V".to_string(),
            search: "Ctrl+F".to_string(),
            preferences: "Ctrl+Comma".to_string(),
            theme_selector: "Ctrl+Shift+T".to_string(),
        }
    }
}

pub struct ConfigManager {
    config: WarpConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        let config_path = config_dir.join("warp-terminal").join("config.yaml");
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_yaml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("Failed to parse config: {}, using defaults", e);
                WarpConfig::default()
            })
        } else {
            let default_config = WarpConfig::default();
            let yaml_content = serde_yaml::to_string(&default_config)?;
            fs::write(&config_path, yaml_content)?;
            default_config
        };

        Ok(ConfigManager {
            config,
            config_path,
        })
    }

    pub fn get_config(&self) -> &WarpConfig {
        &self.config
    }

    pub fn get_preferences(&self) -> &UserPreferences {
        &self.config.preferences
    }

    pub fn update_config(&mut self, config: WarpConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config;
        self.save_config()
    }

    pub fn update_preferences(&mut self, preferences: UserPreferences) -> Result<(), Box<dyn std::error::Error>> {
        self.config.preferences = preferences;
        self.save_config()
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = serde_yaml::to_string(&self.config)?;
        fs::write(&self.config_path, yaml_content)?;
        Ok(())
    }

    pub fn set_theme(&mut self, theme_name: String) -> Result<(), Box<dyn std::error::Error>> {
        self.config.theme = theme_name;
        self.save_config()
    }

    pub fn add_custom_theme_override(&mut self, theme_name: String, overrides: CustomThemeOverrides) -> Result<(), Box<dyn std::error::Error>> {
        self.config.custom_themes.insert(theme_name, overrides);
        self.save_config()
    }

    pub fn get_custom_theme_override(&self, theme_name: &str) -> Option<&CustomThemeOverrides> {
        self.config.custom_themes.get(theme_name)
    }

    pub fn get_prompt_settings(&self) -> &PromptSettings {
        &self.config.prompt
    }

    pub fn update_prompt_settings(&mut self, prompt_settings: PromptSettings) -> Result<(), Box<dyn std::error::Error>> {
        self.config.prompt = prompt_settings;
        self.save_config()
    }
}
