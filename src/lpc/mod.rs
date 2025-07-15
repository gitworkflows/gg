use log::info;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ListFiles,
    OpenFile(String),
    Search(String),
    Unknown,
}

pub struct LanguageProcessor {
    // Could hold NLP models, rule sets, etc.
}

impl LanguageProcessor {
    pub fn new() -> Self {
        LanguageProcessor {}
    }

    pub fn process_command(&self, command: &str) -> CommandType {
        info!("Processing command: '{}'", command);
        let lower_command = command.to_lowercase();

        if lower_command.contains("show me files") || lower_command.contains("list files") {
            CommandType::ListFiles
        } else if lower_command.starts_with("open file") {
            let parts: Vec<&str> = lower_command.splitn(3, ' ').collect();
            if parts.len() > 2 {
                CommandType::OpenFile(parts[2].to_string())
            } else {
                CommandType::Unknown
            }
        } else if lower_command.starts_with("search for") {
            let parts: Vec<&str> = lower_command.splitn(3, ' ').collect();
            if parts.len() > 2 {
                CommandType::Search(parts[2].to_string())
            } else {
                CommandType::Unknown
            }
        } else {
            CommandType::Unknown
        }
    }
}
