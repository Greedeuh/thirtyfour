//! # Thirtyfour Testing Library Extension
//!
//! This crate provides Testing Library integration for the Thirtyfour WebDriver library.
//! It allows you to query DOM elements using semantic selectors similar to React Testing Library,
//! making your tests more maintainable and user-focused.
//!
//! ## Getting Started
//!
//! ```no_run
//! use thirtyfour::prelude::*;
//! use thirtyfour_testing_library_ext::{Screen, By};
//!
//! #[tokio::main]
//! async fn main() -> WebDriverResult<()> {
//!     let caps = DesiredCapabilities::chrome();
//!     let driver = WebDriver::new("http://localhost:9515", caps).await?;
//!     
//!     driver.goto("https://example.com").await?;
//!     
//!     // Create a screen instance
//!     let screen = Screen::build_with_testing_library(driver.clone()).await?;
//!     
//!     // Query by role (semantic selector)
//!     let button = screen.get(By::role("button")).await?;
//!     button.click().await?;
//!     
//!     // Query by text content
//!     let heading = screen.get(By::text("Welcome")).await?;
//!     println!("Found heading: {}", heading.text().await?);
//!     
//!     driver.quit().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Query Methods
//!
//! The Screen struct provides several query methods with different behaviors:
//!
//! - `get()` / `get_all()` - Throw errors if elements aren't found
//! - `query()` / `query_all()` - Return `None` / empty Vec for missing elements  
//! - `find()` / `find_all()` - Wait for elements to appear with retries
//!
//! ## Selector Types
//!
//! - `By::role()` - Query by ARIA role
//! - `By::text()` - Query by text content
//! - `By::label_text()` - Query by label text
//! - `By::placeholder_text()` - Query by placeholder text
//! - `By::alt_text()` - Query by alt text
//! - `By::title()` - Query by title attribute
//! - `By::test_id()` - Query by test ID
//! - `By::display_value()` - Query by display value
//!
//! Each selector type supports options for advanced filtering and matching.

/// Configuration options for the testing library
pub mod configure;
/// Testing library options module
pub mod options;

// Re-export all options for convenience
pub use options::*;
use serde_json::Value;

use thirtyfour::{
    error::{WebDriverError, WebDriverResult},
    prelude::ScriptRet,
    WebDriver, WebElement,
};

// TODO
// - better error handling

/// A struct representing a screen in the testing library that provides DOM queries with different behaviors: get* methods throw errors if elements aren't found, query* methods return null for missing elements, and find* methods return promises that retry until elements are found.
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver,
    within_element: Option<WebElement>,
    configure_options: Option<configure::Options>,
}

impl Screen {
    /// Creates a new `Screen` and loads the testing library script in the browser
    pub async fn build_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        Self::load_testing_library(&driver).await?;

