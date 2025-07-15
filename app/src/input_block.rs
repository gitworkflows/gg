use log::info;

#[derive(Debug, Clone)]
pub struct InputBlock {
    pub prompt: String,
    pub value: String,
    pub input_type: String, // e.g., "text", "password", "number"
}

impl InputBlock {
    pub fn new(prompt: String, input_type: String) -> Self {
        InputBlock {
            prompt,
            value: String::new(),
            input_type,
        }
    }

    pub fn with_value(prompt: String, initial_value: String, input_type: String) -> Self {
        InputBlock {
            prompt,
            value: initial_value,
            input_type,
        }
    }

    pub fn set_value(&mut self, new_value: String) {
        self.value = new_value;
        info!("InputBlock value updated to: {}", self.value);
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn clear(&mut self) {
        self.value.clear();
        info!("InputBlock value cleared.");
    }

    pub fn render(&self) {
        info!("Rendering InputBlock: Prompt='{}', Value='{}', Type='{}'", self.prompt, self.value, self.input_type);
        // In a real UI, this would involve drawing the input field
    }
}
