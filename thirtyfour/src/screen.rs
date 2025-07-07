use std::fs;

use crate::{error::WebDriverResult, WebDriver, WebElement};

/// A struct representing a screen in the testing library
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver,
}

impl Screen {
    /// Creates a new `Screen` and load the testing library script in the browser
    pub async fn load_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        // Load the testing library script in the browser
        let testing_library = fs::read_to_string("js/testing-library.js").unwrap();
        driver.execute(testing_library, vec![]).await?;

        Ok(Screen {
            driver,
        })
    }

    /// Gets a single element by its role. Throws an error if none or multiple elements are found.
    pub async fn get_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByRole(document, '{}');", role), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their role. Throws an error if none are found.
    pub async fn get_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.getAllByRole(document, '{}');", role), vec![])
            .await?
            .elements()
    }

    /// Queries a single element by its role. Returns None if not found.
    pub async fn query_by_role(&self, role: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!("return [window.__TL__.queryByRole(document, '{}')].filter(n => n);", role),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their role
    pub async fn query_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.queryAllByRole(document, '{}');", role), vec![])
            .await?
            .elements()
    }

    /// Finds a single element by its role. Waits for the element to appear.
    pub async fn find_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByRole(document, '{}');", role), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their role. Waits for at least one element to appear.
    pub async fn find_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.findAllByRole(document, '{}');", role), vec![])
            .await?
            .elements()
    }

    /// Gets a single element by its text content. Throws an error if none or multiple elements are found.
    pub async fn get_by_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their text content. Throws an error if none are found.
    pub async fn get_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.getAllByText(document, '{}');", text), vec![])
            .await?
            .elements()
    }

    /// Queries a single element by its text content. Returns None if not found.
    pub async fn query_by_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!("return [window.__TL__.queryByText(document, '{}')].filter(n => n);", text),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their text content
    pub async fn query_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.queryAllByText(document, '{}');", text), vec![])
            .await?
            .elements()
    }

    /// Finds a single element by its text content. Waits for the element to appear.
    pub async fn find_by_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their text content. Waits for at least one element to appear.
    pub async fn find_all_by_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.findAllByText(document, '{}');", text), vec![])
            .await?
            .elements()
    }

    /// Gets a single element by its label text. Throws an error if none or multiple elements are found.
    pub async fn get_by_label_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByLabelText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their label text. Throws an error if none are found.
    pub async fn get_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.getAllByLabelText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Queries a single element by its label text. Returns None if not found.
    pub async fn query_by_label_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByLabelText(document, '{}')].filter(n => n);",
                    text
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their label text
    pub async fn query_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByLabelText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its label text. Waits for the element to appear.
    pub async fn find_by_label_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByLabelText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their label text. Waits for at least one element to appear.
    pub async fn find_all_by_label_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.findAllByLabelText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Gets a single element by its placeholder text. Throws an error if none or multiple elements are found.
    pub async fn get_by_placeholder_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(
                format!("return window.__TL__.getByPlaceholderText(document, '{}');", text),
                vec![],
            )
            .await?
            .element()
    }

    /// Gets all elements by their placeholder text. Throws an error if none are found.
    pub async fn get_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.getAllByPlaceholderText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Queries a single element by its placeholder text. Returns None if not found.
    pub async fn query_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByPlaceholderText(document, '{}')].filter(n => n);",
                    text
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their placeholder text
    pub async fn query_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByPlaceholderText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its placeholder text. Waits for the element to appear.
    pub async fn find_by_placeholder_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(
                format!("return window.__TL__.findByPlaceholderText(document, '{}');", text),
                vec![],
            )
            .await?
            .element()
    }

    /// Finds all elements by their placeholder text. Waits for at least one element to appear.
    pub async fn find_all_by_placeholder_text(
        &self,
        text: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.findAllByPlaceholderText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Gets a single element by its display value. Throws an error if none or multiple elements are found.
    pub async fn get_by_display_value(&self, value: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(
                format!("return window.__TL__.getByDisplayValue(document, '{}');", value),
                vec![],
            )
            .await?
            .element()
    }

    /// Gets all elements by their display value. Throws an error if none are found.
    pub async fn get_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.getAllByDisplayValue(document, '{}');", value),
                vec![],
            )
            .await?
            .elements()
    }

    /// Queries a single element by its display value. Returns None if not found.
    pub async fn query_by_display_value(&self, value: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByDisplayValue(document, '{}')].filter(n => n);",
                    value
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their display value
    pub async fn query_all_by_display_value(
        &self,
        value: &str,
    ) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByDisplayValue(document, '{}');", value),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its display value. Waits for the element to appear.
    pub async fn find_by_display_value(&self, value: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(
                format!("return window.__TL__.findByDisplayValue(document, '{}');", value),
                vec![],
            )
            .await?
            .element()
    }

    /// Finds all elements by their display value. Waits for at least one element to appear.
    pub async fn find_all_by_display_value(&self, value: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.findAllByDisplayValue(document, '{}');", value),
                vec![],
            )
            .await?
            .elements()
    }

    /// Gets a single element by its alt text. Throws an error if none or multiple elements are found.
    pub async fn get_by_alt_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByAltText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their alt text. Throws an error if none are found.
    pub async fn get_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.getAllByAltText(document, '{}');", text), vec![])
            .await?
            .elements()
    }

    /// Queries a single element by its alt text. Returns None if not found.
    pub async fn query_by_alt_text(&self, text: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByAltText(document, '{}')].filter(n => n);",
                    text
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their alt text
    pub async fn query_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByAltText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its alt text. Waits for the element to appear.
    pub async fn find_by_alt_text(&self, text: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByAltText(document, '{}');", text), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their alt text. Waits for at least one element to appear.
    pub async fn find_all_by_alt_text(&self, text: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.findAllByAltText(document, '{}');", text),
                vec![],
            )
            .await?
            .elements()
    }

    /// Gets a single element by its title. Throws an error if none or multiple elements are found.
    pub async fn get_by_title(&self, title: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByTitle(document, '{}');", title), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their title. Throws an error if none are found.
    pub async fn get_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.getAllByTitle(document, '{}');", title), vec![])
            .await?
            .elements()
    }

    /// Queries a single element by its title. Returns None if not found.
    pub async fn query_by_title(&self, title: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByTitle(document, '{}')].filter(n => n);",
                    title
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their title
    pub async fn query_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByTitle(document, '{}');", title),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its title. Waits for the element to appear.
    pub async fn find_by_title(&self, title: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByTitle(document, '{}');", title), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their title. Waits for at least one element to appear.
    pub async fn find_all_by_title(&self, title: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.findAllByTitle(document, '{}');", title), vec![])
            .await?
            .elements()
    }

    /// Gets a single element by its test id. Throws an error if none or multiple elements are found.
    pub async fn get_by_test_id(&self, test_id: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByTestId(document, '{}');", test_id), vec![])
            .await?
            .element()
    }

    /// Gets all elements by their test id. Throws an error if none are found.
    pub async fn get_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.getAllByTestId(document, '{}');", test_id),
                vec![],
            )
            .await?
            .elements()
    }

    /// Queries a single element by its test id. Returns None if not found.
    pub async fn query_by_test_id(&self, test_id: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements = self
            .driver
            .execute(
                format!(
                    "return [window.__TL__.queryByTestId(document, '{}')].filter(n => n);",
                    test_id
                ),
                vec![],
            )
            .await?
            .elements()?;

        if elements.is_empty() {
            return Ok(None);
        }

        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their test id
    pub async fn query_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.queryAllByTestId(document, '{}');", test_id),
                vec![],
            )
            .await?
            .elements()
    }

    /// Finds a single element by its test id. Waits for the element to appear.
    pub async fn find_by_test_id(&self, test_id: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByTestId(document, '{}');", test_id), vec![])
            .await?
            .element()
    }

    /// Finds all elements by their test id. Waits for at least one element to appear.
    pub async fn find_all_by_test_id(&self, test_id: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(
                format!("return window.__TL__.findAllByTestId(document, '{}');", test_id),
                vec![],
            )
            .await?
            .elements()
    }
}
