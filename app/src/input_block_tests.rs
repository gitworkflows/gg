#[cfg(test)]
mod tests {
    use crate::input_block::InputBlock;

    #[test]
    fn test_input_block_creation() {
        let block = InputBlock::new("Enter username:".to_string(), "text".to_string());
        assert_eq!(block.prompt, "Enter username:");
        assert_eq!(block.value, "");
        assert_eq!(block.input_type, "text");
    }

    #[test]
    fn test_input_block_with_initial_value() {
        let block = InputBlock::with_value("Enter email:".to_string(), "test@example.com".to_string(), "text".to_string());
        assert_eq!(block.prompt, "Enter email:");
        assert_eq!(block.value, "test@example.com");
        assert_eq!(block.input_type, "text");
    }

    #[test]
    fn test_set_value() {
        let mut block = InputBlock::new("Enter password:".to_string(), "password".to_string());
        block.set_value("mysecret".to_string());
        assert_eq!(block.value, "mysecret");
    }

    #[test]
    fn test_get_value() {
        let block = InputBlock::with_value("Search:".to_string(), "query".to_string(), "text".to_string());
        assert_eq!(block.get_value(), "query");
    }

    #[test]
    fn test_clear_value() {
        let mut block = InputBlock::with_value("Notes:".to_string(), "Some notes here.".to_string(), "text".to_string());
        block.clear();
        assert_eq!(block.value, "");
    }

    #[test]
    fn test_input_type_variations() {
        let text_block = InputBlock::new("Text:".to_string(), "text".to_string());
        assert_eq!(text_block.input_type, "text");

        let password_block = InputBlock::new("Password:".to_string(), "password".to_string());
        assert_eq!(password_block.input_type, "password");

        let number_block = InputBlock::new("Number:".to_string(), "number".to_string());
        assert_eq!(number_block.input_type, "number");
    }
}
