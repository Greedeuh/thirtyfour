mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let span = screen.get(By::text("some text to find")).await?;

        assert_id(&span, "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let span = screen.query(By::text("some text to find")).await?;

        assert!(span.is_some());
        assert_id(&span.unwrap(), "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let button = screen.query(By::text("NonExistent")).await?;

        assert_none(button)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let spans = screen.get_all(By::text("some text to find")).await?;

        assert_count(&spans, 1)?;
        assert_id(&spans[0], "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let spans = screen.query_all(By::text("some text to find")).await?;

        assert_count(&spans, 1)?;
        assert_id(&spans[0], "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let span = screen.find(By::text("some text to find")).await?;

        assert_id(&span, "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let spans = screen.find_all(By::text("some text to find")).await?;

        assert_count(&spans, 1)?;
        assert_id(&spans[0], "text-to-find").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::text("NonExistentText")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::text("NonExistentText")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::text("NonExistentText")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::text("NonExistentText")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query_all(By::text("NonExistentText")).await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