        Ok(Screen {
            driver,
            within_element: None,
            configure_options: None,
        })
    }

    /// Creates a new `Screen` and but does not load the testing library script
    /// This is useful if you want to load the script later or if you have already loaded in your frontend application by setting up the query function you need:
    /// ```javascript
    /// import {
    ///   queryAllByRole,
    ///   ...
    /// } from "@testing-library/dom"
    ///
    /// window.__TL__ = {
    ///   queryAllByRole,
    ///   ...
    /// }
    /// ```
    pub fn build(driver: WebDriver) -> WebDriverResult<Self> {
        Ok(Screen {
            driver,
            within_element: None,
            configure_options: None,
        })
    }

    /// Creates a new `Screen` wich will be scoped to a specific element
    pub fn within(&self, element: WebElement) -> Screen {
        Screen {
            driver: self.driver.clone(),
            within_element: Some(element),
            configure_options: self.configure_options.clone(),
        }
    }

    /// Configure the testing library options
    pub fn configure(mut self, options: configure::Options) -> Self {
        self.configure_options = Some(options);
        self
    }

    /// Unified get method that accepts a Selector enum and returns a single WebElement
    /// Throws an error if no elements match or if more than one match is found
    pub async fn get(&self, selector: impl Into<By>) -> WebDriverResult<WebElement> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        self.query_executor()
            .execute_query(
                "getBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                false,
            )
            .await?
            .element()
    }

    /// Unified get_all method that accepts a Selector enum and returns all matching WebElements
    /// Throws an error if no elements match
    pub async fn get_all(&self, selector: impl Into<By>) -> WebDriverResult<Vec<WebElement>> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        self.query_executor()
            .execute_query(
                "getAllBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                false,
            )
            .await?
            .elements()
    }

    /// Unified query method that accepts a Selector enum and returns a single WebElement
    /// Returns None if no elements match
    pub async fn query(&self, selector: impl Into<By>) -> WebDriverResult<Option<WebElement>> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        let mut elements = self
            .query_executor()
            .execute_query(
                "queryBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                true,
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Unified query_all method that accepts a Selector enum and returns all matching WebElements
    /// Returns empty Vec if no elements match
    pub async fn query_all(&self, selector: impl Into<By>) -> WebDriverResult<Vec<WebElement>> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        self.query_executor()
            .execute_query(
                "queryAllBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                false,
            )
            .await?
            .elements()
    }

    /// Unified find method that accepts a Selector enum and returns a single WebElement
    /// Waits for the element to appear and throws an error if not found
    pub async fn find(&self, selector: impl Into<By>) -> WebDriverResult<WebElement> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        self.query_executor()
            .execute_query(
                "findBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                false,
            )
            .await?
            .element()
    }

    /// Unified find_all method that accepts a Selector enum and returns all matching WebElements
    /// Waits for elements to appear and throws an error if none are found
    pub async fn find_all(&self, selector: impl Into<By>) -> WebDriverResult<Vec<WebElement>> {
        let selector = selector.into();
        let options_json = selector.options_json()?;
        self.query_executor()
            .execute_query(
                "findAllBy",
                selector.function_suffix(),
                selector.value(),
                options_json,
                false,
            )
            .await?
            .elements()
    }

    /// Logs and returns a URL that can be opened in a browser for debugging using testing-playground
    /// If element is None, logs the entire document. If element is provided, logs only that element.
    pub async fn log_testing_playground_url(
        &self,
        element: Option<WebElement>,
    ) -> WebDriverResult<String> {
        let (script, arguments) = match element {
            Some(element) => (
                "return window.__TL__.logTestingPlaygroundURL(arguments[0]);",
                vec![element.to_json()?],
            ),
            None => ("return window.__TL__.logTestingPlaygroundURL();", vec![]),
        };

        let result = self.query_executor().execute(script, arguments).await?;

        result
            .json()
            .as_str()
            .ok_or_else(|| {
                WebDriverError::Json(
                    "logTestingPlaygroundURL returned non-string value".to_string(),
                )
            })
            .map(|s| s.to_string())
    }

    async fn load_testing_library(driver: &WebDriver) -> WebDriverResult<()> {
        // Load the testing library script in the browser
        let testing_library = include_str!("../js/testing-library.js");
        driver.execute(testing_library, vec![]).await?;

        Ok(())
    }

    /// Get a query executor configured with current options
    fn query_executor(&self) -> QueryExecutor {
        QueryExecutor::new(
            self.driver.clone(),
            self.within_element.clone(),
            self.configure_options.clone(),
        )
    }
}

/// Helper struct for executing Testing Library queries
#[derive(Debug, Clone)]
struct QueryExecutor {
    driver: WebDriver,
    within_element: Option<WebElement>,
    configure_options: Option<configure::Options>,
}

impl QueryExecutor {
    fn new(
        driver: WebDriver,
        within_element: Option<WebElement>,
        configure_options: Option<configure::Options>,
    ) -> Self {
        Self {
            driver,
            within_element,
            configure_options,
        }
    }

    /// Execute a basic Testing Library script with retry logic
    pub async fn execute(&self, script: &str, arguments: Vec<Value>) -> WebDriverResult<ScriptRet> {
        let wrapped_script = self.wrap_load_retry(script);
        self.execute_and_retry_if_library_not_found(&wrapped_script, arguments)
            .await
    }

    /// Execute a Testing Library query
    pub async fn execute_query(
        &self,
        method_prefix: &str,
        function_suffix: &str,
        value: &str,
        options_json: Option<String>,
        with_null_filter: bool,
    ) -> WebDriverResult<ScriptRet> {
        let method_name = format!("{method_prefix}{function_suffix}");
        let (container, arguments) = self.container_and_arguments()?;

        let script = self.build_and_wrap(
            &method_name,
            container,
            value,
            options_json.as_deref(),
            with_null_filter,
        );

        self.execute_and_retry_if_library_not_found(&script, arguments)
            .await
    }

