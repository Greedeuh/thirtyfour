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
pub use test_id::*;
pub use text::*;
pub use title::*;

use std::fs;

use crate::{error::WebDriverResult, prelude::ScriptRet, WebDriver, WebElement};

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
    pub fn placeholder_text_with_options(value: impl Into<String>, options: ByPlaceholderTextOptions) -> Self {
        Self::PlaceholderText(value.into(), Some(Options::PlaceholderText(options)))
    }

    /// Create a display value selector without options
    pub fn display_value(value: impl Into<String>) -> Self {
        Self::DisplayValue(value.into(), None)
    }

    /// Create a display value selector with options
    pub fn display_value_with_options(value: impl Into<String>, options: ByDisplayValueOptions) -> Self {
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
                crate::error::WebDriverError::Json(format!(
                    "Failed to serialize options: {}",
                    e
                ))
            }),
            None => Ok(None),
        }
    }
}

/// A struct representing a screen in the testing library that provides DOM queries with different behaviors: get* methods throw errors if elements aren't found, query* methods return null for missing elements, and find* methods return promises that retry until elements are found.
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver,
}

impl Screen {
    /// Creates a new `Screen` and loads the testing library script in the browser
    pub async fn load_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        // Load the testing library script in the browser
        let testing_library = fs::read_to_string("js/testing-library.js").unwrap();
        driver.execute(testing_library, vec![]).await?;

