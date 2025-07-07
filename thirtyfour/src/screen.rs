use std::fs;

use crate::{error::WebDriverResult, WebDriver, WebElement};

/// A struct representing a screen in the testing library
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver
}

impl Screen {
    /// Creates a new `Screen` and load the testing library script in the browser
    pub async fn load_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        // Load the testing library script in the browser
        let testing_library = fs::read_to_string("js/testing-library.js").unwrap();
        driver
            .execute(testing_library, vec![]).await?;

        Ok(Screen { driver })
    }

    /// Gets a single element by its role. Throws an error if none or multiple elements are found.
    pub async fn get_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.getByRole(document, '{}');", role), vec![])
            .await?.element()
    }

    /// Gets all elements by their role. Throws an error if none are found.
    pub async fn get_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.getAllByRole(document, '{}');", role), vec![])
            .await?.elements()
    }

    /// Queries a single element by its role. Returns None if not found.
    pub async fn query_by_role(&self, role: &str) -> WebDriverResult<Option<WebElement>> {
        let mut elements= self.driver
            .execute(format!("return [window.__TL__.queryByRole(document, '{}')].filter(n => n);", role), vec![])
            .await?.elements()?;

        if elements.is_empty() {
            return Ok(None);
        }
            
        Ok(Some(elements.remove(0)))
    }

    /// Queries all elements by their role
    pub async fn query_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.queryAllByRole(document, '{}');", role), vec![])
            .await?.elements()
    }

    /// Finds a single element by its role. Waits for the element to appear.
    pub async fn find_by_role(&self, role: &str) -> WebDriverResult<WebElement> {
        self.driver
            .execute(format!("return window.__TL__.findByRole(document, '{}');", role), vec![])
            .await?.element()
    }

    /// Finds all elements by their role. Waits for at least one element to appear.
    pub async fn find_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        self.driver
            .execute(format!("return window.__TL__.findAllByRole(document, '{}');", role), vec![])
            .await?.elements()
    }
}