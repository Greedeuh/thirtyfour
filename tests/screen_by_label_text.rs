mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_by_label_text_success(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        let element = screen.get(By::label_text("User name:")).await?;
        assert_id(&element, "user-name").await?;

        let elements = screen.get_all(By::label_text("User name:")).await?;
        assert_count(&elements, 1)?;
        assert_id(&elements[0], "user-name").await?;

        let result = screen.query(By::label_text("User name:")).await?;
        assert!(result.is_some());
        assert_id(&result.unwrap(), "user-name").await?;

        let query_elements = screen.query_all(By::label_text("User name:")).await?;
        assert_count(&query_elements, 1)?;
        assert_id(&query_elements[0], "user-name").await?;

        let find_element = screen.find(By::label_text("User name:")).await?;
        assert_id(&find_element, "user-name").await?;

        let find_elements = screen.find_all(By::label_text("User name:")).await?;
        assert_count(&find_elements, 1)?;
        assert_id(&find_elements[0], "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn test_by_label_text_failure(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        let get_result = screen.get(By::label_text("NonExistentLabel")).await;
        assert_error(get_result)?;

        let get_all_result = screen.get_all(By::label_text("NonExistentLabel")).await;
        assert_error(get_all_result)?;

        let find_result = screen.find(By::label_text("NonExistentLabel")).await;
        assert_error(find_result)?;

        let find_all_result = screen.find_all(By::label_text("NonExistentLabel")).await;
        assert_error(find_all_result)?;

        let query_result = screen.query(By::label_text("NonExistentLabel")).await?;
        assert_none(query_result)?;

        let query_all_result = screen.query_all(By::label_text("NonExistentLabel")).await?;
        assert_count(&query_all_result, 0)?;

        Ok(())
    })
}
