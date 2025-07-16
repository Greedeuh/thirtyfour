mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByTextOptions};

#[rstest]
fn test_by_text_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_text_exact.html").await?;

        let exact_options = ByTextOptions::new().exact(true);

        // Test exact match - should find only "Login" text, not "Please Login Here"
        let element = screen
            .get(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert_id(&element, "text-exact").await?;

        // Test get_all_by_text_with_options
        let elements = screen
            .get_all(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "text-exact").await?;

        // Test query_by_text_with_options
        let result = screen
            .query(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "text-exact").await?;

        // Test query_all_by_text_with_options
        let query_elements = screen
            .query_all(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "text-exact").await?;

        // Test find_by_text_with_options
        let find_element = screen
            .find(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert_id(&find_element, "text-exact").await?;

        // Test find_all_by_text_with_options
        let find_elements = screen
            .find_all(By::text_with_options("Login", exact_options.clone()))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "text-exact").await?;

        Ok(())
    })
}