    /// Build and wrap a Testing Library script in one call
    fn build_and_wrap(
        &self,
        method_name: &str,
        container: &str,
        value: &str,
        options_json: Option<&str>,
        with_null_filter: bool,
    ) -> String {
        let script = self.query_script(
            method_name,
            container,
            value,
            options_json,
            with_null_filter,
        );
        self.wrap_load_retry(&self.wrap_configure(&script))
    }

    fn query_script(
        &self,
        method_name: &str,
        container: &str,
        value: &str,
        options_json: Option<&str>,
        with_null_filter: bool,
    ) -> String {
        // Format the value - detect regex patterns and handle appropriately
        let formatted_value = Self::format_query_value(value);

        let base_call = match options_json {
            Some(options) => {
                format!("window.__TL__.{method_name}({container}, {formatted_value}, {options})")
            }
            None => {
                format!("window.__TL__.{method_name}({container}, {formatted_value})")
            }
        };

        let script = if with_null_filter {
            // Transform null values to empty arrays easier to parse in Rust
            format!("return [{base_call}].filter(n => n);")
        } else {
            format!("return {base_call};")
        };

        // Process any regex markers in the final script
        process_raw_javascript_markers(&script)
    }

    /// Format a query value, detecting regex patterns and handling them appropriately
    fn format_query_value(value: &str) -> String {
        if Self::is_regex_pattern(value) {
            // It's a regex pattern - use the marker system for post-processing
            format!("\"__RAW_JS__{}\"", value)
        } else {
            // Regular string - quote it
            format!("'{}'", value)
        }
    }

    /// Detect if a value is a regex pattern (starts and ends with '/')
    fn is_regex_pattern(value: &str) -> bool {
        if value.starts_with('/') && value.len() > 2 {
            if let Some(last_slash) = value.rfind('/') {
                return last_slash > 0; // Ensure it's not just the opening slash
            }
        }
        false
    }

    const LIBRARY_NOT_FOUND_ERROR: &str = "Testing Library not found";

    fn wrap_load_retry(&self, script: &str) -> String {
        let configured_script = if let Some(ref options) = self.configure_options {
            if let Ok(options_json) = options.to_json_string() {
                format!("window.__TL__.configure({options_json}); {script}")
            } else {
                script.to_string()
            }
        } else {
            script.to_string()
        };

        format!(
            "if (!window.__TL__) return '{}'; {}",
            Self::LIBRARY_NOT_FOUND_ERROR,
            configured_script
        )
    }

    fn wrap_configure(&self, script: &str) -> String {
        let configured_script = if let Some(ref options) = self.configure_options {
            if let Ok(options_json) = options.to_json_string() {
                format!("window.__TL__.configure({options_json}); {script}")
            } else {
                script.to_string()
            }
        } else {
            script.to_string()
        };

        format!(
            "if (!window.__TL__) return '{}'; {}",
            Self::LIBRARY_NOT_FOUND_ERROR,
            configured_script
        )
    }

    /// Get container and arguments for script execution
    fn container_and_arguments(&self) -> WebDriverResult<(&str, Vec<Value>)> {
        if let Some(within_element) = &self.within_element {
            Ok(("arguments[0]", vec![within_element.to_json()?]))
        } else {
            Ok(("document", vec![]))
        }
    }

    /// Execute script with retry logic for missing Testing Library
    async fn execute_and_retry_if_library_not_found(
        &self,
        script: &str,
        arguments: Vec<Value>,
    ) -> WebDriverResult<ScriptRet> {
        let result = self.driver.execute(script, arguments.clone()).await?;

        let string_value = result.json().as_str();
        if string_value == Some(Self::LIBRARY_NOT_FOUND_ERROR) {
            Screen::load_testing_library(&self.driver).await?;
            return self.driver.execute(script, arguments).await;
        }

        Ok(result)
    }
}

