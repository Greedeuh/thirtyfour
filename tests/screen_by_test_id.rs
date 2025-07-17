mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notification = screen.get(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notification = screen.query(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notifications = screen.get_all(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notifications = screen.query_all(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notification = screen.find(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let notifications = screen.find_all(By::test_id("notification")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::test_id("non-existent-test-id")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::test_id("non-existent-test-id")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::test_id("non-existent-test-id")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::test_id("non-existent-test-id")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query(By::test_id("non-existent-test-id")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query_all(By::test_id("non-existent-test-id")).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
