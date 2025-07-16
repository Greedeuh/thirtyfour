mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByPlaceholderTextOptions};

#[rstest]
fn test_by_placeholder_text_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_placeholder_text_exact.html").await?;

        let exact_options = ByPlaceholderTextOptions::new().exact(true);

        // Test exact match - should find only "Username" placeholder, not "Enter your username"
        let element = screen
            .get(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert_id(&element, "placeholder-exact").await?;

        // Test get_all_by_placeholder_text_with_options
        let elements = screen
            .get_all(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "placeholder-exact").await?;

        // Test query_by_placeholder_text_with_options
        let result = screen
            .query(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "placeholder-exact").await?;

        // Test query_all_by_placeholder_text_with_options
        let query_elements = screen
            .query_all(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "placeholder-exact").await?;

        // Test find_by_placeholder_text_with_options
        let find_element = screen
            .find(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert_id(&find_element, "placeholder-exact").await?;

        // Test find_all_by_placeholder_text_with_options
        let find_elements = screen
            .find_all(By::placeholder_text_with_options(
                "Username",
                exact_options.clone(),
            ))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "placeholder-exact").await?;

        Ok(())
    })
}