/// Options enum for unified option handling
#[derive(Debug, Clone)]
pub enum Options {
    /// Role-based query options
    Role(ByRoleOptions),
    /// Text-based query options
    Text(ByTextOptions),
    /// Label text query options
    LabelText(ByLabelTextOptions),
    /// Placeholder text query options
    PlaceholderText(ByPlaceholderTextOptions),
    /// Display value query options
    DisplayValue(ByDisplayValueOptions),
    /// Alt text query options
    AltText(ByAltTextOptions),
    /// Title query options
    Title(ByTitleOptions),
    /// Test ID query options
    TestId(ByTestIdOptions),
}

impl Options {
    /// Serialize the options to JSON string
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        match self {
            Options::Role(options) => options.to_json_string(),
            Options::Text(options) => options.to_json_string(),
            Options::LabelText(options) => options.to_json_string(),
            Options::PlaceholderText(options) => options.to_json_string(),
            Options::DisplayValue(options) => options.to_json_string(),
            Options::AltText(options) => options.to_json_string(),
            Options::Title(options) => options.to_json_string(),
            Options::TestId(options) => options.to_json_string(),
        }
    }
}

/// Fluent builder for role-based queries with comprehensive options
#[derive(Debug, Clone)]
pub struct RoleSelector {
    value: String,
    options: ByRoleOptions,
}

impl RoleSelector {
    /// Create a new RoleSelector with the given role value
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            options: ByRoleOptions::default(),
        }
    }

    /// Set the hidden option - include elements normally excluded from accessibility tree
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.options.hidden = Some(hidden);
        self
    }

    /// Set the name option - filter by accessible name
    /// Accepts strings and automatically detects regex patterns (strings starting and ending with '/')
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        self.options.name = Some(TextMatch::from(name_str));
        self
    }

    /// Set the description option - filter by accessible description  
    /// Accepts strings and automatically detects regex patterns (strings starting and ending with '/')
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let desc_str = description.into();
        self.options.description = Some(TextMatch::from(desc_str));
        self
    }

    /// Set the selected option - filter by selected state (aria-selected)
    pub fn selected(mut self, selected: bool) -> Self {
        self.options.selected = Some(selected);
        self
    }

    /// Set the busy option - filter by busy state (aria-busy)
    pub fn busy(mut self, busy: bool) -> Self {
        self.options.busy = Some(busy);
        self
    }

    /// Set the checked option - filter by checked state (aria-checked)
    pub fn checked(mut self, checked: bool) -> Self {
        self.options.checked = Some(checked);
        self
    }

    /// Set the pressed option - filter by pressed state (aria-pressed)
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.options.pressed = Some(pressed);
        self
    }

    /// Set the suggest option - enable/disable query suggestions
    pub fn suggest(mut self, suggest: bool) -> Self {
        self.options.suggest = Some(suggest);
        self
    }

    /// Set the current option - filter by current state (aria-current)
    pub fn current(mut self, current: CurrentState) -> Self {
        self.options.current = Some(current);
        self
    }

    /// Set the expanded option - filter by expanded state (aria-expanded)
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.options.expanded = Some(expanded);
        self
    }

    /// Set the query_fallbacks option - enable querying fallback roles
    pub fn query_fallbacks(mut self, query_fallbacks: bool) -> Self {
        self.options.query_fallbacks = Some(query_fallbacks);
        self
    }

    /// Set the level option - filter by heading level (only for heading role)
    pub fn level(mut self, level: u8) -> Self {
        self.options.level = Some(level);
        self
    }

    /// Set the value option - filter by value properties (only for range widgets)
    pub fn value(mut self, value: ValueOptions) -> Self {
        self.options.value = Some(value);
        self
    }
}

impl From<RoleSelector> for By {
    fn from(selector: RoleSelector) -> Self {
        By::Role(selector.value, Some(Options::Role(selector.options)))
    }
}

/// Fluent builder for simple queries that only support exact matching
#[derive(Debug, Clone)]
pub struct SimpleSelector {
    value: String,
    selector_type: SimpleSelectorType,
    options: SimpleOptions,
}

#[derive(Debug, Clone)]
enum SimpleSelectorType {
    Text,
    AltText,
    DisplayValue,
    PlaceholderText,
    TestId,
    Title,
}

impl SimpleSelector {
    /// Create a new SimpleSelector with the given value and type
    fn new(value: impl Into<String>, selector_type: SimpleSelectorType) -> Self {
        Self {
            value: value.into(),
            selector_type,
            options: SimpleOptions::default(),
        }
    }

