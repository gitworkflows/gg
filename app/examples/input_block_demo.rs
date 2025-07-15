use crate::input_block::InputBlock;
use log::info;

pub fn run_input_block_examples() {
    info!("--- Running Input Block Examples ---");

    // Default input block
    let default_block = InputBlock::new("Enter your command:".to_string(), "text".to_string());
    info!("Default Input Block: {:?}", default_block);

    // Password input block
    let password_block = InputBlock::new("Enter your password:".to_string(), "password".to_string());
    info!("Password Input Block: {:?}", password_block);

    // Number input block
    let number_block = InputBlock::new("Enter a number:".to_string(), "number".to_string());
    info!("Number Input Block: {:?}", number_block);

    // Input block with initial value
    let prefilled_block = InputBlock::with_value("Edit this text:".to_string(), "initial value".to_string(), "text".to_string());
    info!("Prefilled Input Block: {:?}", prefilled_block);

    // Simulate setting a value
    let mut mutable_block = InputBlock::new("Type something:".to_string(), "text".to_string());
    mutable_block.set_value("Hello, Warp!".to_string());
    info!("Mutable Input Block after setting value: {:?}", mutable_block);

    info!("--- Finished Input Block Examples ---");
}
