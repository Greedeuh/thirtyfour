/// Alt text query options and utilities
pub mod alt_text;
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

use crate::{error::WebDriverResult, prelude::ScriptRet, WebDriver, WebElement};

// TODO
// - what happen when we change page? do we need to reload the script?
// - support configure
// - better error handling
// - logTestingPlaygroundURL


/// A struct representing a screen in the testing library that provides DOM queries with different behaviors: get* methods throw errors if elements aren't found, query* methods return null for missing elements, and find* methods return promises that retry until elements are found.
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver,
    within_element: Option<WebElement>,
}

impl Screen {
    /// Creates a new `Screen` and loads the testing library script in the browser
    pub async fn load_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        // Load the testing library script in the browser
        let testing_library = fs::read_to_string("js/testing-library.js").unwrap();
        driver.execute(testing_library, vec![]).await?;

        Ok(Screen {
            driver,
    within_element: None,
        })
    }

    /// Creates a new `Screen` wich will be scoped to a specific element
    pub fn within(&self, element: WebElement) -> Screen {
        Screen {
            driver: self.driver.clone(),
            within_element: Some(element),
        }
    }

    /// Unified get method that accepts a Selector enum and returns a single WebElement
    /// Throws an error if no elements match or if more than one match is found
    pub async fn get(&self, selector: Selector) -> WebDriverResult<WebElement> {
        let options_json = selector.options_json()?;
        self.execute_tl_selector(
            "getBy",
            selector.function_suffix(),
            selector.value(),
            options_json,
        )
        .await?
        .element()
    }

    /// Unified get_all method that accepts a Selector enum and returns all matching WebElements
    /// Throws an error if no elements match
    pub async fn get_all(&self, selector: Selector) -> WebDriverResult<Vec<WebElement>> {
        let options_json = selector.options_json()?;
        self.execute_tl_selector(
            "getAllBy",
            selector.function_suffix(),
            selector.value(),
            options_json,
        )
        .await?
        .elements()
    }

    /// Unified query method that accepts a Selector enum and returns a single WebElement
    /// Returns None if no elements match
    pub async fn query(&self, selector: Selector) -> WebDriverResult<Option<WebElement>> {
        let options_json = selector.options_json()?;

        let mut elements = self.execute_tl_selector_with_null_filter( "queryBy",
            selector.function_suffix(),
            selector.value(),
            options_json).await?.elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Unified query_all method that accepts a Selector enum and returns all matching WebElements
    /// Returns empty Vec if no elements match
    pub async fn query_all(&self, selector: Selector) -> WebDriverResult<Vec<WebElement>> {
        let options_json = selector.options_json()?;
        self.execute_tl_selector(
            "queryAllBy",
            selector.function_suffix(),
            selector.value(),
            options_json,
        )
        .await?
        .elements()
    }

    /// Unified find method that accepts a Selector enum and returns a single WebElement
    /// Waits for the element to appear and throws an error if not found
    pub async fn find(&self, selector: Selector) -> WebDriverResult<WebElement> {
        let options_json = selector.options_json()?;
        self.execute_tl_selector(
            "findBy",
            selector.function_suffix(),
            selector.value(),
            options_json,
        )
        .await?
        .element()
    }

    /// Unified find_all method that accepts a Selector enum and returns all matching WebElements
    /// Waits for elements to appear and throws an error if none are found
    pub async fn find_all(&self, selector: Selector) -> WebDriverResult<Vec<WebElement>> {
        let options_json = selector.options_json()?;
        self.execute_tl_selector(
            "findAllBy",
            selector.function_suffix(),
            selector.value(),
            options_json,
        )
        .await?
        .elements()
    }


    // Internal helper method for executing Testing Library methods with unified parameters
    async fn execute_tl_selector_with_null_filter(
        &self,
        method_prefix: &str,
        function_suffix: &str,
        value: &str,
        options_json: Option<String>,
    ) -> WebDriverResult<ScriptRet> {
        let method_name = format!("{}{}", method_prefix, function_suffix);

        let (container, arguments) = self.container_and_arguments()?;

        
        // Use a filter to remove null values from the result
        // this simplify the deserialization back to WebElement: null becomes an empty arry => []
        let script = match options_json {
            Some(options) => {
                format!(
                    "return [window.__TL__.{}({}, '{}', {})].filter(n => n);",
                    method_name,
                    container,
                    value,
                    options
                )
            }
            None => {
                format!(
                    "return [window.__TL__.{}({}, '{}')].filter(n => n);",
                    method_name,
                    container,
                    value
                )
            }
        };


        self.driver.execute(script, arguments).await
    }

    // Internal helper method for executing Testing Library methods with unified parameters
    async fn execute_tl_selector(
        &self,
        method_prefix: &str,
        function_suffix: &str,
        value: &str,
        options_json: Option<String>,
    ) -> WebDriverResult<ScriptRet> {
        let method_name = format!("{}{}", method_prefix, function_suffix);

        let (container, arguments) = self.container_and_arguments()?;

        let script = match options_json {
            Some(options) => {
                format!("return window.__TL__.{}({}, '{}', {});", method_name,container, value, options)
            }
            None => {
                format!("return window.__TL__.{}({}, '{}');", method_name,container, value)
            }
        };

        self.driver.execute(script, arguments).await
    }

    fn container_and_arguments(&self) -> WebDriverResult<(&str, Vec<Value>)> {
        if let Some(within_element) = &self.within_element {
            Ok(("arguments[0]", vec![within_element.to_json()?]))
        } else {
            Ok(("document", vec![]))
        }
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

impl Selector {
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
            Selector::Role(_, _) => "Role",
            Selector::Text(_, _) => "Text",
            Selector::LabelText(_, _) => "LabelText",
            Selector::PlaceholderText(_, _) => "PlaceholderText",
            Selector::DisplayValue(_, _) => "DisplayValue",
            Selector::AltText(_, _) => "AltText",
            Selector::Title(_, _) => "Title",
            Selector::TestId(_, _) => "TestId",
        }
    }

    /// Returns the selector value (text, role, etc.)
    fn value(&self) -> &str {
        match self {
            Selector::Role(value, _) => value,
            Selector::Text(value, _) => value,
            Selector::LabelText(value, _) => value,
            Selector::PlaceholderText(value, _) => value,
            Selector::DisplayValue(value, _) => value,
            Selector::AltText(value, _) => value,
            Selector::Title(value, _) => value,
            Selector::TestId(value, _) => value,
        }
    }

    /// Returns the options if any
    fn options(&self) -> &Option<Options> {
        match self {
            Selector::Role(_, options) => options,
            Selector::Text(_, options) => options,
            Selector::LabelText(_, options) => options,
            Selector::PlaceholderText(_, options) => options,
            Selector::DisplayValue(_, options) => options,
            Selector::AltText(_, options) => options,
            Selector::Title(_, options) => options,
            Selector::TestId(_, options) => options,
        }
    }

    /// Returns the serialized options JSON string if any
    fn options_json(&self) -> Result<Option<String>, crate::error::WebDriverError> {
        match self.options() {
            Some(options) => options.to_json_string().map(Some).map_err(|e| {
                crate::error::WebDriverError::Json(format!("Failed to serialize options: {}", e))
            }),
            None => Ok(None),
        }
    }
}

/// Selector enum for unified DOM queries
#[derive(Debug, Clone)]
pub enum Selector {
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
