use serde::{Serialize, Serializer};
use serde_json::Value;
use regex;

/// Common trait for all testing-library option types.
///
/// This trait provides standard methods for serialization and construction
/// that are shared across all option types.
pub trait TestingLibraryOptions: Serialize + Default {
    /// Create a new instance with default values.
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    /// Serialize this options struct to a JSON string.
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(self)?;
        // Post-process to convert marked raw JavaScript values
        Ok(process_raw_javascript_markers(&json))
    }

    /// Serialize this options struct to a JSON value.
    fn to_json_value(&self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

/// A wrapper type that indicates a value should be serialized as raw JavaScript
#[derive(Debug, Clone)]
pub struct RawJavaScript(pub String);

impl Serialize for RawJavaScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Mark this as a special regex value that needs post-processing
        // We'll use a special marker that we can detect and replace later
        let marked_value = format!("__RAW_JS__{}", self.0);
        marked_value.serialize(serializer)
    }
}

/// Represents text matching options for Testing Library queries
/// Supports both string and regex patterns like the JavaScript Testing Library
#[derive(Debug, Clone)]
pub enum TextMatch {
    /// Exact string match
    Exact(String),
    /// Substring match
    Substring(String),
    /// Regular expression match
    Regex(String),
}

impl Serialize for TextMatch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextMatch::Exact(s) => s.serialize(serializer),
            TextMatch::Substring(s) => s.serialize(serializer),
            TextMatch::Regex(pattern) => {
                // Use RawJavaScript wrapper to indicate this should be raw JS
                RawJavaScript(pattern.clone()).serialize(serializer)
            }
        }
    }
}

impl TextMatch {
    /// Validate that the regex pattern is properly formatted
    pub fn validate_regex(&self) -> Result<(), String> {
        match self {
            TextMatch::Regex(pattern) => {
                // Check if it looks like a regex literal
                if !pattern.starts_with('/') {
                    return Err(
                        "Regex pattern must start with '/' (e.g., '/pattern/' or '/pattern/i')"
                            .to_string(),
                    );
                }

                // Find the last '/' to separate pattern from flags
                let last_slash = pattern.rfind('/');
                if last_slash.is_none() || last_slash.unwrap() == 0 {
                    return Err("Regex pattern must contain at least one '/' after the pattern (e.g., '/pattern/')".to_string());
                }

                let last_slash_pos = last_slash.unwrap();
                let inner_pattern = &pattern[1..last_slash_pos];

                // Validate the regex pattern (ignore flags for now)
                regex::Regex::new(inner_pattern)
                    .map_err(|e| format!("Invalid regex pattern: {e}"))?;

                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Get the text value for string matches or pattern for regex matches
    pub fn text_value(&self) -> &str {
        match self {
            TextMatch::Exact(text) => text,
            TextMatch::Substring(text) => text,
            TextMatch::Regex(pattern) => pattern,
        }
    }

    /// Check if this is an exact match
    pub fn is_exact(&self) -> bool {
        matches!(self, TextMatch::Exact(_))
    }

    /// Check if this is a regex match
    pub fn is_regex(&self) -> bool {
        matches!(self, TextMatch::Regex(_))
    }
}

impl From<&str> for TextMatch {
    fn from(text: &str) -> Self {
        if text.starts_with('/') && text.len() > 2 {
            if let Some(last_slash) = text.rfind('/') {
                if last_slash > 0 {
                    return Self::Regex(text.to_string());
                }
            }
        }
        Self::Exact(text.to_string())
    }
}

impl From<String> for TextMatch {
    fn from(text: String) -> Self {
        TextMatch::from(text.as_str())
    }
}

/// Convert marked raw JavaScript values to actual raw JavaScript
pub fn process_raw_javascript_markers(json: &str) -> String {
    // Replace "__RAW_JS__/pattern/" with /pattern/ (remove quotes and marker)
    use regex::Regex;
    let re = Regex::new(r#""__RAW_JS__([^"]+)""#).unwrap();
    re.replace_all(json, "$1").to_string()
}
