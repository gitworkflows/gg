use std::collections::HashMap;
use uuid::Uuid;
use log::info;

// Re-export WarpDriveItem from warp_drive_ui for consistency
pub use crate::warp_drive_ui::WarpDriveItem;

/// Manages all items stored in Warp Drive (workflows, notebooks, env vars, etc.)
pub struct DriveManager {
    items: HashMap<Uuid, WarpDriveItem>,
}

impl DriveManager {
    /// Creates a new, empty DriveManager.
    pub fn new() -> Self {
        DriveManager {
            items: HashMap::new(),
        }
    }

    /// Adds a new item to the DriveManager.
    pub fn add_item(&mut self, item: WarpDriveItem) {
        info!("Adding item to DriveManager: {:?}", item);
        let id = match &item {
            WarpDriveItem::Workflow { id, .. } => *id,
            WarpDriveItem::Notebook { id, .. } => *id,
            WarpDriveItem::Prompt { id, .. } => *id,
            WarpDriveItem::EnvironmentVariables { id, .. } => *id,
        };
        self.items.insert(id, item);
    }

    /// Retrieves an item by its UUID.
    pub fn get_item(&self, id: &Uuid) -> Option<&WarpDriveItem> {
        self.items.get(id)
    }

    /// Retrieves a mutable reference to an item by its UUID.
    pub fn get_item_mut(&mut self, id: &Uuid) -> Option<&mut WarpDriveItem> {
        self.items.get_mut(id)
    }

    /// Returns a vector of all items currently in the DriveManager.
    pub fn get_all_items(&self) -> Vec<WarpDriveItem> {
        self.items.values().cloned().collect()
    }

    /// Removes an item by its UUID.
    pub fn remove_item(&mut self, id: &Uuid) -> Option<WarpDriveItem> {
        info!("Removing item from DriveManager with ID: {}", id);
        self.items.remove(id)
    }

    /// Clears all items from the DriveManager.
    pub fn clear_items(&mut self) {
        info!("Clearing all items from DriveManager.");
        self.items.clear();
    }

    /// Returns the number of items in the DriveManager.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the DriveManager contains no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Future methods could include:
    // - `filter_by_type(item_type: ItemType) -> Vec<&WarpDriveItem>`
    // - `search_items(query: &str) -> Vec<&WarpDriveItem>`
    // - `save_to_disk(&self, path: &Path)`
    // - `load_from_disk(path: &Path) -> Result<Self>`
}

impl Default for DriveManager {
    fn default() -> Self {
        Self::new()
    }
}
