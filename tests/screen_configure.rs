mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{configure, By};

#[rstest]
fn test_configure_default_hidden(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("screen_configure.html").await?;

        // Create screen with configure options to include hidden elements by default
        let configure_options = configure::Options::new().with_default_hidden(true);

        let screen = screen.configure(configure_options);

        // Try to find hidden button using just the role selector
        // This should work because we configured the screen to include hidden elements by default
        screen.get(By::text("Hidden Button")).await?;

        Ok(())
    })
}

#[rstest]
fn test_configure_default_ignore(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("screen_configure.html").await?;

        // Create screen with configure options to change what elements are ignored
        let configure_options = configure::Options::new().with_default_ignore("div");

        let screen = screen.configure(configure_options);

        // Try to find text in the paragraph
        // This should work because we configured to ignore divs instead of script/style
        let paragraph = screen.get(By::text("This is a paragraph")).await?;
        assert_text(&paragraph, "This is a paragraph").await?;

        Ok(())
    })
}

#[rstest]
fn test_configure_test_id_attribute(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("screen_configure.html").await?;

        // Create screen with custom test ID attribute
        let configure_options =
            configure::Options::new().with_test_id_attribute("my-custom-testid");

        let screen = screen.configure(configure_options);

        // Try to find element using the custom test ID attribute
        let custom_button = screen.get(By::test_id("custom-test-id")).await?;
        assert_text(&custom_button, "Custom TestId Button").await?;

        Ok(())
    })
}
