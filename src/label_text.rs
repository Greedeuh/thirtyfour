use serde::Serialize;
use crate::options_common::TestingLibraryOptions;

/// Options for label text queries
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByLabelTextOptions {
    /// CSS selector to filter elements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Whether to use exact text matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl ByLabelTextOptions {
    /// Create a new empty ByLabelTextOptions
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the selector option
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    /// Set the exact option
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}

impl TestingLibraryOptions for ByLabelTextOptions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_text_options_empty_serialization() {
        let options = ByLabelTextOptions::new();
        let json = options.to_json_string().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_label_text_options_basic_serialization() {
        let options = ByLabelTextOptions::new().selector("input").exact(false);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["selector"], "input");
        assert_eq!(json_value["exact"], false);
    }

    #[test]
    fn test_label_text_options_partial_serialization() {
        let options = ByLabelTextOptions::new().exact(true);

        let json_value = options.to_json_value().unwrap();
        assert!(json_value["selector"].is_null());
        assert_eq!(json_value["exact"], true);
    }

    #[test]
    fn test_label_text_options_json_string() {
        let options = ByLabelTextOptions::new().selector("textarea").exact(true);

        let json_string = options.to_json_string().unwrap();
        assert!(json_string.contains("\"selector\":\"textarea\""));
        assert!(json_string.contains("\"exact\":true"));
    }
}