        Ok(Screen {
            driver,
        })
    }

    // Internal helper method for executing Testing Library methods
    async fn execute_tl_method(&self, method: &str, selector: &str) -> WebDriverResult<ScriptRet> {
        self.driver
            .execute(format!("return window.__TL__.{}(document, '{}');", method, selector), vec![])
            .await
    }

    // Internal helper method for executing Testing Library methods with array wrapper
    async fn execute_tl_method_with_filter(
        &self,
        method: &str,
        selector: &str,
    ) -> WebDriverResult<ScriptRet> {
        self.driver
            .execute(
                format!(
                    "return [window.__TL__.{}(document, '{}')].filter(n => n);",
                    method, selector
                ),
                vec![],
            )
            .await
    }

    // Internal helper method for executing Testing Library role methods with options
    async fn execute_tl_role_method(
        &self,
        method: &str,
        role: &str,
        options: Option<&ByRoleOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize role options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, role, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, role)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library role methods with options and array filter
    async fn execute_tl_role_method_with_filter(
        &self,
        method: &str,
        role: &str,
        options: Option<&ByRoleOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize role options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, role, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, role)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library label text methods with options
    async fn execute_tl_labeltext_method(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByLabelTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize label text options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, text, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library label text methods with options and array filter
    async fn execute_tl_labeltext_method_with_filter(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByLabelTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize label text options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, text, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library text methods with options
    async fn execute_tl_text_method(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize text options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, text, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library text methods with options and array filter
    async fn execute_tl_text_method_with_filter(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize text options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, text, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library placeholder text methods with options
    async fn execute_tl_placeholder_text_method(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByPlaceholderTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize placeholder text options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, text, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library placeholder text methods with options and array filter
    async fn execute_tl_placeholder_text_method_with_filter(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByPlaceholderTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize placeholder text options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, text, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library display value methods with options
    async fn execute_tl_display_value_method(
        &self,
        method: &str,
        value: &str,
        options: Option<&ByDisplayValueOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize display value options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, value, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, value)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library display value methods with options and array filter
    async fn execute_tl_display_value_method_with_filter(
        &self,
        method: &str,
        value: &str,
        options: Option<&ByDisplayValueOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize display value options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, value, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, value)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library alt text methods with options
    async fn execute_tl_alt_text_method(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByAltTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize alt text options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, text, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library alt text methods with options and array filter
    async fn execute_tl_alt_text_method_with_filter(
        &self,
        method: &str,
        text: &str,
        options: Option<&ByAltTextOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize alt text options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, text, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, text)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library title methods with options
    async fn execute_tl_title_method(
        &self,
        method: &str,
        title: &str,
        options: Option<&ByTitleOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize title options: {}",
                        e
                    ))
                })?;

                format!("return window.__TL__.{}(document, '{}', {});", method, title, options_json)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, title)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library title methods with options and array filter
    async fn execute_tl_title_method_with_filter(
        &self,
        method: &str,
        title: &str,
        options: Option<&ByTitleOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize title options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, title, options_json
                )
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, title)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library test ID methods with options
    async fn execute_tl_test_id_method(
        &self,
        method: &str,
        test_id: &str,
        options: Option<&ByTestIdOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize test ID options: {}",
                        e
                    ))
                })?;

                format!(
                    "return window.__TL__.{}(document, '{}', {});",
                    method, test_id, options_json
                )
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method, test_id)
            }
        };

        self.driver.execute(script, vec![]).await
    }

    // Internal helper method for executing Testing Library test ID methods with options and array filter
    async fn execute_tl_test_id_method_with_filter(
        &self,
        method: &str,
        test_id: &str,
        options: Option<&ByTestIdOptions>,
    ) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!(
                        "Failed to serialize test ID options: {}",
                        e
                    ))
                })?;

                format!(
                    "return [window.__TL__.{}(document, '{}', {})].filter(n => n);",
                    method, test_id, options_json
                )
            }
            None => {
                format!(
                    "return [window.__TL__.{}(document, '{}')].filter(n => n);",
                    method, test_id
                )
            }
        };

        self.driver.execute(script, vec![]).await
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

        let script = match options_json {
            Some(options) => {
                format!("return window.__TL__.{}(document, '{}', {});", method_name, value, options)
            }
            None => {
                format!("return window.__TL__.{}(document, '{}');", method_name, value)
            }
        };

        self.driver.execute(script, vec![]).await
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

    // Role-based methods

    /// Returns an array of all matching elements for a query by role, throws an error if no elements match.
    pub async fn get_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByRole", role).await?.elements()
    }

    /// Returns an array of all matching elements for a query by role with options, throws an error if no elements match.
    pub async fn get_all_by_role_with_options(
        &self,
        role: &str,
        options: &ByRoleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("getAllByRole", role, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by role, returns None if no elements match.
    pub async fn query_by_role(&self, role: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByRole", role).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by role with options, returns None if no elements match.
    pub async fn query_by_role_with_options(
        &self,
        role: &str,
        options: &ByRoleOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_role_method_with_filter("queryByRole", role, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by role, returns empty array if no elements match.
    pub async fn query_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByRole", role).await?.elements()
    }

    /// Returns an array of all matching elements for a query by role with options, returns empty array if no elements match.
    pub async fn query_all_by_role_with_options(
        &self,
        role: &str,
        options: &ByRoleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("queryAllByRole", role, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given role query.
    pub async fn find_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByRole", role).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given role query with options.
    pub async fn find_by_role_with_options(
        &self,
        role: &str,
        options: &ByRoleOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_role_method("findByRole", role, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given role query.
    pub async fn find_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByRole", role).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given role query with options.
    pub async fn find_all_by_role_with_options(
        &self,
        role: &str,
        options: &ByRoleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("findAllByRole", role, Some(options)).await?.elements()
    }

    // Text-based methods

    /// Returns an array of all matching elements for a query by text content, throws an error if no elements match.
    pub async fn get_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by text content with options, throws an error if no elements match.
    pub async fn get_all_by_text_with_options(
        &self,
        text: &str,
        options: &ByTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_text_method("getAllByText", text, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by text content, returns None if no elements match.
    pub async fn query_by_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by text content with options, returns None if no elements match.
    pub async fn query_by_text_with_options(
        &self,
        text: &str,
        options: &ByTextOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_text_method_with_filter("queryByText", text, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by text content, returns empty array if no elements match.
    pub async fn query_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by text content with options, returns empty array if no elements match.
    pub async fn query_all_by_text_with_options(
        &self,
        text: &str,
        options: &ByTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_text_method("queryAllByText", text, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given text content query.
    pub async fn find_by_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByText", text).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given text content query with options.
    pub async fn find_by_text_with_options(
        &self,
        text: &str,
        options: &ByTextOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_text_method("findByText", text, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given text content query.
    pub async fn find_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByText", text).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given text content query with options.
    pub async fn find_all_by_text_with_options(
        &self,
        text: &str,
        options: &ByTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_text_method("findAllByText", text, Some(options)).await?.elements()
    }

    // Label text methods

    /// Returns an array of all matching elements for a query by label text, throws an error if no elements match.
    pub async fn get_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByLabelText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by label text with options, throws an error if no elements match.
    pub async fn get_all_by_label_text_with_options(
        &self,
        text: &str,
        options: &ByLabelTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_labeltext_method("getAllByLabelText", text, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by label text, returns None if no elements match.
    pub async fn query_by_label_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByLabelText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by label text with options, returns None if no elements match.
    pub async fn query_by_label_text_with_options(
        &self,
        text: &str,
        options: &ByLabelTextOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_labeltext_method_with_filter("queryByLabelText", text, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by label text, returns empty array if no elements match.
    pub async fn query_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByLabelText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by label text with options, returns empty array if no elements match.
    pub async fn query_all_by_label_text_with_options(
        &self,
        text: &str,
        options: &ByLabelTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_labeltext_method("queryAllByLabelText", text, Some(options))
            .await?
            .elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given label text query.
    pub async fn find_by_label_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByLabelText", text).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given label text query with options.
    pub async fn find_by_label_text_with_options(
        &self,
        text: &str,
        options: &ByLabelTextOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_labeltext_method("findByLabelText", text, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given label text query.
    pub async fn find_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByLabelText", text).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given label text query with options.
    pub async fn find_all_by_label_text_with_options(
        &self,
        text: &str,
        options: &ByLabelTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_labeltext_method("findAllByLabelText", text, Some(options))
            .await?
            .elements()
    }

    // Placeholder text methods

    /// Returns an array of all matching elements for a query by placeholder text, throws an error if no elements match.
    pub async fn get_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByPlaceholderText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by placeholder text with options, throws an error if no elements match.
    pub async fn get_all_by_placeholder_text_with_options(
        &self,
        text: &str,
        options: &ByPlaceholderTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_placeholder_text_method("getAllByPlaceholderText", text, Some(options))
            .await?
            .elements()
    }

    /// Returns the matching element for a query by placeholder text, returns None if no elements match.
    pub async fn query_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByPlaceholderText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by placeholder text with options, returns None if no elements match.
    pub async fn query_by_placeholder_text_with_options(
        &self,
        text: &str,
        options: &ByPlaceholderTextOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_placeholder_text_method_with_filter(
                "queryByPlaceholderText",
                text,
                Some(options),
            )
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by placeholder text, returns empty array if no elements match.
    pub async fn query_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByPlaceholderText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by placeholder text with options, returns empty array if no elements match.
    pub async fn query_all_by_placeholder_text_with_options(
        &self,
        text: &str,
        options: &ByPlaceholderTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_placeholder_text_method("queryAllByPlaceholderText", text, Some(options))
            .await?
            .elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given placeholder text query.
    pub async fn find_by_placeholder_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByPlaceholderText", text).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given placeholder text query with options.
    pub async fn find_by_placeholder_text_with_options(
        &self,
        text: &str,
        options: &ByPlaceholderTextOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_placeholder_text_method("findByPlaceholderText", text, Some(options))
            .await?
            .element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given placeholder text query.
    pub async fn find_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByPlaceholderText", text).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given placeholder text query with options.
    pub async fn find_all_by_placeholder_text_with_options(
        &self,
        text: &str,
        options: &ByPlaceholderTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_placeholder_text_method("findAllByPlaceholderText", text, Some(options))
            .await?
            .elements()
    }

    // Display value methods

    /// Returns an array of all matching elements for a query by display value, throws an error if no elements match.
    pub async fn get_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByDisplayValue", value).await?.elements()
    }

    /// Returns an array of all matching elements for a query by display value with options, throws an error if no elements match.
    pub async fn get_all_by_display_value_with_options(
        &self,
        value: &str,
        options: &ByDisplayValueOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_display_value_method("getAllByDisplayValue", value, Some(options))
            .await?
            .elements()
    }

    /// Returns the matching element for a query by display value, returns None if no elements match.
    pub async fn query_by_display_value(&self, value: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByDisplayValue", value).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by display value with options, returns None if no elements match.
    pub async fn query_by_display_value_with_options(
        &self,
        value: &str,
        options: &ByDisplayValueOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_display_value_method_with_filter(
                "queryByDisplayValue",
                value,
                Some(options),
            )
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by display value, returns empty array if no elements match.
    pub async fn query_all_by_display_value(
        &self,
        value: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByDisplayValue", value).await?.elements()
    }

    /// Returns an array of all matching elements for a query by display value with options, returns empty array if no elements match.
    pub async fn query_all_by_display_value_with_options(
        &self,
        value: &str,
        options: &ByDisplayValueOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_display_value_method("queryAllByDisplayValue", value, Some(options))
            .await?
            .elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given display value query.
    pub async fn find_by_display_value(&self, value: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByDisplayValue", value).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given display value query with options.
    pub async fn find_by_display_value_with_options(
        &self,
        value: &str,
        options: &ByDisplayValueOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_display_value_method("findByDisplayValue", value, Some(options))
            .await?
            .element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given display value query.
    pub async fn find_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByDisplayValue", value).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given display value query with options.
    pub async fn find_all_by_display_value_with_options(
        &self,
        value: &str,
        options: &ByDisplayValueOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_display_value_method("findAllByDisplayValue", value, Some(options))
            .await?
            .elements()
    }

    // Alt text methods

    /// Returns an array of all matching elements for a query by alt text, throws an error if no elements match.
    pub async fn get_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByAltText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by alt text with options, throws an error if no elements match.
    pub async fn get_all_by_alt_text_with_options(
        &self,
        text: &str,
        options: &ByAltTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_alt_text_method("getAllByAltText", text, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by alt text, returns None if no elements match.
    pub async fn query_by_alt_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByAltText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by alt text with options, returns None if no elements match.
    pub async fn query_by_alt_text_with_options(
        &self,
        text: &str,
        options: &ByAltTextOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_alt_text_method_with_filter("queryByAltText", text, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by alt text, returns empty array if no elements match.
    pub async fn query_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByAltText", text).await?.elements()
    }

    /// Returns an array of all matching elements for a query by alt text with options, returns empty array if no elements match.
    pub async fn query_all_by_alt_text_with_options(
        &self,
        text: &str,
        options: &ByAltTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_alt_text_method("queryAllByAltText", text, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given alt text query.
    pub async fn find_by_alt_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByAltText", text).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given alt text query with options.
    pub async fn find_by_alt_text_with_options(
        &self,
        text: &str,
        options: &ByAltTextOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_alt_text_method("findByAltText", text, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given alt text query.
    pub async fn find_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByAltText", text).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given alt text query with options.
    pub async fn find_all_by_alt_text_with_options(
        &self,
        text: &str,
        options: &ByAltTextOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_alt_text_method("findAllByAltText", text, Some(options)).await?.elements()
    }

    // Title methods

    /// Returns an array of all matching elements for a query by title, throws an error if no elements match.
    pub async fn get_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByTitle", title).await?.elements()
    }

    /// Returns an array of all matching elements for a query by title with options, throws an error if no elements match.
    pub async fn get_all_by_title_with_options(
        &self,
        title: &str,
        options: &ByTitleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_title_method("getAllByTitle", title, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by title, returns None if no elements match.
    pub async fn query_by_title(&self, title: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByTitle", title).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by title with options, returns None if no elements match.
    pub async fn query_by_title_with_options(
        &self,
        title: &str,
        options: &ByTitleOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_title_method_with_filter("queryByTitle", title, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by title, returns empty array if no elements match.
    pub async fn query_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByTitle", title).await?.elements()
    }

    /// Returns an array of all matching elements for a query by title with options, returns empty array if no elements match.
    pub async fn query_all_by_title_with_options(
        &self,
        title: &str,
        options: &ByTitleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_title_method("queryAllByTitle", title, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given title query.
    pub async fn find_by_title(&self, title: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByTitle", title).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given title query with options.
    pub async fn find_by_title_with_options(
        &self,
        title: &str,
        options: &ByTitleOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_title_method("findByTitle", title, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given title query.
    pub async fn find_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByTitle", title).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given title query with options.
    pub async fn find_all_by_title_with_options(
        &self,
        title: &str,
        options: &ByTitleOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_title_method("findAllByTitle", title, Some(options)).await?.elements()
    }

    // Test ID methods

    /// Returns an array of all matching elements for a query by test ID, throws an error if no elements match.
    pub async fn get_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByTestId", test_id).await?.elements()
    }

    /// Returns the matching element for a query by test ID, returns None if no elements match.
    pub async fn query_by_test_id(&self, test_id: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements =
            self.execute_tl_method_with_filter("queryByTestId", test_id).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by test ID, returns empty array if no elements match.
    pub async fn query_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByTestId", test_id).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given test ID query.
    pub async fn find_by_test_id(&self, test_id: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByTestId", test_id).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given test ID query.
    pub async fn find_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByTestId", test_id).await?.elements()
    }

    /// Returns an array of all matching elements for a query by test ID with options, throws an error if no elements match.
    pub async fn get_all_by_test_id_with_options(
        &self,
        test_id: &str,
        options: &ByTestIdOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_test_id_method("getAllByTestId", test_id, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by test ID with options, returns None if no elements match.
    pub async fn query_by_test_id_with_options(
        &self,
        test_id: &str,
        options: &ByTestIdOptions,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .execute_tl_test_id_method_with_filter("queryByTestId", test_id, Some(options))
            .await?
            .elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by test ID with options, returns empty array if no elements match.
    pub async fn query_all_by_test_id_with_options(
        &self,
        test_id: &str,
        options: &ByTestIdOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_test_id_method("queryAllByTestId", test_id, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given test ID query with options.
    pub async fn find_by_test_id_with_options(
        &self,
        test_id: &str,
        options: &ByTestIdOptions,
    ) -> WebDriverResult<WebElement> {
        self.execute_tl_test_id_method("findByTestId", test_id, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given test ID query with options.
    pub async fn find_all_by_test_id_with_options(
        &self,
        test_id: &str,
        options: &ByTestIdOptions,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_test_id_method("findAllByTestId", test_id, Some(options)).await?.elements()
    }
}
