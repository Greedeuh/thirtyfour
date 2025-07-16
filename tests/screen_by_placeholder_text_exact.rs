mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_by_placeholder_text_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_placeholder_text_exact.html")
            .await?;

        // Test exact match - should find only "Username" placeholder, not "Enter your username"
        let element = screen
            .get(By::placeholder_text("Username").exact(true))
            .await?;
        assert_id(&element, "placeholder-exact").await?;

        let elements = screen
            .get_all(By::placeholder_text("Username").exact(true))
            .await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "placeholder-exact").await?;

        let result = screen
            .query(By::placeholder_text("Username").exact(true))
            .await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "placeholder-exact").await?;

        let query_elements = screen
            .query_all(By::placeholder_text("Username").exact(true))
            .await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "placeholder-exact").await?;

        let find_element = screen
            .find(By::placeholder_text("Username").exact(true))
            .await?;
        assert_id(&find_element, "placeholder-exact").await?;

        let find_elements = screen
            .find_all(By::placeholder_text("Username").exact(true))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "placeholder-exact").await?;

        Ok(())
    })
}
