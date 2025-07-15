use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyBindings {
    pub open_warp_drive: String,
    pub toggle_command_palette: String,
    // Add more keybindings as needed
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            open_warp_drive: "Ctrl+Shift+D".to_string(),
            toggle_command_palette: "Ctrl+P".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub key_bindings: KeyBindings,
    pub font_size: u16,
    pub user_preferences: UserPreferences,
    // Add more preference settings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub enable_fuzzy_search: bool,
    pub enable_collaboration: bool,
    pub show_welcome_message: bool,
    pub max_history_size: usize,
    pub enable_auto_update: bool,
    pub enable_telemetry: bool, // Added telemetry preference
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            enable_fuzzy_search: true,
            enable_collaboration: false,
            show_welcome_message: true,
            max_history_size: 1000,
            enable_auto_update: true,
            enable_telemetry: true, // Default to true
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            key_bindings: KeyBindings::default(),
            font_size: 14,
            user_preferences: UserPreferences::default(),
        }
    }
}

pub struct PreferencesManager {
    preferences: Preferences,
}

impl PreferencesManager {
    pub fn new() -> Self {
        // In a real app, load from a config file
        PreferencesManager {
            preferences: Preferences::default(),
        }
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn update_key_binding(&mut self, action: &str, new_binding: String) {
        match action {
            "open_warp_drive" => self.preferences.key_bindings.open_warp_drive = new_binding,
            "toggle_command_palette" => self.preferences.key_bindings.toggle_command_palette = new_binding,
            _ => log::warn!("Unknown keybinding action: {}", action),
        }
    }

    pub fn update_user_preference(&mut self, preference: &str, value: bool) {
        match preference {
            "enable_fuzzy_search" => self.preferences.user_preferences.enable_fuzzy_search = value,
            "enable_collaboration" => self.preferences.user_preferences.enable_collaboration = value,
            "show_welcome_message" => self.preferences.user_preferences.show_welcome_message = value,
            "enable_auto_update" => self.preferences.user_preferences.enable_auto_update = value,
            _ => log::warn!("Unknown user preference: {}", preference),
        }
    }

    pub fn update_max_history_size(&mut self, size: usize) {
        self.preferences.user_preferences.max_history_size = size;
    }
}
