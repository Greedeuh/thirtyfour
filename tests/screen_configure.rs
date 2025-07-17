mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{configure, By, Screen};

#[rstest]
fn test_configure_default_hidden(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = screen_configure_page_url();
        c.goto(&url).await?;

        // Create screen with configure options to include hidden elements by default
        let configure_options = configure::Options::new().with_default_hidden(true);

        let screen =
            Screen::build_with_testing_library(c.clone()).await?.configure(configure_options);

        // Try to find hidden button using just the role selector
        // This should work because we configured the screen to include hidden elements by default
        screen.get(By::text("Hidden Button")).await?;

        Ok(())
    })
}

#[rstest]
fn test_configure_default_ignore(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = screen_configure_page_url();
        c.goto(&url).await?;

        // Create screen with configure options to change what elements are ignored
        let configure_options = configure::Options::new().with_default_ignore("div");

        let screen =
            Screen::build_with_testing_library(c.clone()).await?.configure(configure_options);

        // Try to find text in the paragraph
        // This should work because we configured to ignore divs instead of script/style
        let paragraph = screen.get(By::text("This is a paragraph")).await?;
        assert_eq!(paragraph.text().await?, "This is a paragraph");

        Ok(())
    })
}

#[rstest]
fn test_configure_test_id_attribute(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = screen_configure_page_url();
        c.goto(&url).await?;

        // Create screen with custom test ID attribute
        let configure_options =
            configure::Options::new().with_test_id_attribute("my-custom-testid");

        let screen =
            Screen::build_with_testing_library(c.clone()).await?.configure(configure_options);

        // Try to find element using the custom test ID attribute
        let custom_button = screen.get(By::test_id("custom-test-id")).await?;
        assert_eq!(custom_button.text().await?, "Custom TestId Button");

        Ok(())
    })
}
