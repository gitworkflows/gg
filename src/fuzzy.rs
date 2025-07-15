use fuzzy_matcher::{FuzzyMatcher as FuzzyMatcherTrait, skim::SkimMatcherV2};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub score: i64,
    pub suggestion_type: SuggestionType,
}

#[derive(Debug, Clone)]
pub enum SuggestionType {
    Command,
    File,
    Directory,
    History,
    Alias,
}

pub struct FuzzyMatcher {
    matcher: SkimMatcherV2,
    command_cache: Vec<String>,
    file_cache: Vec<String>,
    history_cache: Vec<String>,
}

impl FuzzyMatcher {
    pub fn new() -> Self {
        FuzzyMatcher {
            matcher: SkimMatcherV2::default(),
            command_cache: Self::load_system_commands(),
            file_cache: Vec::new(),
            history_cache: Vec::new(),
        }
    }

    pub fn get_suggestions(&self, input: &str) -> Vec<Suggestion> {
        if input.is_empty() {
            return Vec::new();
        }

        let mut suggestions = Vec::new();

        // Match against commands
        for command in &self.command_cache {
            if let Some(score) = self.matcher.fuzzy_match(command, input) {
                suggestions.push(Suggestion {
                    text: command.clone(),
                    score,
                    suggestion_type: SuggestionType::Command,
                });
            }
        }

        // Match against history
        for history_item in &self.history_cache {
            if let Some(score) = self.matcher.fuzzy_match(history_item, input) {
                suggestions.push(Suggestion {
                    text: history_item.clone(),
                    score,
                    suggestion_type: SuggestionType::History,
                });
            }
        }

        // Sort by score (descending)
        suggestions.sort_by(|a, b| b.score.cmp(&a.score));
        suggestions.truncate(10); // Limit to top 10 suggestions

        suggestions
    }

    pub fn update_history(&mut self, command: String) {
        if !self.history_cache.contains(&command) {
            self.history_cache.push(command);
            
            // Keep only the last 100 history items
            if self.history_cache.len() > 100 {
                self.history_cache.remove(0);
            }
        }
    }

    fn load_system_commands() -> Vec<String> {
        // This would typically scan PATH and load available commands
        vec![
            "ls".to_string(),
            "cd".to_string(),
            "pwd".to_string(),
            "cat".to_string(),
            "grep".to_string(),
            "find".to_string(),
            "git".to_string(),
            "npm".to_string(),
            "cargo".to_string(),
            "docker".to_string(),
            "kubectl".to_string(),
            "ssh".to_string(),
            "scp".to_string(),
            "curl".to_string(),
            "wget".to_string(),
            "vim".to_string(),
            "nano".to_string(),
            "code".to_string(),
        ]
    }
}
