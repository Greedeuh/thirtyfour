mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, TextMatch};

#[rstest]
fn test_by_role_success(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        let element = screen.get(By::role("tooltip")).await?;
        assert_text(&element, "Tooltip text").await?;

        let elements = screen.get_all(By::role("tooltip")).await?;
        assert_count(&elements, 1)?;
        assert_text(&elements[0], "Tooltip text").await?;

        let result = screen.query(By::role("tooltip")).await?;
        assert!(result.is_some());
        assert_text(&result.unwrap(), "Tooltip text").await?;

        let query_elements = screen.query_all(By::role("tooltip")).await?;
        assert_count(&query_elements, 1)?;
        assert_text(&query_elements[0], "Tooltip text").await?;

        let find_element = screen.find(By::role("tooltip")).await?;
        assert_text(&find_element, "Tooltip text").await?;

        let find_elements = screen.find_all(By::role("tooltip")).await?;
        assert_count(&find_elements, 1)?;
        assert_text(&find_elements[0], "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn test_by_role_failure(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        let get_result = screen.get(By::role("item")).await;
        assert_error(get_result)?;

        let get_all_result = screen.get_all(By::role("item")).await;
        assert_error(get_all_result)?;

        let find_result = screen.find(By::role("item")).await;
        assert_error(find_result)?;

        let find_all_result = screen.find_all(By::role("item")).await;
        assert_error(find_all_result)?;

        let query_result = screen.query(By::role("item")).await?;
        assert_none(query_result)?;

        let query_all_result = screen.query_all(By::role("item")).await?;
        assert_count(&query_all_result, 0)?;

        Ok(())
    })
}