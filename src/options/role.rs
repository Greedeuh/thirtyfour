use crate::options::common::{TestingLibraryOptions, TextMatch};
use serde::{Serialize, Serializer};

#[cfg(test)]
use crate::options::common::process_raw_javascript_markers;

#[cfg(test)]
use serde_json::Value;

/// Options for value-based queries on range widgets
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueOptions {
    /// Minimum value (aria-valuemin)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// Maximum value (aria-valuemax)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// Current value (aria-valuenow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub now: Option<i32>,
    /// Text representation of value (aria-valuetext)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMatch>,
}

/// Comprehensive options for role-based queries
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ByRoleOptions {
    /// Include elements normally excluded from accessibility tree
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Filter by accessible name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TextMatch>,
    /// Filter by accessible description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<TextMatch>,
    /// Filter by selected state (aria-selected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// Filter by busy state (aria-busy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    /// Filter by checked state (aria-checked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    /// Filter by pressed state (aria-pressed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
    /// Enable/disable query suggestions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggest: Option<bool>,
    /// Filter by current state (aria-current)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<CurrentState>,
    /// Filter by expanded state (aria-expanded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    /// Enable querying fallback roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_fallbacks: Option<bool>,
    /// Filter by heading level (only for heading role)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    /// Filter by value properties (only for range widgets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueOptions>,
}

/// Represents the current state for aria-current attribute
#[derive(Debug, Clone)]
pub enum CurrentState {
    /// aria-current="false" or no aria-current attribute
    False,
    /// aria-current="true"
    True,
    /// aria-current="page"
    Page,
    /// aria-current="step"
    Step,
    /// aria-current="location"
    Location,
    /// aria-current="date"
    Date,
    /// aria-current="time"
    Time,
    /// Custom aria-current value
    Custom(String),
}

impl Serialize for CurrentState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CurrentState::False => false.serialize(serializer),
            CurrentState::True => true.serialize(serializer),
            CurrentState::Page => "page".serialize(serializer),
            CurrentState::Step => "step".serialize(serializer),
            CurrentState::Location => "location".serialize(serializer),
            CurrentState::Date => "date".serialize(serializer),
            CurrentState::Time => "time".serialize(serializer),
            CurrentState::Custom(s) => s.serialize(serializer),
        }
    }
}


impl ByRoleOptions {
    /// Create a new empty ByRoleOptions
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the hidden option
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = Some(hidden);
        self
    }

    /// Set the name option
    pub fn name(mut self, name: TextMatch) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the description option
    pub fn description(mut self, description: TextMatch) -> Self {
        self.description = Some(description);
        self
    }

    /// Set the selected option
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Set the busy option
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = Some(busy);
        self
    }

    /// Set the checked option
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Set the pressed option
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// Set the suggest option
    pub fn suggest(mut self, suggest: bool) -> Self {
        self.suggest = Some(suggest);
        self
    }

    /// Set the current option
    pub fn current(mut self, current: CurrentState) -> Self {
        self.current = Some(current);
        self
    }

    /// Set the expanded option
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = Some(expanded);
        self
    }

    /// Set the query_fallbacks option
    pub fn query_fallbacks(mut self, query_fallbacks: bool) -> Self {
        self.query_fallbacks = Some(query_fallbacks);
        self
    }

    /// Set the level option (only for heading role)
    pub fn level(mut self, level: u8) -> Self {
        self.level = Some(level);
        self
    }

    /// Set the value option (only for range widgets)
    pub fn value(mut self, value: ValueOptions) -> Self {
        self.value = Some(value);
        self
    }

}

impl TestingLibraryOptions for ByRoleOptions {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_options_serialization() {
        let options = ByRoleOptions::new();
        let json = options.to_json_string().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_basic_options_serialization() {
        let options = ByRoleOptions::new().hidden(true).selected(false);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["hidden"], true);
        assert_eq!(json_value["selected"], false);
        assert!(json_value["name"].is_null());
    }

