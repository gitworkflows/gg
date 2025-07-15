pub mod executor;
pub mod manager;
pub mod ui;

// Re-export key structs for easier access
pub use executor::{Workflow, WorkflowExecutor};
pub use manager::WorkflowManager;
pub use ui::WorkflowBrowser;
