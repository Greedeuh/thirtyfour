mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let input = screen.get(By::label_text("User name:")).await?;

        assert_id(&input, "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen.get_all(By::label_text("User name:")).await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let password = screen.query(By::label_text("User name:")).await?;

        assert!(password.is_some());
        assert_id(&password.unwrap(), "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen.query_all(By::label_text("User name:")).await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let input = screen.find(By::label_text("User name:")).await?;

        assert_id(&input, "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let inputs = screen.find_all(By::label_text("User name:")).await?;

        assert_count(&inputs, 1)?;
        assert_id(&inputs[0], "user-name").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::label_text("NonExistentLabel")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::label_text("NonExistentLabel")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::label_text("NonExistentLabel")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::label_text("NonExistentLabel")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_by_label_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query(By::label_text("NonExistentLabel")).await?;

        assert_none(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_label_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query_all(By::label_text("NonExistentLabel")).await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