    #[test]
    fn test_text_match_exact_serialization() {
        let options = ByRoleOptions::new().name(TextMatch::Exact("Submit".to_string()));

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["name"], "Submit");
    }

    #[test]
    fn test_text_match_regex_serialization() {
        let options = ByRoleOptions::new().name(TextMatch::Regex("/^submit.*/".to_string()));

        // Test the string serialization (which processes markers)
        let json_string = options.to_json_string().unwrap();
        assert!(json_string.contains("/^submit.*/"));

        // Note: to_json_value returns the raw marker, which is expected
        // since it's used internally before marker processing
        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["name"], "__RAW_JS__/^submit.*/");
    }

    #[test]
    fn test_current_state_serialization() {
        let options_false = ByRoleOptions::new().current(CurrentState::False);
        let json_false = options_false.to_json_value().unwrap();
        assert_eq!(json_false["current"], false);

        let options_page = ByRoleOptions::new().current(CurrentState::Page);
        let json_page = options_page.to_json_value().unwrap();
        assert_eq!(json_page["current"], "page");

        let options_custom =
            ByRoleOptions::new().current(CurrentState::Custom("custom-value".to_string()));
        let json_custom = options_custom.to_json_value().unwrap();
        assert_eq!(json_custom["current"], "custom-value");
    }

    #[test]
    fn test_value_options_serialization() {
        let value_opts = ValueOptions {
            min: Some(0),
            max: Some(100),
            now: Some(50),
            text: Some(TextMatch::Exact("medium".to_string())),
        };

        let options = ByRoleOptions::new().value(value_opts);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["value"]["min"], 0);
        assert_eq!(json_value["value"]["max"], 100);
        assert_eq!(json_value["value"]["now"], 50);
        assert_eq!(json_value["value"]["text"], "medium");
    }

    #[test]
    fn test_query_fallbacks_rename() {
        let options = ByRoleOptions::new().query_fallbacks(true);

        let json_value = options.to_json_value().unwrap();
        assert_eq!(json_value["queryFallbacks"], true);
        assert!(json_value["query_fallbacks"].is_null());
    }

    #[test]
    fn test_complex_options_serialization() {
        let options = ByRoleOptions::new()
            .name(TextMatch::Substring("button".to_string()))
            .hidden(false)
            .pressed(true)
            .level(2)
            .current(CurrentState::Page);

        let json_string = options.to_json_string().unwrap();

        // Parse back to verify structure
        let parsed: Value = serde_json::from_str(&json_string).unwrap();
        assert_eq!(parsed["name"], "button");
        assert_eq!(parsed["hidden"], false);
        assert_eq!(parsed["pressed"], true);
        assert_eq!(parsed["level"], 2);
        assert_eq!(parsed["current"], "page");
    }

    #[test]
    fn test_serialization_example() {
        // Example usage: Creating complex options for a button query
        let options = ByRoleOptions::new()
            .name(TextMatch::Regex("/submit|send/".to_string()))
            .pressed(false)
            .hidden(false)
            .suggest(true);

        let json_string = options.to_json_string().unwrap();
        println!("Serialized options: {json_string}");

        // This would be used in JavaScript like:
        // getByRole('button', {name: /submit|send/, pressed: false, hidden: false, suggest: true})

        // Note: The processed JSON is not valid JSON because regex is unquoted
        // This is intentional for JavaScript consumption
        assert!(json_string.contains("/submit|send/"));
        assert!(json_string.contains("\"pressed\":false"));
        assert!(json_string.contains("\"hidden\":false"));
        assert!(json_string.contains("\"suggest\":true"));
    }

    #[test]
    fn test_regex_validation() {
        // Valid regex patterns
        let valid_regex = TextMatch::Regex("/test.*/".to_string());
        assert!(valid_regex.validate_regex().is_ok());

        let valid_regex_with_flags = TextMatch::Regex("/test.*/i".to_string());
        assert!(valid_regex_with_flags.validate_regex().is_ok());

        // Invalid regex patterns
        let invalid_no_slashes = TextMatch::Regex("test.*".to_string());
        assert!(invalid_no_slashes.validate_regex().is_err());

        let invalid_pattern = TextMatch::Regex("/[/".to_string());
        assert!(invalid_pattern.validate_regex().is_err());

        // Non-regex variants should always be valid
        let exact_match = TextMatch::Exact("test".to_string());
        assert!(exact_match.validate_regex().is_ok());
    }

    #[test]
    fn test_raw_javascript_marker_processing() {
        // Test the marker processing function
        let json_with_markers = r#"{"name":"__RAW_JS__/Save.*/","pressed":false}"#;
        let processed = process_raw_javascript_markers(json_with_markers);
        assert_eq!(processed, r#"{"name":/Save.*/,"pressed":false}"#);

        // Test with flags
        let json_with_flags = r#"{"name":"__RAW_JS__/save/i","hidden":true}"#;
        let processed_flags = process_raw_javascript_markers(json_with_flags);
        assert_eq!(processed_flags, r#"{"name":/save/i,"hidden":true}"#);

        // Test with multiple markers
        let json_multiple = r#"{"name":"__RAW_JS__/button/","description":"__RAW_JS__/click.*/i"}"#;
        let processed_multiple = process_raw_javascript_markers(json_multiple);
        assert_eq!(
            processed_multiple,
            r#"{"name":/button/,"description":/click.*/i}"#
        );

        // Test with no markers (should remain unchanged)
        let json_no_markers = r#"{"name":"button","pressed":true}"#;
        let processed_no_markers = process_raw_javascript_markers(json_no_markers);
        assert_eq!(processed_no_markers, r#"{"name":"button","pressed":true}"#);
    }
}
