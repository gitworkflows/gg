use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub command: String,
    pub tags: Option<Vec<String>>,
    pub description: Option<String>,
    pub source_url: Option<String>,
    pub author: Option<String>,
    pub author_url: Option<String>,
    pub shells: Option<Vec<Shell>>,
    pub arguments: Option<Vec<WorkflowArgument>>,
    
    // Internal metadata
    #[serde(skip)]
    pub id: Uuid,
    #[serde(skip)]
    pub file_path: Option<PathBuf>,
    #[serde(skip)]
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub last_used: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub usage_count: u32,
    #[serde(skip)]
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowArgument {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub argument_type: Option<ArgumentType>,
    pub required: Option<bool>,
    pub validation: Option<ArgumentValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    String,
    Number,
    Boolean,
    File,
    Directory,
    Url,
    Email,
    Choice(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentValidation {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
    PowerShell,
}

#[derive(Debug, Clone)]
pub struct WorkflowExecution {
    pub workflow_id: Uuid,
    pub resolved_command: String,
    pub arguments: HashMap<String, String>,
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkflowCollection {
    pub name: String,
    pub description: Option<String>,
    pub workflows: Vec<Workflow>,
    pub source_url: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
}

pub struct WorkflowManager {
    workflows: HashMap<Uuid, Workflow>,
    collections: Vec<WorkflowCollection>,
    workflows_directory: PathBuf,
    execution_history: Vec<WorkflowExecution>,
    favorites: Vec<Uuid>,
    recent_workflows: Vec<Uuid>,
}

impl Default for Workflow {
    fn default() -> Self {
        Workflow {
            name: String::new(),
            command: String::new(),
            tags: None,
            description: None,
            source_url: None,
            author: None,
            author_url: None,
            shells: None,
            arguments: None,
            id: Uuid::new_v4(),
            file_path: None,
            created_at: Utc::now(),
            last_used: None,
            usage_count: 0,
            is_favorite: false,
        }
    }
}

impl WorkflowManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let workflows_directory = Self::get_workflows_directory()?;
        
        // Create workflows directory if it doesn't exist
        if !workflows_directory.exists() {
            fs::create_dir_all(&workflows_directory)?;
            Self::create_default_workflows(&workflows_directory)?;
        }

        let mut manager = WorkflowManager {
            workflows: HashMap::new(),
            collections: Vec::new(),
            workflows_directory,
            execution_history: Vec::new(),
            favorites: Vec::new(),
            recent_workflows: Vec::new(),
        };

        manager.load_workflows()?;
        manager.load_builtin_workflows();
        
        Ok(manager)
    }

    fn get_workflows_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        Ok(config_dir.join("warp-terminal").join("workflows"))
    }

    pub fn load_workflows(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.workflows.clear();
        
        if !self.workflows_directory.exists() {
            return Ok(());
        }

        self.load_workflows_from_directory(&self.workflows_directory)?;
        
        // Load from subdirectories (collections)
        for entry in fs::read_dir(&self.workflows_directory)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.load_collection_from_directory(&path)?;
            }
        }

        Ok(())
    }

    fn load_workflows_from_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "yml" || extension == "yaml" {
                        if let Err(e) = self.load_workflow_from_file(&path) {
                            eprintln!("Failed to load workflow from {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn load_collection_from_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let collection_name = dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let mut collection = WorkflowCollection {
            name: collection_name,
            description: None,
            workflows: Vec::new(),
            source_url: None,
            version: None,
            author: None,
        };

        // Load collection metadata if exists
        let metadata_path = dir.join("collection.yml");
        if metadata_path.exists() {
            let content = fs::read_to_string(&metadata_path)?;
            if let Ok(metadata) = serde_yaml::from_str::<WorkflowCollection>(&content) {
                collection.description = metadata.description;
                collection.source_url = metadata.source_url;
                collection.version = metadata.version;
                collection.author = metadata.author;
            }
        }

        // Load workflows from collection directory
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.file_name() != Some(std::ffi::OsStr::new("collection.yml")) {
                if let Some(extension) = path.extension() {
                    if extension == "yml" || extension == "yaml" {
                        if let Ok(workflow) = self.load_workflow_from_file(&path) {
                            collection.workflows.push(workflow.clone());
                        }
                    }
                }
            }
        }

        if !collection.workflows.is_empty() {
            self.collections.push(collection);
        }

        Ok(())
    }

    pub fn load_workflow_from_file(&mut self, path: &Path) -> Result<Workflow, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut workflow: Workflow = serde_yaml::from_str(&content)?;
        
        workflow.id = Uuid::new_v4();
        workflow.file_path = Some(path.to_path_buf());
        workflow.created_at = Utc::now();
        
        self.workflows.insert(workflow.id, workflow.clone());
        Ok(workflow)
    }

    pub fn save_workflow(&self, workflow: &Workflow) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = if let Some(path) = &workflow.file_path {
            path.clone()
        } else {
            self.workflows_directory.join(format!("{}.yml", workflow.name.replace(' ', "_").to_lowercase()))
        };

        let yaml_content = serde_yaml::to_string(workflow)?;
        fs::write(file_path, yaml_content)?;
        Ok(())
    }

    pub fn create_workflow(&mut self, workflow: Workflow) -> Result<Uuid, Box<dyn std::error::Error>> {
        let workflow_id = workflow.id;
        self.save_workflow(&workflow)?;
        self.workflows.insert(workflow_id, workflow);
        Ok(workflow_id)
    }

    pub fn delete_workflow(&mut self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(workflow) = self.workflows.get(id) {
            if let Some(file_path) = &workflow.file_path {
                if file_path.exists() {
                    fs::remove_file(file_path)?;
                }
            }
        }
        
        self.workflows.remove(id);
        self.favorites.retain(|&fav_id| fav_id != *id);
        self.recent_workflows.retain(|&recent_id| recent_id != *id);
        
        Ok(())
    }

    pub fn search_workflows(&self, query: &str) -> Vec<&Workflow> {
        let query = query.to_lowercase();
        
        self.workflows
            .values()
            .filter(|workflow| {
                workflow.name.to_lowercase().contains(&query) ||
                workflow.command.to_lowercase().contains(&query) ||
                workflow.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query)) ||
                workflow.tags.as_ref().map_or(false, |tags| {
                    tags.iter().any(|tag| tag.to_lowercase().contains(&query))
                })
            })
            .collect()
    }

    pub fn get_workflows_by_tag(&self, tag: &str) -> Vec<&Workflow> {
        self.workflows
            .values()
            .filter(|workflow| {
                workflow.tags.as_ref().map_or(false, |tags| {
                    tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
                })
            })
            .collect()
    }

    pub fn get_workflows_by_shell(&self, shell: &Shell) -> Vec<&Workflow> {
        self.workflows
            .values()
            .filter(|workflow| {
                workflow.shells.as_ref().map_or(true, |shells| shells.contains(shell))
            })
            .collect()
    }

    pub fn get_all_workflows(&self) -> Vec<&Workflow> {
        self.workflows.values().collect()
    }

    pub fn get_workflow(&self, id: &Uuid) -> Option<&Workflow> {
        self.workflows.get(id)
    }

    pub fn get_favorites(&self) -> Vec<&Workflow> {
        self.favorites
            .iter()
            .filter_map(|id| self.workflows.get(id))
            .collect()
    }

    pub fn get_recent_workflows(&self) -> Vec<&Workflow> {
        self.recent_workflows
            .iter()
            .filter_map(|id| self.workflows.get(id))
            .collect()
    }

    pub fn add_to_favorites(&mut self, id: Uuid) {
        if !self.favorites.contains(&id) {
            self.favorites.push(id);
        }
        
        if let Some(workflow) = self.workflows.get_mut(&id) {
            workflow.is_favorite = true;
        }
    }

    pub fn remove_from_favorites(&mut self, id: &Uuid) {
        self.favorites.retain(|&fav_id| fav_id != *id);
        
        if let Some(workflow) = self.workflows.get_mut(id) {
            workflow.is_favorite = false;
        }
    }

    pub fn resolve_workflow_command(&self, workflow: &Workflow, arguments: &HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
        let mut resolved_command = workflow.command.clone();
        
        if let Some(workflow_args) = &workflow.arguments {
            for arg in workflow_args {
                let placeholder = format!("{{{{{}}}}}", arg.name);
                let value = arguments.get(&arg.name)
                    .or(arg.default_value.as_ref())
                    .ok_or(format!("Missing required argument: {}", arg.name))?;
                
                // Validate argument if validation rules exist
                if let Some(validation) = &arg.validation {
                    self.validate_argument(value, validation)?;
                }
                
                resolved_command = resolved_command.replace(&placeholder, value);
            }
        }
        
        // Check for any remaining unresolved placeholders
        let placeholder_regex = Regex::new(r"\{\{[^}]+\}\}").unwrap();
        if placeholder_regex.is_match(&resolved_command) {
            return Err("Command contains unresolved placeholders".into());
        }
        
        Ok(resolved_command)
    }

    fn validate_argument(&self, value: &str, validation: &ArgumentValidation) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(pattern) = &validation.pattern {
            let regex = Regex::new(pattern)?;
            if !regex.is_match(value) {
                return Err(format!("Value '{}' does not match pattern '{}'", value, pattern).into());
            }
        }
        
        if let Some(min_len) = validation.min_length {
            if value.len() < min_len {
                return Err(format!("Value must be at least {} characters long", min_len).into());
            }
        }
        
        if let Some(max_len) = validation.max_length {
            if value.len() > max_len {
                return Err(format!("Value must be at most {} characters long", max_len).into());
            }
        }
        
        Ok(())
    }

    pub fn execute_workflow(&mut self, workflow_id: Uuid, arguments: HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
        let workflow = self.workflows.get(&workflow_id)
            .ok_or("Workflow not found")?
            .clone();
        
        let resolved_command = self.resolve_workflow_command(&workflow, &arguments)?;
        
        // Update workflow usage statistics
        if let Some(workflow) = self.workflows.get_mut(&workflow_id) {
            workflow.last_used = Some(Utc::now());
            workflow.usage_count += 1;
        }
        
        // Add to recent workflows
        self.recent_workflows.retain(|&id| id != workflow_id);
        self.recent_workflows.insert(0, workflow_id);
        if self.recent_workflows.len() > 20 {
            self.recent_workflows.truncate(20);
        }
        
        // Record execution
        let execution = WorkflowExecution {
            workflow_id,
            resolved_command: resolved_command.clone(),
            arguments,
            executed_at: Utc::now(),
            success: true, // Will be updated based on actual execution
            output: None,
            error: None,
        };
        
        self.execution_history.push(execution);
        
        Ok(resolved_command)
    }

    pub fn import_workflow_from_url(&mut self, url: &str) -> Result<Uuid, Box<dyn std::error::Error>> {
        // This would fetch the workflow from a URL
        // For now, we'll simulate it
        let workflow = Workflow {
            name: "Imported Workflow".to_string(),
            command: "echo 'Imported from URL'".to_string(),
            source_url: Some(url.to_string()),
            ..Default::default()
        };
        
        self.create_workflow(workflow)
    }

    pub fn export_workflow(&self, id: &Uuid, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let workflow = self.workflows.get(id)
            .ok_or("Workflow not found")?;
        
        let yaml_content = serde_yaml::to_string(workflow)?;
        fs::write(path, yaml_content)?;
        Ok(())
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::new();
        
        for workflow in self.workflows.values() {
            if let Some(workflow_tags) = &workflow.tags {
                for tag in workflow_tags {
                    tags.insert(tag.clone());
                }
            }
        }
        
        let mut sorted_tags: Vec<String> = tags.into_iter().collect();
        sorted_tags.sort();
        sorted_tags
    }

    pub fn get_collections(&self) -> &Vec<WorkflowCollection> {
        &self.collections
    }

    fn create_default_workflows(workflows_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let default_workflows = vec![
            Workflow {
                name: "Git Status".to_string(),
                command: "git status".to_string(),
                tags: Some(vec!["git".to_string()]),
                description: Some("Show the working tree status".to_string()),
                shells: Some(vec![Shell::Bash, Shell::Zsh, Shell::Fish]),
                ..Default::default()
            },
            Workflow {
                name: "Find Files".to_string(),
                command: "find {{directory}} -name '{{pattern}}'".to_string(),
                tags: Some(vec!["search", "files".to_string()]),
                description: Some("Find files matching a pattern".to_string()),
                arguments: Some(vec![
                    WorkflowArgument {
                        name: "directory".to_string(),
                        description: Some("Directory to search in".to_string()),
                        default_value: Some(".".to_string()),
                        argument_type: Some(ArgumentType::Directory),
                        required: Some(false),
                        validation: None,
                    },
                    WorkflowArgument {
                        name: "pattern".to_string(),
                        description: Some("File name pattern to search for".to_string()),
                        default_value: None,
                        argument_type: Some(ArgumentType::String),
                        required: Some(true),
                        validation: None,
                    },
                ]),
                ..Default::default()
            },
            Workflow {
                name: "Docker List Containers".to_string(),
                command: "docker ps {{flags}}".to_string(),
                tags: Some(vec!["docker", "containers".to_string()]),
                description: Some("List Docker containers".to_string()),
                arguments: Some(vec![
                    WorkflowArgument {
                        name: "flags".to_string(),
                        description: Some("Additional flags for docker ps".to_string()),
                        default_value: Some("-a".to_string()),
                        argument_type: Some(ArgumentType::Choice(vec![
                            "-a".to_string(),
                            "--all".to_string(),
                            "-q".to_string(),
                            "--quiet".to_string(),
                        ])),
                        required: Some(false),
                        validation: None,
                    },
                ]),
                ..Default::default()
            },
        ];

        for workflow in default_workflows {
            let file_path = workflows_dir.join(format!("{}.yml", workflow.name.replace(' ', "_").to_lowercase()));
            let yaml_content = serde_yaml::to_string(&workflow)?;
            fs::write(file_path, yaml_content)?;
        }

        Ok(())
    }

    fn load_builtin_workflows(&mut self) {
        // Add more built-in workflows that don't need files
        let builtin_workflows = vec![
            Workflow {
                name: "System Info".to_string(),
                command: "uname -a && uptime".to_string(),
                tags: Some(vec!["system", "info".to_string()]),
                description: Some("Display system information and uptime".to_string()),
                ..Default::default()
            },
            Workflow {
                name: "Disk Usage".to_string(),
                command: "df -h".to_string(),
                tags: Some(vec!["system", "disk".to_string()]),
                description: Some("Show disk usage in human-readable format".to_string()),
                ..Default::default()
            },
            Workflow {
                name: "Process Tree".to_string(),
                command: "ps aux | head -20".to_string(),
                tags: Some(vec!["system", "processes".to_string()]),
                description: Some("Show running processes".to_string()),
                ..Default::default()
            },
        ];

        for workflow in builtin_workflows {
            self.workflows.insert(workflow.id, workflow);
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::Zsh => write!(f, "zsh"),
            Shell::Bash => write!(f, "bash"),
            Shell::Fish => write!(f, "fish"),
            Shell::PowerShell => write!(f, "powershell"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_workflow_parsing() {
        let yaml_content = r#"
name: Test Workflow
command: echo {{message}}
tags: ["test", "example"]
description: A test workflow
arguments:
  - name: message
    description: Message to echo
    default_value: "Hello World"
"#;

        let workflow: Workflow = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(workflow.name, "Test Workflow");
        assert_eq!(workflow.command, "echo {{message}}");
        assert!(workflow.tags.is_some());
        assert!(workflow.arguments.is_some());
    }

    #[test]
    fn test_command_resolution() {
        let workflow = Workflow {
            name: "Test".to_string(),
            command: "echo {{msg}} > {{file}}".to_string(),
            arguments: Some(vec![
                WorkflowArgument {
                    name: "msg".to_string(),
                    description: None,
                    default_value: Some("hello".to_string()),
                    argument_type: None,
                    required: None,
                    validation: None,
                },
                WorkflowArgument {
                    name: "file".to_string(),
                    description: None,
                    default_value: None,
                    argument_type: None,
                    required: Some(true),
                    validation: None,
                },
            ]),
            ..Default::default()
        };

        let manager = WorkflowManager::new().unwrap();
        let mut args = HashMap::new();
        args.insert("file".to_string(), "output.txt".to_string());

        let resolved = manager.resolve_workflow_command(&workflow, &args).unwrap();
        assert_eq!(resolved, "echo hello > output.txt");
    }
}
