use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::config::{WarpConfig, UserPreferences, KeyBindings};
use crate::themes::WarpTheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub avatar_path: Option<PathBuf>,
    pub config: WarpConfig,
    pub custom_themes: HashMap<String, WarpTheme>,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub is_default: bool,
    pub tags: Vec<String>,
    pub workspace_settings: WorkspaceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    pub startup_directory: PathBuf,
    pub environment_variables: HashMap<String, String>,
    pub startup_commands: Vec<String>,
    pub session_restore: bool,
    pub window_settings: WindowSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub position: Option<(i32, i32)>,
    pub maximized: bool,
    pub always_on_top: bool,
    pub opacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesConfig {
    pub active_profile_id: Uuid,
    pub profiles: HashMap<Uuid, UserProfile>,
    pub profile_switching_enabled: bool,
    pub auto_switch_rules: Vec<AutoSwitchRule>,
    pub quick_switch_profiles: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub id: Uuid,
    pub name: String,
    pub condition: SwitchCondition,
    pub target_profile_id: Uuid,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwitchCondition {
    DirectoryPattern(String),
    TimeOfDay { start: String, end: String },
    DayOfWeek(Vec<String>),
    ProjectType(String),
    GitRepository(String),
}

impl Default for UserProfile {
    fn default() -> Self {
        UserProfile {
            id: Uuid::new_v4(),
            name: "Default".to_string(),
            description: Some("Default user profile".to_string()),
            avatar_path: None,
            config: WarpConfig::default(),
            custom_themes: HashMap::new(),
            created_at: Utc::now(),
            last_used: Utc::now(),
            is_default: true,
            tags: vec!["default".to_string()],
            workspace_settings: WorkspaceSettings::default(),
        }
    }
}

impl Default for WorkspaceSettings {
    fn default() -> Self {
        WorkspaceSettings {
            startup_directory: dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            environment_variables: HashMap::new(),
            startup_commands: vec![],
            session_restore: true,
            window_settings: WindowSettings::default(),
        }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        WindowSettings {
            width: 1200,
            height: 800,
            position: None,
            maximized: false,
            always_on_top: false,
            opacity: 1.0,
        }
    }
}

impl Default for ProfilesConfig {
    fn default() -> Self {
        let default_profile = UserProfile::default();
        let profile_id = default_profile.id;
        let mut profiles = HashMap::new();
        profiles.insert(profile_id, default_profile);

        ProfilesConfig {
            active_profile_id: profile_id,
            profiles,
            profile_switching_enabled: true,
            auto_switch_rules: vec![],
            quick_switch_profiles: vec![profile_id],
        }
    }
}

pub struct ProfileManager {
    config: ProfilesConfig,
    profiles_directory: PathBuf,
    config_path: PathBuf,
}

impl ProfileManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let profiles_directory = Self::get_profiles_directory()?;
        let config_path = profiles_directory.join("profiles.yaml");

        // Create profiles directory if it doesn't exist
        if !profiles_directory.exists() {
            fs::create_dir_all(&profiles_directory)?;
        }

        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_yaml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("Failed to parse profiles config: {}, using defaults", e);
                ProfilesConfig::default()
            })
        } else {
            let default_config = ProfilesConfig::default();
            let yaml_content = serde_yaml::to_string(&default_config)?;
            fs::write(&config_path, yaml_content)?;
            default_config
        };

        Ok(ProfileManager {
            config,
            profiles_directory,
            config_path,
        })
    }

    fn get_profiles_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        Ok(config_dir.join("warp-terminal").join("profiles"))
    }

    pub fn get_active_profile(&self) -> Option<&UserProfile> {
        self.config.profiles.get(&self.config.active_profile_id)
    }

    pub fn get_active_profile_mut(&mut self) -> Option<&mut UserProfile> {
        self.config.profiles.get_mut(&self.config.active_profile_id)
    }

    pub fn get_profile(&self, id: &Uuid) -> Option<&UserProfile> {
        self.config.profiles.get(id)
    }

    pub fn get_all_profiles(&self) -> Vec<&UserProfile> {
        self.config.profiles.values().collect()
    }

    pub fn create_profile(&mut self, name: String, description: Option<String>) -> Result<Uuid, Box<dyn std::error::Error>> {
        let profile = UserProfile {
            id: Uuid::new_v4(),
            name,
            description,
            avatar_path: None,
            config: WarpConfig::default(),
            custom_themes: HashMap::new(),
            created_at: Utc::now(),
            last_used: Utc::now(),
            is_default: false,
            tags: vec![],
            workspace_settings: WorkspaceSettings::default(),
        };

        let profile_id = profile.id;
        self.config.profiles.insert(profile_id, profile);
        self.save_config()?;
        Ok(profile_id)
    }

    pub fn duplicate_profile(&mut self, source_id: &Uuid, new_name: String) -> Result<Uuid, Box<dyn std::error::Error>> {
        let source_profile = self.config.profiles.get(source_id)
            .ok_or("Source profile not found")?
            .clone();

        let mut new_profile = source_profile;
        new_profile.id = Uuid::new_v4();
        new_profile.name = new_name;
        new_profile.created_at = Utc::now();
        new_profile.last_used = Utc::now();
        new_profile.is_default = false;

        let profile_id = new_profile.id;
        self.config.profiles.insert(profile_id, new_profile);
        self.save_config()?;
        Ok(profile_id)
    }

    pub fn delete_profile(&mut self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(profile) = self.config.profiles.get(id) {
            if profile.is_default {
                return Err("Cannot delete default profile".into());
            }
        }

        self.config.profiles.remove(id);
        
        // If we deleted the active profile, switch to default
        if self.config.active_profile_id == *id {
            if let Some(default_profile) = self.config.profiles.values().find(|p| p.is_default) {
                self.config.active_profile_id = default_profile.id;
            } else if let Some(first_profile) = self.config.profiles.values().next() {
                self.config.active_profile_id = first_profile.id;
            }
        }

        // Remove from quick switch profiles
        self.config.quick_switch_profiles.retain(|&profile_id| profile_id != *id);

        self.save_config()?;
        Ok(())
    }

    pub fn switch_profile(&mut self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.profiles.contains_key(id) {
            return Err("Profile not found".into());
        }

        self.config.active_profile_id = *id;
        
        // Update last used timestamp
        if let Some(profile) = self.config.profiles.get_mut(id) {
            profile.last_used = Utc::now();
        }

        self.save_config()?;
        Ok(())
    }

    pub fn update_profile(&mut self, id: &Uuid, profile: UserProfile) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.profiles.contains_key(id) {
            return Err("Profile not found".into());
        }

        self.config.profiles.insert(*id, profile);
        self.save_config()?;
        Ok(())
    }

    pub fn export_profile(&self, id: &Uuid, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let profile = self.config.profiles.get(id)
            .ok_or("Profile not found")?;

        let yaml_content = serde_yaml::to_string(profile)?;
        fs::write(path, yaml_content)?;
        Ok(())
    }

    pub fn import_profile(&mut self, path: &Path) -> Result<Uuid, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut profile: UserProfile = serde_yaml::from_str(&content)?;
        
        // Generate new ID to avoid conflicts
        profile.id = Uuid::new_v4();
        profile.created_at = Utc::now();
        profile.last_used = Utc::now();
        profile.is_default = false;

        let profile_id = profile.id;
        self.config.profiles.insert(profile_id, profile);
        self.save_config()?;
        Ok(profile_id)
    }

    pub fn add_auto_switch_rule(&mut self, rule: AutoSwitchRule) -> Result<(), Box<dyn std::error::Error>> {
        self.config.auto_switch_rules.push(rule);
        self.save_config()?;
        Ok(())
    }

    pub fn remove_auto_switch_rule(&mut self, rule_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.config.auto_switch_rules.retain(|rule| rule.id != *rule_id);
        self.save_config()?;
        Ok(())
    }

    pub fn check_auto_switch_rules(&self, current_directory: &Path) -> Option<Uuid> {
        for rule in &self.config.auto_switch_rules {
            if !rule.enabled {
                continue;
            }

            match &rule.condition {
                SwitchCondition::DirectoryPattern(pattern) => {
                    if current_directory.to_string_lossy().contains(pattern) {
                        return Some(rule.target_profile_id);
                    }
                }
                SwitchCondition::TimeOfDay { start, end } => {
                    let now = chrono::Local::now().time();
                    if let (Ok(start_time), Ok(end_time)) = (
                        chrono::NaiveTime::parse_from_str(start, "%H:%M"),
                        chrono::NaiveTime::parse_from_str(end, "%H:%M")
                    ) {
                        if now >= start_time && now <= end_time {
                            return Some(rule.target_profile_id);
                        }
                    }
                }
                SwitchCondition::DayOfWeek(days) => {
                    let today = chrono::Local::now().weekday().to_string();
                    if days.contains(&today) {
                        return Some(rule.target_profile_id);
                    }
                }
                SwitchCondition::ProjectType(project_type) => {
                    // Check for project files (package.json, Cargo.toml, etc.)
                    let project_files = match project_type.as_str() {
                        "rust" => vec!["Cargo.toml"],
                        "node" => vec!["package.json"],
                        "python" => vec!["requirements.txt", "pyproject.toml"],
                        "go" => vec!["go.mod"],
                        _ => vec![],
                    };

                    for file in project_files {
                        if current_directory.join(file).exists() {
                            return Some(rule.target_profile_id);
                        }
                    }
                }
                SwitchCondition::GitRepository(repo_name) => {
                    if let Ok(repo) = git2::Repository::open(current_directory) {
                        if let Ok(remote) = repo.find_remote("origin") {
                            if let Some(url) = remote.url() {
                                if url.contains(repo_name) {
                                    return Some(rule.target_profile_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn add_to_quick_switch(&mut self, profile_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.quick_switch_profiles.contains(&profile_id) {
            self.config.quick_switch_profiles.push(profile_id);
            self.save_config()?;
        }
        Ok(())
    }

    pub fn remove_from_quick_switch(&mut self, profile_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.config.quick_switch_profiles.retain(|&id| id != *profile_id);
        self.save_config()?;
        Ok(())
    }

    pub fn get_quick_switch_profiles(&self) -> Vec<&UserProfile> {
        self.config.quick_switch_profiles
            .iter()
            .filter_map(|id| self.config.profiles.get(id))
            .collect()
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = serde_yaml::to_string(&self.config)?;
        fs::write(&self.config_path, yaml_content)?;
        Ok(())
    }

    pub fn get_profiles_by_tag(&self, tag: &str) -> Vec<&UserProfile> {
        self.config.profiles
            .values()
            .filter(|profile| profile.tags.contains(&tag.to_string()))
            .collect()
    }

    pub fn search_profiles(&self, query: &str) -> Vec<&UserProfile> {
        let query = query.to_lowercase();
        self.config.profiles
            .values()
            .filter(|profile| {
                profile.name.to_lowercase().contains(&query) ||
                profile.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query)) ||
                profile.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_profile_creation() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());
        
        let mut manager = ProfileManager::new().unwrap();
        let profile_id = manager.create_profile("Test Profile".to_string(), Some("Test description".to_string())).unwrap();
        
        let profile = manager.get_profile(&profile_id).unwrap();
        assert_eq!(profile.name, "Test Profile");
        assert_eq!(profile.description, Some("Test description".to_string()));
        assert!(!profile.is_default);
    }

    #[test]
    fn test_profile_switching() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());
        
        let mut manager = ProfileManager::new().unwrap();
        let profile_id = manager.create_profile("Test Profile".to_string(), None).unwrap();
        
        manager.switch_profile(&profile_id).unwrap();
        assert_eq!(manager.config.active_profile_id, profile_id);
    }

    #[test]
    fn test_profile_duplication() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());
        
        let mut manager = ProfileManager::new().unwrap();
        let original_id = manager.create_profile("Original".to_string(), None).unwrap();
        let duplicate_id = manager.duplicate_profile(&original_id, "Duplicate".to_string()).unwrap();
        
        let original = manager.get_profile(&original_id).unwrap();
        let duplicate = manager.get_profile(&duplicate_id).unwrap();
        
        assert_eq!(duplicate.name, "Duplicate");
        assert_eq!(original.config.theme, duplicate.config.theme);
        assert_ne!(original.id, duplicate.id);
    }
}
