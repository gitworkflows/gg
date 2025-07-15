use log::info;

pub struct McpViewCollection {
    items: Vec<String>,
    selected_index: Option<usize>,
}

impl McpViewCollection {
    pub fn new() -> Self {
        McpViewCollection {
            items: vec![
                "Workflow A".to_string(),
                "Notebook B".to_string(),
                "Environment C".to_string(),
            ],
            selected_index: None,
        }
    }

    pub fn display(&self) {
        info!("Displaying MCP Collection View. Items: {:?}", self.items);
        if let Some(index) = self.selected_index {
            info!("Selected item: {}", self.items[index]);
        }
        // Placeholder for actual UI rendering logic
    }

    pub fn select_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = Some(index);
            info!("MCP Collection: Selected item at index {}", index);
        } else {
            info!("MCP Collection: Invalid index {}", index);
        }
    }

    pub fn get_selected_item_name(&self) -> Option<&String> {
        self.selected_index.map(|idx| &self.items[idx])
    }
}
