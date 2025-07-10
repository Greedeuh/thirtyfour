use serde::Serialize;
use serde_json::Value;

/// Options for title queries
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByTitleOptions {
    /// Whether to use exact text matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl ByTitleOptions {
    /// Create a new empty ByTitleOptions
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the exact option
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }

    /// Serialize the options to a JSON string for use in Testing Library method calls
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize the options to a JSON Value for use in Testing Library method calls
    pub fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_options_empty_serialization() {
        let options = ByTitleOptions::new();
        let json = options.to_json_string().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_title_options_exact_true_serialization() {
        let options = ByTitleOptions::new().exact(true);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["exact"], true);
    }

    #[test]
    fn test_title_options_exact_false_serialization() {
        let options = ByTitleOptions::new().exact(false);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["exact"], false);
    }

    #[test]
    fn test_title_options_json_string() {
        let options = ByTitleOptions::new().exact(true);

        let json_string = options.to_json_string().unwrap();
        assert!(json_string.contains("\"exact\":true"));
    }
}
