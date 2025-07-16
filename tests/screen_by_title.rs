mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let save_btn = screen.get(By::title("Some title")).await?;

        assert_id(&save_btn, "some-title").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let delete_btn = screen.query(By::title("Some title")).await?;

        assert!(delete_btn.is_some());
        assert_id(&delete_btn.unwrap(), "some-title").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let buttons = screen.get_all(By::title("Some title")).await?;

        assert_count(&buttons, 1)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let buttons = screen.query_all(By::title("Some title")).await?;

        assert_count(&buttons, 1)?;
        assert_id(&buttons[0], "some-title").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let button = screen.find(By::title("Some title")).await?;

        assert_id(&button, "some-title").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let buttons = screen.find_all(By::title("Some title")).await?;

        assert_count(&buttons, 1)?;
        assert_id(&buttons[0], "some-title").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::title("NonExistentTitle")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::title("NonExistentTitle")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::title("NonExistentTitle")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::title("NonExistentTitle")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_by_title_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query(By::title("NonExistentTitle")).await?;

        assert_none(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_title_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query_all(By::title("NonExistentTitle")).await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
