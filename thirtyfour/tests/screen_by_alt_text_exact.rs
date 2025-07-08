mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::{ByAltTextOptions, Screen};
use thirtyfour::support::block_on;

#[rstest]
fn test_by_alt_text_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_alt_text_exact_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;

        let exact_options = ByAltTextOptions::new().exact(true);

        // Test exact match - should find only "Logo" alt text, not "Company Logo Image"
        let element = screen.get_by_alt_text_with_options("Logo", &exact_options).await?;
        assert_eq!(element.id().await?, Some("alt-exact".to_string()));

        // Test get_all_by_alt_text_with_options
        let elements = screen.get_all_by_alt_text_with_options("Logo", &exact_options).await?;
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].id().await?, Some("alt-exact".to_string()));

        // Test query_by_alt_text_with_options
        let result = screen.query_by_alt_text_with_options("Logo", &exact_options).await?;
        assert!(result.is_some());
        assert_eq!(result.unwrap().id().await?, Some("alt-exact".to_string()));

        // Test query_all_by_alt_text_with_options
        let query_elements = screen.query_all_by_alt_text_with_options("Logo", &exact_options).await?;
        assert_eq!(query_elements.len(), 1);
        assert_eq!(query_elements[0].id().await?, Some("alt-exact".to_string()));

        // Test find_by_alt_text_with_options
        let find_element = screen.find_by_alt_text_with_options("Logo", &exact_options).await?;
        assert_eq!(find_element.id().await?, Some("alt-exact".to_string()));

        // Test find_all_by_alt_text_with_options
        let find_elements = screen.find_all_by_alt_text_with_options("Logo", &exact_options).await?;
        assert_eq!(find_elements.len(), 1);
        assert_eq!(find_elements[0].id().await?, Some("alt-exact".to_string()));

        Ok(())
    })
}