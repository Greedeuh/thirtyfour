mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_by_alt_text_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_alt_text_exact.html")
            .await?;

        // Test exact match - should find only "Logo" alt text, not "Company Logo Image"
        let element = screen.get(By::alt_text("Logo").exact(true)).await?;
        assert_id(&element, "alt-exact").await?;

        // Test get_all_by_alt_text_with_options
        let elements = screen.get_all(By::alt_text("Logo").exact(true)).await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "alt-exact").await?;

        // Test query_by_alt_text_with_options
        let result = screen.query(By::alt_text("Logo").exact(true)).await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "alt-exact").await?;

        // Test query_all_by_alt_text_with_options
        let query_elements = screen.query_all(By::alt_text("Logo").exact(true)).await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "alt-exact").await?;

        // Test find_by_alt_text_with_options
        let find_element = screen.find(By::alt_text("Logo").exact(true)).await?;
        assert_id(&find_element, "alt-exact").await?;

        // Test find_all_by_alt_text_with_options
        let find_elements = screen.find_all(By::alt_text("Logo").exact(true)).await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "alt-exact").await?;

        Ok(())
    })
}
