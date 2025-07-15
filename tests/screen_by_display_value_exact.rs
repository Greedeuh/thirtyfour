mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByDisplayValueOptions, Screen};

#[rstest]
fn test_by_display_value_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_display_value_exact_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let exact_options = ByDisplayValueOptions::new().exact(true);

        // Test exact match - should find only "Submit" value, not "Submit Form"
        let element = screen
            .get(By::display_value_with_options(
                "Submit".to_string(),
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(element.id().await?, Some("value-exact".to_string()));

        // Test get_all_by_display_value_with_options
        let elements = screen
            .get_all(By::display_value_with_options(
                "Submit",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].id().await?, Some("value-exact".to_string()));

        // Test query_by_display_value_with_options
        let result = screen
            .query(By::display_value_with_options(
                "Submit",
                exact_options.clone(),
            ))
            .await?;
        assert!(result.is_some());
        assert_eq!(result.unwrap().id().await?, Some("value-exact".to_string()));

        // Test query_all_by_display_value_with_options
        let query_elements = screen
            .query_all(By::display_value_with_options(
                "Submit",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(query_elements.len(), 1);
        assert_eq!(
            query_elements[0].id().await?,
            Some("value-exact".to_string())
        );

        // Test find_by_display_value_with_options
        let find_element = screen
            .find(By::display_value_with_options(
                "Submit",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(find_element.id().await?, Some("value-exact".to_string()));

        // Test find_all_by_display_value_with_options
        let find_elements = screen
            .find_all(By::display_value_with_options(
                "Submit",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(find_elements.len(), 1);
        assert_eq!(
            find_elements[0].id().await?,
            Some("value-exact".to_string())
        );

        Ok(())
    })
}
