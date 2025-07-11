mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::{By, ByTitleOptions, Screen};
use thirtyfour::support::block_on;

#[rstest]
fn test_by_title_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_title_exact_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let exact_options = ByTitleOptions::new().exact(true);

        // Test exact match - should find only "Help" title, not "Click for Help"
        let element =
            screen.get(By::title_with_options("Help".to_string(), exact_options.clone())).await?;
        assert_eq!(element.id().await?, Some("title-exact".to_string()));

        // Test get_all_by_title_with_options
        let elements =
            screen.get_all(By::title_with_options("Help", exact_options.clone())).await?;
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].id().await?, Some("title-exact".to_string()));

        // Test query_by_title_with_options
        let result = screen.query(By::title_with_options("Help", exact_options.clone())).await?;
        assert!(result.is_some());
        assert_eq!(result.unwrap().id().await?, Some("title-exact".to_string()));

        // Test query_all_by_title_with_options
        let query_elements =
            screen.query_all(By::title_with_options("Help", exact_options.clone())).await?;
        assert_eq!(query_elements.len(), 1);
        assert_eq!(query_elements[0].id().await?, Some("title-exact".to_string()));

        // Test find_by_title_with_options
        let find_element =
            screen.find(By::title_with_options("Help", exact_options.clone())).await?;
        assert_eq!(find_element.id().await?, Some("title-exact".to_string()));

        // Test find_all_by_title_with_options
        let find_elements =
            screen.find_all(By::title_with_options("Help", exact_options.clone())).await?;
        assert_eq!(find_elements.len(), 1);
        assert_eq!(find_elements[0].id().await?, Some("title-exact".to_string()));

        Ok(())
    })
}