    /// Set the exact option - whether to use exact text matching
    pub fn exact(mut self, exact: bool) -> Self {
        self.options.exact = Some(exact);
        self
    }
}

impl From<SimpleSelector> for By {
    fn from(selector: SimpleSelector) -> Self {
        let options = if selector.options.exact.is_some() {
            Some(match selector.selector_type {
                SimpleSelectorType::Text => Options::Text(selector.options),
                SimpleSelectorType::AltText => Options::AltText(selector.options),
                SimpleSelectorType::DisplayValue => Options::DisplayValue(selector.options),
                SimpleSelectorType::PlaceholderText => Options::PlaceholderText(selector.options),
                SimpleSelectorType::TestId => Options::TestId(selector.options),
                SimpleSelectorType::Title => Options::Title(selector.options),
            })
        } else {
            None
        };

        match selector.selector_type {
            SimpleSelectorType::Text => By::Text(selector.value, options),
            SimpleSelectorType::AltText => By::AltText(selector.value, options),
            SimpleSelectorType::DisplayValue => By::DisplayValue(selector.value, options),
            SimpleSelectorType::PlaceholderText => By::PlaceholderText(selector.value, options),
            SimpleSelectorType::TestId => By::TestId(selector.value, options),
            SimpleSelectorType::Title => By::Title(selector.value, options),
        }
    }
}

/// Fluent builder for label text queries with selector and exact options
#[derive(Debug, Clone)]
pub struct LabelTextSelector {
    value: String,
    options: ByLabelTextOptions,
}

impl LabelTextSelector {
    /// Create a new LabelTextSelector with the given label text value
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            options: ByLabelTextOptions::default(),
        }
    }

    /// Set the selector option - CSS selector to filter elements
    pub fn selector(mut self, selector: impl Into<String>) -> Self {
        self.options.selector = Some(selector.into());
        self
    }

    /// Set the exact option - whether to use exact text matching
    pub fn exact(mut self, exact: bool) -> Self {
        self.options.exact = Some(exact);
        self
    }
}

impl From<LabelTextSelector> for By {
    fn from(selector: LabelTextSelector) -> Self {
        let options = if selector.options.selector.is_some() || selector.options.exact.is_some() {
            Some(Options::LabelText(selector.options))
        } else {
            None
        };
        By::LabelText(selector.value, options)
    }
}

impl By {
    /// Create a role selector without options
    pub fn role(value: &str) -> RoleSelector {
        RoleSelector::new(value)
    }

    /// Create a text selector without options
    pub fn text(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::Text)
    }

    /// Create a label text selector without options
    pub fn label_text(value: &str) -> LabelTextSelector {
        LabelTextSelector::new(value)
    }

    /// Create a placeholder text selector without options
    pub fn placeholder_text(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::PlaceholderText)
    }

    /// Create a display value selector without options
    pub fn display_value(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::DisplayValue)
    }

    /// Create an alt text selector without options
    pub fn alt_text(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::AltText)
    }

    /// Create a title selector without options
    pub fn title(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::Title)
    }

    /// Create a test ID selector without options
    pub fn test_id(value: &str) -> SimpleSelector {
        SimpleSelector::new(value, SimpleSelectorType::TestId)
    }

    /// Returns the function suffix for the Testing Library method name
    fn function_suffix(&self) -> &str {
        match self {
            By::Role(_, _) => "Role",
            By::Text(_, _) => "Text",
            By::LabelText(_, _) => "LabelText",
            By::PlaceholderText(_, _) => "PlaceholderText",
            By::DisplayValue(_, _) => "DisplayValue",
            By::AltText(_, _) => "AltText",
            By::Title(_, _) => "Title",
            By::TestId(_, _) => "TestId",
        }
    }

    /// Returns the selector value (text, role, etc.)
    fn value(&self) -> &str {
        match self {
            By::Role(value, _) => value,
            By::Text(value, _) => value,
            By::LabelText(value, _) => value,
            By::PlaceholderText(value, _) => value,
            By::DisplayValue(value, _) => value,
            By::AltText(value, _) => value,
            By::Title(value, _) => value,
            By::TestId(value, _) => value,
        }
    }

    /// Returns the options if any
    fn options(&self) -> &Option<Options> {
        match self {
            By::Role(_, options) => options,
            By::Text(_, options) => options,
            By::LabelText(_, options) => options,
            By::PlaceholderText(_, options) => options,
            By::DisplayValue(_, options) => options,
            By::AltText(_, options) => options,
            By::Title(_, options) => options,
            By::TestId(_, options) => options,
        }
    }

    /// Returns the serialized options JSON string if any
    fn options_json(&self) -> Result<Option<String>, WebDriverError> {
        match self.options() {
            Some(options) => options
                .to_json_string()
                .map(Some)
                .map_err(|e| WebDriverError::Json(format!("Failed to serialize options: {e}"))),
            None => Ok(None),
        }
    }
}

