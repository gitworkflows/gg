use crate::input_block::InputBlock;
use log::info;

pub fn run_input_block_examples() {
    info!("--- Running Input Block Examples ---");

    // Example 1: Basic text input
    let mut text_input = InputBlock::new("Enter your name:".to_string(), "text".to_string());
    text_input.set_value("Alice".to_string());
    info!("Text Input Block: Prompt='{}', Value='{}'", text_input.prompt, text_input.value);

    // Example 2: Password input
    let mut password_input = InputBlock::new("Enter your password:".to_string(), "password".to_string());
    password_input.set_value("secret123".to_string());
    info!("Password Input Block: Prompt='{}', Value='{}'", password_input.prompt, password_input.value);

    // Example 3: Number input
    let mut number_input = InputBlock::new("Enter your age:".to_string(), "number".to_string());
    number_input.set_value("30".to_string());
    info!("Number Input Block: Prompt='{}', Value='{}'", number_input.prompt, number_input.value);

    // Example 4: Input with initial value
    let mut prefilled_input = InputBlock::with_value(
        "Edit this pre-filled text:".to_string(),
        "Initial content".to_string(),
        "text".to_string(),
    );
    info!("Prefilled Input Block (initial): Prompt='{}', Value='{}'", prefilled_input.prompt, prefilled_input.value);
    prefilled_input.set_value("Updated content".to_string());
    info!("Prefilled Input Block (updated): Prompt='{}', Value='{}'", prefilled_input.prompt, prefilled_input.value);

    // Example 5: Clearing an input
    let mut clearable_input = InputBlock::new("Type something to clear:".to_string(), "text".to_string());
    clearable_input.set_value("This will be cleared".to_string());
    info!("Clearable Input Block (before clear): Value='{}'", clearable_input.value);
    clearable_input.clear();
    info!("Clearable Input Block (after clear): Value='{}'", clearable_input.value);

    info!("--- Finished Input Block Examples ---");
}
