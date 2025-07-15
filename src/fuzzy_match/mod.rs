// This module would contain the core logic for fuzzy string matching,
// used for features like command suggestions, file search, or command palette.

pub mod matcher; // The actual fuzzy matching algorithm

// Re-export for easier access
pub use matcher::FuzzyMatcher;

// /** rest of code here **/
