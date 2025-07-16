mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_by_display_value_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_display_value_exact.html")
            .await?;

        // Test exact match - should find only "Submit" value, not "Submit Form"
        let element = screen.get(By::display_value("Submit").exact(true)).await?;
        assert_id(&element, "value-exact").await?;

        let elements = screen
            .get_all(By::display_value("Submit").exact(true))
            .await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "value-exact").await?;

        let result = screen
            .query(By::display_value("Submit").exact(true))
            .await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "value-exact").await?;

        let query_elements = screen
            .query_all(By::display_value("Submit").exact(true))
            .await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "value-exact").await?;

        let find_element = screen.find(By::display_value("Submit").exact(true)).await?;
        assert_id(&find_element, "value-exact").await?;

        let find_elements = screen
            .find_all(By::display_value("Submit").exact(true))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "value-exact").await?;

        Ok(())
    })
}