/// Selector enum for unified DOM queries
#[derive(Debug, Clone)]
pub enum By {
    /// Query by element role
    Role(String, Option<Options>),
    /// Query by text content
    Text(String, Option<Options>),
    /// Query by label text
    LabelText(String, Option<Options>),
    /// Query by placeholder text
    PlaceholderText(String, Option<Options>),
    /// Query by display value
    DisplayValue(String, Option<Options>),
    /// Query by alt text
    AltText(String, Option<Options>),
    /// Query by title
    Title(String, Option<Options>),
    /// Query by test ID
    TestId(String, Option<Options>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_regex_pattern() {
        // Valid regex patterns
        assert!(QueryExecutor::is_regex_pattern("/hello/"));
        assert!(QueryExecutor::is_regex_pattern("/hello/i"));
        assert!(QueryExecutor::is_regex_pattern("/^submit.*$/i"));
        assert!(QueryExecutor::is_regex_pattern("/Hello W?oRlD/i"));

        // Invalid regex patterns (not regex)
        assert!(!QueryExecutor::is_regex_pattern("hello"));
        assert!(!QueryExecutor::is_regex_pattern("hello/"));
        assert!(!QueryExecutor::is_regex_pattern("/hello"));
        assert!(!QueryExecutor::is_regex_pattern("/"));
        assert!(!QueryExecutor::is_regex_pattern(""));
        assert!(!QueryExecutor::is_regex_pattern("regular text"));
    }

    #[test]
    fn test_format_query_value() {
        // Regular strings should be quoted
        assert_eq!(QueryExecutor::format_query_value("hello"), "'hello'");
        assert_eq!(
            QueryExecutor::format_query_value("Hello World"),
            "'Hello World'"
        );
        assert_eq!(QueryExecutor::format_query_value("Submit"), "'Submit'");

        // Regex patterns should use marker system
        assert_eq!(
            QueryExecutor::format_query_value("/hello/"),
            "\"__RAW_JS__/hello/\""
        );
        assert_eq!(
            QueryExecutor::format_query_value("/hello/i"),
            "\"__RAW_JS__/hello/i\""
        );
        assert_eq!(
            QueryExecutor::format_query_value("/^submit.*$/i"),
            "\"__RAW_JS__/^submit.*$/i\""
        );
    }

    #[test]
    fn test_regex_functionality_examples() {
        // Test that our TextMatch From implementation works correctly
        let exact_text = TextMatch::from("Hello World");
        assert!(matches!(exact_text, TextMatch::Exact(_)));
        assert_eq!(exact_text.text_value(), "Hello World");
        assert!(!exact_text.is_regex());

        let regex_text = TextMatch::from("/hello/i");
        assert!(matches!(regex_text, TextMatch::Regex(_)));
        assert_eq!(regex_text.text_value(), "/hello/i");
        assert!(regex_text.is_regex());
    }

    #[test]
    fn test_process_raw_javascript_markers() {
        // Test the marker processing function works correctly
        let json_with_markers =
            r#"return window.__TL__.getByText(document, "__RAW_JS__/Hello World/i");"#;
        let processed = process_raw_javascript_markers(json_with_markers);
        assert_eq!(
            processed,
            r#"return window.__TL__.getByText(document, /Hello World/i);"#
        );

        // Test with regular strings (should remain unchanged)
        let json_no_markers = r#"return window.__TL__.getByText(document, 'Hello World');"#;
        let processed_no_markers = process_raw_javascript_markers(json_no_markers);
        assert_eq!(
            processed_no_markers,
            r#"return window.__TL__.getByText(document, 'Hello World');"#
        );
    }
}
