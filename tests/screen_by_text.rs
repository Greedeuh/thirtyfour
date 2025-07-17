mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let span = screen.get(By::text("some text to find")).await?;

        assert_eq!(span.id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn query_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let span = screen.query(By::text("some text to find")).await?;

        assert!(span.is_some());
        assert_eq!(span.unwrap().id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn query_by_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let button = screen.query(By::text("NonExistent")).await?;

        assert!(button.is_none());

        Ok(())
    })
}

#[rstest]
fn get_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let spans = screen.get_all(By::text("some text to find")).await?;

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn query_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let spans = screen.query_all(By::text("some text to find")).await?;

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn find_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let span = screen.find(By::text("some text to find")).await?;

        assert_eq!(span.id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn find_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let spans = screen.find_all(By::text("some text to find")).await?;

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn get_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::text("NonExistentText")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::text("NonExistentText")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::text("NonExistentText")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::text("NonExistentText")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_all_by_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query_all(By::text("NonExistentText")).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
