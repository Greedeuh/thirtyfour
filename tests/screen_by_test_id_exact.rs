mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_by_test_id_exact_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_test_id_exact.html")
            .await?;

        // Test exact match - should find only "save" test ID, not "save-button"
        let element = screen.get(By::test_id("save").exact(true)).await?;
        assert_id(&element, "testid-exact").await?;

        // Test get_all_by_test_id_with_options
        let elements = screen.get_all(By::test_id("save").exact(true)).await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "testid-exact").await?;

        // Test query_by_test_id_with_options
        let result = screen.query(By::test_id("save").exact(true)).await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "testid-exact").await?;

        // Test query_all_by_test_id_with_options
        let query_elements = screen.query_all(By::test_id("save").exact(true)).await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "testid-exact").await?;

        // Test find_by_test_id_with_options
        let find_element = screen.find(By::test_id("save").exact(true)).await?;
        assert_id(&find_element, "testid-exact").await?;

        // Test find_all_by_test_id_with_options
        let find_elements = screen.find_all(By::test_id("save").exact(true)).await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "testid-exact").await?;

        Ok(())
    })
}
