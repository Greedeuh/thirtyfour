use serde::{Deserialize, Serialize};

/// Configuration options for the testing library
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    /// Set to true if window.getComputedStyle supports pseudo-elements i.e. a second argument.
    /// If you're using testing-library in a browser you almost always want to set this to true.
    /// Only very old browser don't support this property (such as IE 8 and earlier).
    /// However, jsdom does not support the second argument currently.
    /// This includes versions of jsdom prior to 16.4.0 and any version that logs a not implemented
    /// warning when calling getComputedStyle with a second argument e.g.
    /// window.getComputedStyle(document.createElement('div'), '::after').
    /// Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_style_supports_pseudo_elements: Option<bool>,

    /// The default value for the hidden option used by getByRole.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hidden: Option<bool>,

    /// The default value for the ignore option used by getByText.
    /// Also determines the nodes that are being ignored when errors are printed.
    /// Defaults to "script, style".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ignore: Option<String>,

    /// By default, waitFor will ensure that the stack trace for errors thrown by Testing Library
    /// is cleaned up and shortened so it's easier for you to identify the part of your code
    /// that resulted in the error (async stack traces are hard to debug).
    /// If you want to disable this, then set showOriginalStackTrace to false.
    /// You can also disable this for a specific call in the options you pass to waitFor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_original_stack_trace: Option<bool>,

    /// When enabled, if better queries are available, the test will fail and provide a
    /// suggested query to use instead.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throw_suggestions: Option<bool>,

    /// The attribute used by getByTestId and related queries.
    /// Defaults to "data-testid".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_id_attribute: Option<String>,

    /// The global timeout value in milliseconds used by waitFor utilities.
    /// Defaults to 1000ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_util_timeout: Option<u64>,
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

impl Options {
    /// Creates a new Options instance with all values set to None
    pub fn new() -> Self {
        Self {
            computed_style_supports_pseudo_elements: None,
            default_hidden: None,
            default_ignore: None,
            show_original_stack_trace: None,
            throw_suggestions: None,
            test_id_attribute: None,
            async_util_timeout: None,
        }
    }

    /// Builder method to set computed_style_supports_pseudo_elements
    pub fn with_computed_style_supports_pseudo_elements(mut self, value: bool) -> Self {
        self.computed_style_supports_pseudo_elements = Some(value);
        self
    }

    /// Builder method to set default_hidden
    pub fn with_default_hidden(mut self, value: bool) -> Self {
        self.default_hidden = Some(value);
        self
    }

    /// Builder method to set default_ignore
    pub fn with_default_ignore(mut self, value: impl Into<String>) -> Self {
        self.default_ignore = Some(value.into());
        self
    }

    /// Builder method to set show_original_stack_trace
    pub fn with_show_original_stack_trace(mut self, value: bool) -> Self {
        self.show_original_stack_trace = Some(value);
        self
    }

    /// Builder method to set throw_suggestions
    pub fn with_throw_suggestions(mut self, value: bool) -> Self {
        self.throw_suggestions = Some(value);
        self
    }

    /// Builder method to set test_id_attribute
    pub fn with_test_id_attribute(mut self, value: impl Into<String>) -> Self {
        self.test_id_attribute = Some(value.into());
        self
    }

    /// Builder method to set async_util_timeout
    pub fn with_async_util_timeout(mut self, value: u64) -> Self {
        self.async_util_timeout = Some(value);
        self
    }

    /// Serialize the options to JSON string
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
