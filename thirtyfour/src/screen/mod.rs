/// Role-based query options and utilities
pub mod role;

pub use role::*;

use std::fs;

use crate::{error::WebDriverResult, prelude::ScriptRet, WebDriver, WebElement};

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

        Ok(Screen { driver })
    }

    // Internal helper method for executing Testing Library methods
    async fn execute_tl_method(&self, method: &str, selector: &str) -> WebDriverResult<ScriptRet> {
        self.driver.execute(
            format!("return window.__TL__.{}(document, '{}');", method, selector),
            vec![],
        ).await
    }

    // Internal helper method for executing Testing Library methods with array wrapper
    async fn execute_tl_method_with_filter(&self, method: &str, selector: &str) -> WebDriverResult<ScriptRet> {
        self.driver.execute(
            format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, selector),
            vec![],
        ).await
    }

    // Internal helper method for executing Testing Library role methods with options
    async fn execute_tl_role_method(&self, method: &str, role: &str, options: Option<&ByRoleOptions>) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!("Failed to serialize role options: {}", e))
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
    async fn execute_tl_role_method_with_filter(&self, method: &str, role: &str, options: Option<&ByRoleOptions>) -> WebDriverResult<ScriptRet> {
        let script = match options {
            Some(opts) => {
                let options_json = opts.to_json_string().map_err(|e| {
                    crate::error::WebDriverError::Json(format!("Failed to serialize role options: {}", e))
                })?;
                
                format!("return [window.__TL__.{}(document, '{}', {})].filter(n => n);", method, role, options_json)
            }
            None => {
                format!("return [window.__TL__.{}(document, '{}')].filter(n => n);", method, role)
            }
        };
        
        self.driver.execute(script, vec![]).await
    }

    // Role-based methods
    /// Returns the matching element for a query by role, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_role_method("getByRole", role, None).await?.element()
    }

    /// Returns the matching element for a query by role with options, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<WebElement> {
        self.execute_tl_role_method("getByRole", role, Some(options)).await?.element()
    }

    /// Returns an array of all matching elements for a query by role, throws an error if no elements match.
    pub async fn get_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByRole", role).await?.elements()
    }

    /// Returns an array of all matching elements for a query by role with options, throws an error if no elements match.
    pub async fn get_all_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("getAllByRole", role, Some(options)).await?.elements()
    }

    /// Returns the matching element for a query by role, returns None if no elements match.
    pub async fn query_by_role(&self, role: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByRole", role).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns the matching element for a query by role with options, returns None if no elements match.
    pub async fn query_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_role_method_with_filter("queryByRole", role, Some(options)).await?.elements()?;
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
    pub async fn query_all_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("queryAllByRole", role, Some(options)).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given role query.
    pub async fn find_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByRole", role).await?.element()
    }

    /// Returns a promise which resolves when an element is found which matches the given role query with options.
    pub async fn find_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<WebElement> {
        self.execute_tl_role_method("findByRole", role, Some(options)).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given role query.
    pub async fn find_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByRole", role).await?.elements()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given role query with options.
    pub async fn find_all_by_role_with_options(&self, role: &str, options: &ByRoleOptions) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_role_method("findAllByRole", role, Some(options)).await?.elements()
    }

    // Text-based methods
    /// Returns the matching element for a query by text content, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByText", text).await?.element()
    }

    /// Returns an array of all matching elements for a query by text content, throws an error if no elements match.
    pub async fn get_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByText", text).await?.elements()
    }

    /// Returns the matching element for a query by text content, returns None if no elements match.
    pub async fn query_by_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by text content, returns empty array if no elements match.
    pub async fn query_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByText", text).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given text content query.
    pub async fn find_by_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByText", text).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given text content query.
    pub async fn find_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByText", text).await?.elements()
    }

    // Label text methods
    /// Returns the matching element for a query by label text, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_label_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByLabelText", text).await?.element()
    }

    /// Returns an array of all matching elements for a query by label text, throws an error if no elements match.
    pub async fn get_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByLabelText", text).await?.elements()
    }

    /// Returns the matching element for a query by label text, returns None if no elements match.
    pub async fn query_by_label_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByLabelText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by label text, returns empty array if no elements match.
    pub async fn query_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByLabelText", text).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given label text query.
    pub async fn find_by_label_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByLabelText", text).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given label text query.
    pub async fn find_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByLabelText", text).await?.elements()
    }

    // Placeholder text methods
    /// Returns the matching element for a query by placeholder text, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_placeholder_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByPlaceholderText", text).await?.element()
    }

    /// Returns an array of all matching elements for a query by placeholder text, throws an error if no elements match.
    pub async fn get_all_by_placeholder_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByPlaceholderText", text).await?.elements()
    }

    /// Returns the matching element for a query by placeholder text, returns None if no elements match.
    pub async fn query_by_placeholder_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByPlaceholderText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by placeholder text, returns empty array if no elements match.
    pub async fn query_all_by_placeholder_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByPlaceholderText", text).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given placeholder text query.
    pub async fn find_by_placeholder_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByPlaceholderText", text).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given placeholder text query.
    pub async fn find_all_by_placeholder_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByPlaceholderText", text).await?.elements()
    }

    // Display value methods
    /// Returns the matching element for a query by display value, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_display_value(&self, value: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByDisplayValue", value).await?.element()
    }

    /// Returns an array of all matching elements for a query by display value, throws an error if no elements match.
    pub async fn get_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByDisplayValue", value).await?.elements()
    }

    /// Returns the matching element for a query by display value, returns None if no elements match.
    pub async fn query_by_display_value(&self, value: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByDisplayValue", value).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by display value, returns empty array if no elements match.
    pub async fn query_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByDisplayValue", value).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given display value query.
    pub async fn find_by_display_value(&self, value: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByDisplayValue", value).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given display value query.
    pub async fn find_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByDisplayValue", value).await?.elements()
    }

    // Alt text methods
    /// Returns the matching element for a query by alt text, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_alt_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByAltText", text).await?.element()
    }

    /// Returns an array of all matching elements for a query by alt text, throws an error if no elements match.
    pub async fn get_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByAltText", text).await?.elements()
    }

    /// Returns the matching element for a query by alt text, returns None if no elements match.
    pub async fn query_by_alt_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByAltText", text).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by alt text, returns empty array if no elements match.
    pub async fn query_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByAltText", text).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given alt text query.
    pub async fn find_by_alt_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByAltText", text).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given alt text query.
    pub async fn find_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByAltText", text).await?.elements()
    }

    // Title methods
    /// Returns the matching element for a query by title, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_title(&self, title: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByTitle", title).await?.element()
    }

    /// Returns an array of all matching elements for a query by title, throws an error if no elements match.
    pub async fn get_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByTitle", title).await?.elements()
    }

    /// Returns the matching element for a query by title, returns None if no elements match.
    pub async fn query_by_title(&self, title: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByTitle", title).await?.elements()?;
        if elements.is_empty() {
            return Ok(None);
        }
        Ok(Some(elements.remove(0)))
    }

    /// Returns an array of all matching elements for a query by title, returns empty array if no elements match.
    pub async fn query_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("queryAllByTitle", title).await?.elements()
    }

    /// Returns a promise which resolves when an element is found which matches the given title query.
    pub async fn find_by_title(&self, title: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("findByTitle", title).await?.element()
    }

    /// Returns a promise which resolves to an array of elements when any elements are found which match the given title query.
    pub async fn find_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("findAllByTitle", title).await?.elements()
    }

    // Test ID methods
    /// Returns the matching element for a query by test ID, throws an error if no elements match or if more than one match is found.
    pub async fn get_by_test_id(&self, test_id: &str) -> WebDriverResult<WebElement> {
        self.execute_tl_method("getByTestId", test_id).await?.element()
    }

    /// Returns an array of all matching elements for a query by test ID, throws an error if no elements match.
    pub async fn get_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.execute_tl_method("getAllByTestId", test_id).await?.elements()
    }

    /// Returns the matching element for a query by test ID, returns None if no elements match.
    pub async fn query_by_test_id(&self, test_id: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self.execute_tl_method_with_filter("queryByTestId", test_id).await?.elements()?;
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
}