use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use regex::Regex;
use anyhow::{Result, anyhow};
use log::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub commands: Vec<String>,
    // Add more workflow properties like variables, inputs, etc.
}

pub struct WorkflowManager {
    workflows: HashMap<String, Workflow>,
    workflows_dir: PathBuf,
}

impl WorkflowManager {
    pub fn new() -> Result<Self> {
        let workflows_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("warp-terminal-clone")
            .join("workflows");
        
        let mut manager = WorkflowManager {
            workflows: HashMap::new(),
            workflows_dir,
        };
        manager.load_workflows()?;
        info!("WorkflowManager initialized with {} workflows.", manager.workflows.len());
        Ok(manager)
    }

    pub fn load_workflows(&mut self) -> Result<()> {
        self.workflows.clear();
        if !self.workflows_dir.exists() {
            fs::create_dir_all(&self.workflows_dir)?;
        }

        for entry in fs::read_dir(&self.workflows_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                let workflow_str = fs::read_to_string(&path)?;
                let workflow: Workflow = serde_json::from_str(&workflow_str)?;
                self.workflows.insert(workflow.name.clone(), workflow);
            }
        }
        Ok(())
    }

    fn save_workflow_to_file(&self, workflow: &Workflow) -> Result<()> {
        let file_name = format!("{}.json", workflow.name);
        let path = self.workflows_dir.join(file_name);
        let json_string = serde_json::to_string_pretty(workflow)?;
        fs::write(path, json_string)?;
        Ok(())
    }

    fn delete_workflow_file(&self, name: &str) -> Result<()> {
        let file_name = format!("{}.json", name);
        let path = self.workflows_dir.join(file_name);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn get_workflow(&self, name: &str) -> Option<&Workflow> {
        self.workflows.get(name)
    }

    pub fn list_workflows(&self) -> Vec<String> {
        self.workflows.keys().cloned().collect()
    }

    pub fn remove_workflow(&mut self, name: &str) -> Result<()> {
        if self.workflows.remove(name).is_some() {
            self.delete_workflow_file(name)?;
            info!("Removed workflow: {}", name);
            Ok(())
        } else {
            error!("Workflow '{}' not found for removal.", name);
            Err(anyhow!("Workflow '{}' not found.", name))
        }
    }

    pub fn add_workflow(&mut self, workflow: Workflow) {
        info!("Adding workflow: {}", workflow.name);
        self.workflows.insert(workflow.name.clone(), workflow);
    }

    pub fn execute_workflow(&self, name: &str, args: HashMap<String, String>) -> Result<String, String> {
        if let Some(workflow) = self.workflows.get(name) {
            let mut command = workflow.commands[0].clone();
            for (arg_name, _desc) in workflow.commands.iter().flat_map(|cmd| {
                Regex::new(r"\$\{([^}]+)\}").unwrap().captures_iter(cmd)
                    .map(|cap| cap[1].to_string())
            }) {
                if let Some(value) = args.get(&arg_name) {
                    command = command.replace(&format!("${{{}}}", arg_name), value);
                } else {
                    return Err(format!("Missing argument: {}", arg_name));
                }
            }
            Ok(command)
        } else {
            Err(format!("Workflow '{}' not found.", name))
        }
    }

    pub fn import_workflow_from_path(&mut self, path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let imported_workflow: Workflow = serde_json::from_str(&file_content)?;
        
        // Generate a new name to avoid conflicts if importing the same workflow multiple times
        let original_name = imported_workflow.name.clone();
        let mut new_workflow = imported_workflow;
        new_workflow.name = format!("Imported: {}", new_workflow.name);

        self.workflows.insert(new_workflow.name.clone(), new_workflow.clone());
        self.save_workflow_to_file(&new_workflow)?;
        
        println!("Successfully imported workflow: {} (new name: {})", original_name, new_workflow.name);
        Ok(vec![new_workflow.name])
    }

    pub fn export_workflow_to_path(&self, name: &str, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(workflow) = self.workflows.get(name) {
            let json_string = serde_json::to_string_pretty(workflow)?;
            fs::write(path, json_string)?;
            println!("Successfully exported workflow {} to {:?}", workflow.name, path);
            Ok(())
        } else {
            Err(format!("Workflow '{}' not found.", name).into())
        }
    }
}
