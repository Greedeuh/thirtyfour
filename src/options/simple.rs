use serde::Serialize;
use crate::options::common::TestingLibraryOptions;

/// Simple options struct for testing-library queries that only need exact matching.
/// 
/// This struct consolidates the common pattern used by text, alt_text, display_value,
/// placeholder_text, test_id, and title options.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleOptions {
    /// Whether to use exact text matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl SimpleOptions {
    /// Create a new instance with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the exact option
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}

impl TestingLibraryOptions for SimpleOptions {}

// Type aliases for clarity and API compatibility
pub type ByTextOptions = SimpleOptions;
pub type ByAltTextOptions = SimpleOptions;
pub type ByDisplayValueOptions = SimpleOptions;
pub type ByPlaceholderTextOptions = SimpleOptions;
pub type ByTestIdOptions = SimpleOptions;
pub type ByTitleOptions = SimpleOptions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_options_empty_serialization() {
        let options = SimpleOptions::new();
        let json = options.to_json_string().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_simple_options_exact_true_serialization() {
        let options = SimpleOptions::new().exact(true);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["exact"], true);
    }

    #[test]
    fn test_simple_options_exact_false_serialization() {
        let options = SimpleOptions::new().exact(false);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["exact"], false);
    }

    #[test]
    fn test_simple_options_json_string() {
        let options = SimpleOptions::new().exact(true);

        let json_string = options.to_json_string().unwrap();
        assert!(json_string.contains("\"exact\":true"));
    }

    // Test type aliases work correctly
    #[test]
    fn test_type_aliases() {
        let text_options = ByTextOptions::new().exact(true);
        let alt_text_options = ByAltTextOptions::new().exact(false);
        let display_value_options = ByDisplayValueOptions::new().exact(true);
        let placeholder_text_options = ByPlaceholderTextOptions::new().exact(false);
        let test_id_options = ByTestIdOptions::new().exact(true);
        let title_options = ByTitleOptions::new().exact(false);

        // All should serialize correctly
        assert!(text_options.to_json_string().unwrap().contains("\"exact\":true"));
        assert!(alt_text_options.to_json_string().unwrap().contains("\"exact\":false"));
        assert!(display_value_options.to_json_string().unwrap().contains("\"exact\":true"));
        assert!(placeholder_text_options.to_json_string().unwrap().contains("\"exact\":false"));
        assert!(test_id_options.to_json_string().unwrap().contains("\"exact\":true"));
        assert!(title_options.to_json_string().unwrap().contains("\"exact\":false"));
    }

    #[test]
    fn test_trait_implementation() {
        let options = ByTextOptions::new().exact(true);
        
        // Test that trait methods work
        assert!(options.to_json_string().is_ok());
        assert!(options.to_json_value().is_ok());
    }
}