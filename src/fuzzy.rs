use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct FuzzySearch {
    matcher: SkimMatcherV2,
}

impl FuzzySearch {
    pub fn new() -> Self {
        FuzzySearch {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Performs a fuzzy search on a list of candidates.
    /// Returns a vector of (score, candidate) tuples, sorted by score in descending order.
    pub fn fuzzy_match(&self, query: &str, candidates: &[String]) -> Vec<(i64, String)> {
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

    /// Checks if a query matches a candidate with a minimum score.
    pub fn is_match(&self, query: &str, candidate: &str, min_score: i64) -> bool {
        self.matcher.fuzzy_match(candidate, query).map_or(false, |score| score >= min_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_basic() {
        let fuzzy_search = FuzzySearch::new();
        let candidates = vec![
            "apple".to_string(),
            "banana".to_string(),
            "apricot".to_string(),
            "grape".to_string(),
        ];

        let results = fuzzy_search.fuzzy_match("app", &candidates);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].1, "apple");
        assert_eq!(results[1].1, "apricot");
    }

    #[test]
    fn test_fuzzy_match_no_match() {
        let fuzzy_search = FuzzySearch::new();
        let candidates = vec![
            "apple".to_string(),
            "banana".to_string(),
        ];
        let results = fuzzy_search.fuzzy_match("xyz", &candidates);
        assert!(results.is_empty());
    }

    #[test]
    fn test_is_match() {
        let fuzzy_search = FuzzySearch::new();
        assert!(fuzzy_search.is_match("abc", "axbyc", 0));
        assert!(!fuzzy_search.is_match("abc", "axbyc", 100)); // Assuming 100 is a high score
    }

    #[test]
    fn test_empty_query() {
        let fuzzy_search = FuzzySearch::new();
        let candidates = vec!["test".to_string()];
        let results = fuzzy_search.fuzzy_match("", &candidates);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "test");
    }
}
