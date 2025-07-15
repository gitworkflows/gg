use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher as TraitFuzzyMatcher; // Alias to avoid conflict with struct name

/// A wrapper around `skim`'s fuzzy matcher for consistent usage.
pub struct FuzzyMatcher {
    matcher: SkimMatcherV2,
}

impl FuzzyMatcher {
    /// Creates a new `FuzzyMatcher` with default settings.
    pub fn new() -> Self {
        FuzzyMatcher {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Performs a fuzzy match between a `query` and a `text`.
    /// Returns `Some(score)` if a match is found, otherwise `None`.
    pub fn fuzzy_match(&self, text: &str, query: &str) -> Option<i64> {
        self.matcher.fuzzy_match(text, query)
    }

    /// Performs a fuzzy match and returns the matched indices if a match is found.
    pub fn fuzzy_match_indices(&self, text: &str, query: &str) -> Option<(i64, Vec<usize>)> {
        self.matcher.fuzzy_match_indices(text, query)
    }

    /// Filters a list of candidates based on a fuzzy query.
    /// Returns a vector of `(score, candidate)` tuples, sorted by score in descending order.
    pub fn filter_candidates(&self, query: &str, candidates: &[String]) -> Vec<(i64, String)> {
        let mut results: Vec<(i64, String)> = candidates
            .iter()
            .filter_map(|candidate| {
                self.matcher.fuzzy_match(candidate, query)
                    .map(|score| (score, candidate.clone()))
            })
            .collect();

        results.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by score, descending
        results
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_basic() {
        let matcher = FuzzyMatcher::new();
        assert!(matcher.fuzzy_match("hello world", "hlo").is_some());
        assert!(matcher.fuzzy_match("rust programming", "rust prog").is_some());
        assert!(matcher.fuzzy_match("apple", "aple").is_some());
        assert!(matcher.fuzzy_match("apple", "xyz").is_none());
    }

    #[test]
    fn test_fuzzy_match_indices() {
        let matcher = FuzzyMatcher::new();
        if let Some((score, indices)) = matcher.fuzzy_match_indices("hello world", "hlo") {
            assert!(score > 0);
            assert_eq!(indices, vec![0, 2, 4]); // h, l, o
        } else {
            panic!("Match not found");
        }
    }

    #[test]
    fn test_filter_candidates() {
        let matcher = FuzzyMatcher::new();
        let candidates = vec![
            "foo_bar_baz".to_string(),
            "foobar".to_string(),
            "fobaz".to_string(),
            "qux".to_string(),
        ];
        let results = matcher.filter_candidates("fb", &candidates);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, "foobar"); // Order might vary slightly based on score, but these two should be present
        assert_eq!(results[1].1, "fobaz");
    }

    #[test]
    fn test_empty_query() {
        let matcher = FuzzyMatcher::new();
        let candidates = vec!["test".to_string()];
        let results = matcher.filter_candidates("", &candidates);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "test");
    }
}
