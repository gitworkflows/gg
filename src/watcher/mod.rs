// This module would contain logic for watching file system changes,
// useful for features like auto-reloading configurations, triggering
// actions on file modifications, or updating directory listings.

pub mod service; // For the actual file watching service

#[derive(Debug, Clone)]
pub enum FileWatcherEvent {
    FileChanged(PathBuf),
    FileCreated(PathBuf),
    FileDeleted(PathBuf),
    DirectoryChanged(PathBuf),
    Error(String),
}
