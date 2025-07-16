mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let image = screen.get(By::alt_text("Some image")).await?;

        assert_id(&image, "some-image").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let image: Option<WebElement> = screen.query(By::alt_text("Some image")).await?;

        assert!(image.is_some());
        assert_id(&image.unwrap(), "some-image").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let images = screen.get_all(By::alt_text("Some image")).await?;

        assert_count(&images, 1)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let images = screen.query_all(By::alt_text("Some image")).await?;

        assert_count(&images, 1)?;
        assert_id(&images[0], "some-image").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let image = screen.find(By::alt_text("Some image")).await?;

        assert_id(&image, "some-image").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let images = screen.find_all(By::alt_text("Some image")).await?;

        assert_count(&images, 1)?;
        assert_id(&images[0], "some-image").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::alt_text("NonExistentAlt")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::alt_text("NonExistentAlt")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::alt_text("NonExistentAlt")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::alt_text("NonExistentAlt")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query(By::alt_text("NonExistentAlt")).await?;

        assert_none(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_alt_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query_all(By::alt_text("NonExistentAlt")).await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
