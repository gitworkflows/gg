use log::info;
use uuid::Uuid;

/// Represents a workflow that can be executed.
#[derive(Debug, Clone)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub steps: Vec<String>, // Simplified: just a list of command strings
}

/// Executes workflows.
pub struct WorkflowExecutor;

impl WorkflowExecutor {
    pub fn new() -> Self {
        WorkflowExecutor {}
    }

    /// Executes a given workflow.
    /// In a real scenario, this would interact with the shell, API, etc.
    pub async fn execute(&self, workflow: &Workflow) -> anyhow::Result<()> {
        info!("Executing workflow: '{}' (ID: {})", workflow.name, workflow.id);
        for (i, step) in workflow.steps.iter().enumerate() {
            info!("  Step {}: Executing command: '{}'", i + 1, step);
            // Simulate command execution
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            info!("  Step {}: Command '{}' completed.", i + 1, step);
        }
        info!("Workflow '{}' completed.", workflow.name);
        Ok(())
    }

    /// Exports a workflow to a specified path.
    /// This is a placeholder for saving to a file format like YAML or JSON.
    pub fn export(&self, workflow: &Workflow, path: &std::path::Path) -> anyhow::Result<()> {
        info!("Exporting workflow '{}' to {:?}", workflow.name, path);
        let content = format!(
            "name: {}\nid: {}\nsteps:\n{}",
            workflow.name,
            workflow.id,
            workflow.steps.iter().map(|s| format!("  - \"{}\"", s)).collect::<Vec<_>>().join("\n")
        );
        std::fs::write(path, content)?;
        info!("Workflow exported successfully.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_workflow_execution() {
        let executor = WorkflowExecutor::new();
        let workflow = Workflow {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            steps: vec![
                "echo 'Hello'".to_string(),
                "ls -la".to_string(),
            ],
        };

        let result = executor.execute(&workflow).await;
        assert!(result.is_ok());
        // Check logs for confirmation of execution
    }

    #[tokio::test]
    async fn test_workflow_export() -> anyhow::Result<()> {
        let executor = WorkflowExecutor::new();
        let workflow = Workflow {
            id: Uuid::new_v4(),
            name: "Exportable Workflow".to_string(),
            steps: vec![
                "step 1".to_string(),
                "step 2".to_string(),
            ],
        };

        let temp_dir = tempdir()?;
        let export_path = temp_dir.path().join("exported_workflow.yaml");

        executor.export(&workflow, &export_path)?;

        assert!(export_path.exists());
        let content = fs::read_to_string(&export_path)?;
        assert!(content.contains(&workflow.name));
        assert!(content.contains(&workflow.id.to_string()));
        assert!(content.contains("- \"step 1\""));
        assert!(content.contains("- \"step 2\""));

        Ok(())
    }
}
