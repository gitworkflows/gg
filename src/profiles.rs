use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs;
use chrono::{DateTime, Utc};

use crate::config::{WarpConfig, KeyBindings, PromptSettings};
use crate::themes::WarpTheme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub config: WarpConfig,
    pub auto_switch_rules: Vec<AutoSwitchRule>,
    pub is_quick_switch: bool,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Default Profile".to_string(),
            description: Some("The default terminal profile.".to_string()),
            config: WarpConfig::default(),
            auto_switch_rules: Vec::new(),
            is_quick_switch: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub path: PathBuf,
    pub rule_type: AutoSwitchRuleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoSwitchRuleType {
    Exact,
    StartsWith,
    Contains,
}

pub struct ProfileManager {
    profiles: HashMap<Uuid, UserProfile>,
    active_profile_id: Uuid,
    profiles_dir: PathBuf,
}

impl ProfileManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let profiles_dir = PathBuf::from("profiles");
        let mut manager = Self {
            profiles: HashMap::new(),
            active_profile_id: Uuid::new_v4(), // Will be set to default or loaded active
            profiles_dir,
        };
        manager.load_profiles()?;
        Ok(manager)
    }

    fn load_profiles(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.profiles_dir.exists() {
            fs::create_dir_all(&self.profiles_dir)?;
        }

        let mut loaded_profiles = HashMap::new();
        for entry in fs::read_dir(&self.profiles_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let profile_str = fs::read_to_string(&path)?;
                let profile: UserProfile = serde_json::from_str(&profile_str)?;
                loaded_profiles.insert(profile.id, profile);
            }
        }

        if loaded_profiles.is_empty() {
            let default_profile = UserProfile::default();
            self.active_profile_id = default_profile.id;
            loaded_profiles.insert(default_profile.id, default_profile.clone());
            self.save_profile_to_file(&default_profile)?;
        } else {
            // Try to find a previously active profile, or pick the first one
            self.active_profile_id = *loaded_profiles.keys().next().unwrap();
            // TODO: Load active profile ID from a separate state file
        }
        self.profiles = loaded_profiles;
        Ok(())
    }

    fn save_profile_to_file(&self, profile: &UserProfile) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{}.json", profile.id);
        let path = self.profiles_dir.join(file_name);
        let json_string = serde_json::to_string_pretty(profile)?;
        fs::write(path, json_string)?;
        Ok(())
    }

    fn delete_profile_file(&self, profile_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let file_name = format!("{}.json", profile_id);
        let path = self.profiles_dir.join(file_name);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn get_active_profile(&self) -> Option<&UserProfile> {
        self.profiles.get(&self.active_profile_id)
    }

    pub fn switch_profile(&mut self, profile_id: &Uuid) -> Result<(), String> {
        if self.profiles.contains_key(profile_id) {
            self.active_profile_id = *profile_id;
            // TODO: Save active profile ID to a state file
            Ok(())
        } else {
            Err(format!("Profile with ID {} not found.", profile_id))
        }
    }

    pub fn create_profile(&mut self, name: String, description: Option<String>) -> Result<Uuid, Box<dyn std::error::Error>> {
        let mut new_profile = UserProfile {
            id: Uuid::new_v4(),
            name,
            description,
            config: WarpConfig::default(), // Default config for new profile
            auto_switch_rules: Vec::new(),
            is_quick_switch: false,
        };
        // Inherit current active profile's config
        if let Some(active_profile) = self.get_active_profile() {
            new_profile.config = active_profile.config.clone();
        }

        self.profiles.insert(new_profile.id, new_profile.clone());
        self.save_profile_to_file(&new_profile)?;
        Ok(new_profile.id)
    }

    pub fn update_profile(&mut self, id: &Uuid, updated_profile: UserProfile) -> Result<(), Box<dyn std::error::Error>> {
        if self.profiles.contains_key(id) {
            self.profiles.insert(*id, updated_profile.clone());
            self.save_profile_to_file(&updated_profile)?;
            Ok(())
        } else {
            Err(format!("Profile with ID {} not found.", id).into())
        }
    }

    pub fn duplicate_profile(&mut self, source_id: &Uuid, new_name: String) -> Result<Uuid, Box<dyn std::error::Error>> {
        if let Some(source_profile) = self.profiles.get(source_id) {
            let mut new_profile = source_profile.clone();
            new_profile.id = Uuid::new_v4();
            new_profile.name = new_name;
            new_profile.is_quick_switch = false; // Duplicates are not quick-switch by default
            self.profiles.insert(new_profile.id, new_profile.clone());
            self.save_profile_to_file(&new_profile)?;
            Ok(new_profile.id)
        } else {
            Err(format!("Source profile with ID {} not found.", source_id).into())
        }
    }

    pub fn delete_profile(&mut self, profile_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if self.profiles.len() <= 1 {
            return Err("Cannot delete the last profile.".into());
        }
        if self.active_profile_id == *profile_id {
            // Switch to another profile before deleting the active one
            let new_active_id = *self.profiles.keys().find(|&id| id != profile_id).unwrap();
            self.switch_profile(&new_active_id)?;
        }
        self.profiles.remove(profile_id);
        self.delete_profile_file(profile_id)?;
        Ok(())
    }

    pub fn get_all_profiles(&self) -> Vec<&UserProfile> {
        self.profiles.values().collect()
    }

    pub fn get_quick_switch_profiles(&self) -> Vec<&UserProfile> {
        self.profiles.values().filter(|p| p.is_quick_switch).collect()
    }

    pub fn add_to_quick_switch(&mut self, profile_id: Uuid) -> Result<(), String> {
        if let Some(profile) = self.profiles.get_mut(&profile_id) {
            profile.is_quick_switch = true;
            self.save_profile_to_file(profile).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Profile with ID {} not found.", profile_id))
        }
    }

    pub fn remove_from_quick_switch(&mut self, profile_id: &Uuid) -> Result<(), String> {
        if let Some(profile) = self.profiles.get_mut(profile_id) {
            profile.is_quick_switch = false;
            self.save_profile_to_file(profile).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Profile with ID {} not found.", profile_id))
        }
    }

    pub fn add_auto_switch_rule(&mut self, profile_id: Uuid, rule: AutoSwitchRule) -> Result<(), String> {
        if let Some(profile) = self.profiles.get_mut(&profile_id) {
            profile.auto_switch_rules.push(rule);
            self.save_profile_to_file(profile).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Profile with ID {} not found.", profile_id))
        }
    }

    pub fn remove_auto_switch_rule(&mut self, profile_id: Uuid, index: usize) -> Result<(), String> {
        if let Some(profile) = self.profiles.get_mut(&profile_id) {
            if index < profile.auto_switch_rules.len() {
                profile.auto_switch_rules.remove(index);
                self.save_profile_to_file(profile).map_err(|e| e.to_string())?;
                Ok(())
            } else {
                Err("Rule index out of bounds.".to_string())
            }
        } else {
            Err(format!("Profile with ID {} not found.", profile_id))
        }
    }

    pub fn check_auto_switch_rules(&self, current_path: &Path) -> Option<Uuid> {
        for profile in self.profiles.values() {
            for rule in &profile.auto_switch_rules {
                let rule_path_str = rule.path.to_string_lossy().to_lowercase();
                let current_path_str = current_path.to_string_lossy().to_lowercase();

                let matches = match rule.rule_type {
                    AutoSwitchRuleType::Exact => current_path_str == rule_path_str,
                    AutoSwitchRuleType::StartsWith => current_path_str.starts_with(&rule_path_str),
                    AutoSwitchRuleType::Contains => current_path_str.contains(&rule_path_str),
                };

                if matches {
                    return Some(profile.id);
                }
            }
        }
        None
    }
}
