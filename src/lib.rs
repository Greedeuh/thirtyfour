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

/// Alt text query options and utilities
pub mod alt_text;
/// Configuration options for the testing library
pub mod configure;
/// Display value query options and utilities
pub mod display_value;
/// Label text query options and utilities
pub mod label_text;
/// Placeholder text query options and utilities
pub mod placeholder_text;
/// Role-based query options and utilities
pub mod role;
/// Test ID query options and utilities
pub mod test_id;
/// Text query options and utilities
pub mod text;
/// Title query options and utilities
pub mod title;

pub use alt_text::*;
pub use display_value::*;
pub use label_text::*;
pub use placeholder_text::*;
pub use role::*;
use serde_json::Value;
pub use test_id::*;
pub use text::*;
pub use title::*;

use std::fs;

use thirtyfour::{error::WebDriverResult, prelude::ScriptRet, WebDriver, WebElement};

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
    pub async fn get(&self, selector: By) -> WebDriverResult<WebElement> {
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
    pub async fn get_all(&self, selector: By) -> WebDriverResult<Vec<WebElement>> {
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
    pub async fn query(&self, selector: By) -> WebDriverResult<Option<WebElement>> {
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
    pub async fn query_all(&self, selector: By) -> WebDriverResult<Vec<WebElement>> {
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
    pub async fn find(&self, selector: By) -> WebDriverResult<WebElement> {
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
    pub async fn find_all(&self, selector: By) -> WebDriverResult<Vec<WebElement>> {
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
                thirtyfour::error::WebDriverError::Json(
                    "logTestingPlaygroundURL returned non-string value".to_string(),
                )
            })
            .map(|s| s.to_string())
    }

    async fn load_testing_library(driver: &WebDriver) -> WebDriverResult<()> {
        // Load the testing library script in the browser
        let testing_library = fs::read_to_string("js/testing-library.js").unwrap();
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
        let wrapped_script = self.wrap_load_retry(&script);
        self.execute_and_retry_if_library_not_found(&wrapped_script, arguments).await
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

        self.execute_and_retry_if_library_not_found(&script, arguments).await
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
        let script =
            self.query_script(method_name, container, value, options_json, with_null_filter);
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
        let base_call = match options_json {
            Some(options) => {
                format!("window.__TL__.{method_name}({container}, '{value}', {options})")
            }
            None => {
                format!("window.__TL__.{method_name}({container}, '{value}')")
            }
        };

        if with_null_filter {
            // Transform null values to empty arrays easier to parse in Rust
            format!("return [{base_call}].filter(n => n);")
        } else {
            format!("return {base_call};")
        }
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

impl By {
    /// Create a role selector without options
    pub fn role(value: impl Into<String>) -> Self {
        Self::Role(value.into(), None)
    }

    /// Create a role selector with options
    pub fn role_with_options(value: impl Into<String>, options: ByRoleOptions) -> Self {
        Self::Role(value.into(), Some(Options::Role(options)))
    }

    /// Create a text selector without options
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into(), None)
    }

    /// Create a text selector with options
    pub fn text_with_options(value: impl Into<String>, options: ByTextOptions) -> Self {
        Self::Text(value.into(), Some(Options::Text(options)))
    }

    /// Create a label text selector without options
    pub fn label_text(value: impl Into<String>) -> Self {
        Self::LabelText(value.into(), None)
    }

    /// Create a label text selector with options
    pub fn label_text_with_options(value: impl Into<String>, options: ByLabelTextOptions) -> Self {
        Self::LabelText(value.into(), Some(Options::LabelText(options)))
    }

    /// Create a placeholder text selector without options
    pub fn placeholder_text(value: impl Into<String>) -> Self {
        Self::PlaceholderText(value.into(), None)
    }

    /// Create a placeholder text selector with options
    pub fn placeholder_text_with_options(
        value: impl Into<String>,
        options: ByPlaceholderTextOptions,
    ) -> Self {
        Self::PlaceholderText(value.into(), Some(Options::PlaceholderText(options)))
    }

    /// Create a display value selector without options
    pub fn display_value(value: impl Into<String>) -> Self {
        Self::DisplayValue(value.into(), None)
    }

    /// Create a display value selector with options
    pub fn display_value_with_options(
        value: impl Into<String>,
        options: ByDisplayValueOptions,
    ) -> Self {
        Self::DisplayValue(value.into(), Some(Options::DisplayValue(options)))
    }

    /// Create an alt text selector without options
    pub fn alt_text(value: impl Into<String>) -> Self {
        Self::AltText(value.into(), None)
    }

    /// Create an alt text selector with options
    pub fn alt_text_with_options(value: impl Into<String>, options: ByAltTextOptions) -> Self {
        Self::AltText(value.into(), Some(Options::AltText(options)))
    }

    /// Create a title selector without options
    pub fn title(value: impl Into<String>) -> Self {
        Self::Title(value.into(), None)
    }

    /// Create a title selector with options
    pub fn title_with_options(value: impl Into<String>, options: ByTitleOptions) -> Self {
        Self::Title(value.into(), Some(Options::Title(options)))
    }

    /// Create a test ID selector without options
    pub fn test_id(value: impl Into<String>) -> Self {
        Self::TestId(value.into(), None)
    }

    /// Create a test ID selector with options
    pub fn test_id_with_options(value: impl Into<String>, options: ByTestIdOptions) -> Self {
        Self::TestId(value.into(), Some(Options::TestId(options)))
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
    fn options_json(&self) -> Result<Option<String>, thirtyfour::error::WebDriverError> {
        match self.options() {
            Some(options) => options.to_json_string().map(Some).map_err(|e| {
                thirtyfour::error::WebDriverError::Json(format!("Failed to serialize options: {e}"))
            }),
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
