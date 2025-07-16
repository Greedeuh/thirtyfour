mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByTitleOptions};

#[rstest]
fn test_by_title_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_title_exact.html").await?;

        let exact_options = ByTitleOptions::new().exact(true);

        // Test exact match - should find only "Help" title, not "Click for Help"
        let element = screen
            .get(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert_id(&element, "title-exact").await?;

        // Test get_all_by_title_with_options
        let elements = screen
            .get_all(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "title-exact").await?;

        // Test query_by_title_with_options
        let result = screen
            .query(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "title-exact").await?;

        // Test query_all_by_title_with_options
        let query_elements = screen
            .query_all(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "title-exact").await?;

        // Test find_by_title_with_options
        let find_element = screen
            .find(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert_id(&find_element, "title-exact").await?;

        // Test find_all_by_title_with_options
        let find_elements = screen
            .find_all(By::title_with_options("Help", exact_options.clone()))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "title-exact").await?;

        Ok(())
    })
}
