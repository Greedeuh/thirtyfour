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

    /// Queries all elements by their role
    pub async fn query_all_by_role(&self, role: &str) -> WebDriverResult<Vec<WebElement>> {
        // Execute the queryAllByRole function from the testing library
        self.driver
            .execute(format!("return window.__TL__.queryAllByRole(document, '{}');", role), vec![])
            .await?.elements()

    }
}