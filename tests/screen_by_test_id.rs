mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn get_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notification = screen.get(By::test_id("notification")).await?;

        assert_id(&notification, "notification").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notification = screen.query(By::test_id("notification")).await?;

        assert!(notification.is_some());
        assert_id(&notification.unwrap(), "notification").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notifications = screen.get_all(By::test_id("notification")).await?;

        assert_count(&notifications, 1)?;
        assert_id(&notifications[0], "notification").await?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notifications = screen.query_all(By::test_id("notification")).await?;

        assert_count(&notifications, 1)?;
        assert_id(&notifications[0], "notification").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notification = screen.find(By::test_id("notification")).await?;

        assert_id(&notification, "notification").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let notifications = screen.find_all(By::test_id("notification")).await?;

        assert_count(&notifications, 1)?;
        assert_id(&notifications[0], "notification").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::test_id("non-existent-test-id")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::test_id("non-existent-test-id")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::test_id("non-existent-test-id")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::test_id("non-existent-test-id")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn query_by_test_id_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.query(By::test_id("non-existent-test-id")).await?;

        assert_none(result)?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen
            .query_all(By::test_id("non-existent-test-id"))
            .await?;

        assert_count(&result, 0)?;

        Ok(())
    })
}
