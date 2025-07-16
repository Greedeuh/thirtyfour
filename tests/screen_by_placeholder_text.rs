mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let email_input = screen.get(By::placeholder_text("jean@email.fr")).await?;

        assert_id(&email_input, "email").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let email_input = screen.query(By::placeholder_text("jean@email.fr")).await?;

        assert!(email_input.is_some());
        assert_id(&email_input.unwrap(), "email").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen
            .get_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "email").await?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen
            .query_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "email").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let input = screen.find(By::placeholder_text("jean@email.fr")).await?;

        assert_id(&input, "email").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen
            .find_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "email").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .get(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .get_all(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .find(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .find_all(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .query(By::placeholder_text("NonExistentPlaceholder"))
            .await?;

        assert_none(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_placeholder_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .query_all(By::placeholder_text("NonExistentPlaceholder"))
            .await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
