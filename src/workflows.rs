use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub arguments: Option<HashMap<String, String>>, // Argument name -> description
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub created_at: DateTime<Utc>, // New field for creation timestamp
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Workflow".to_string(),
            description: None,
            command: "".to_string(),
            arguments: None,
            tags: Vec::new(),
            is_favorite: false,
            created_at: Utc::now(), // Initialize with current time
        }
    }
}

// This file is now deprecated and its content has been moved to:
// - src/workflows/manager.rs (for WorkflowManager struct and logic)
// - src/workflows/executor.rs (for WorkflowExecutor logic)
// - src/workflows/ui.rs (for WorkflowBrowser UI)

// You should remove this file from your project's module declarations in main.rs
// and update any imports that were pointing here.
