use crate::{error::WebDriverResult, WebDriver};

/// A struct representing a screen in the testing library
#[derive(Debug, Clone)]
pub struct Screen {
    driver: WebDriver
}

impl Screen {
    /// Creates a new `Screen` and load the testing library on through the provided `WebDriver`
    pub async fn load_with_testing_library(driver: WebDriver) -> WebDriverResult<Self> {
        driver
            .execute("", vec![]).await?;
        Ok(Screen { driver })
    }
}