mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::Screen;
use thirtyfour::support::block_on;

#[rstest]
fn get_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notification = screen.get_by_test_id("notification").await?;

        assert_eq!(notification.id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn query_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notification = screen.query_by_test_id("notification").await?;

        assert!(notification.is_some());
        assert_eq!(notification.unwrap().id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn get_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notifications = screen.get_all_by_test_id("notification").await?;

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notifications = screen.query_all_by_test_id("notification").await?;

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn find_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notification = screen.find_by_test_id("notification").await?;

        assert_eq!(notification.id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn find_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notifications = screen.find_all_by_test_id("notification").await?;

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn get_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_test_id("non-existent-test-id").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_all_by_test_id("non-existent-test-id").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_by_test_id("non-existent-test-id").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_all_by_test_id("non-existent-test-id").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_test_id_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_test_id("non-existent-test-id").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_all_by_test_id("non-existent-test-id").await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
