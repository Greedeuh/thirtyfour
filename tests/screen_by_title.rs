mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let save_btn = screen.get(By::title("Some title")).await?;

        assert_eq!(save_btn.id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn query_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let delete_btn = screen.query(By::title("Some title")).await?;

        assert!(delete_btn.is_some());
        assert_eq!(delete_btn.unwrap().id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn get_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let buttons = screen.get_all(By::title("Some title")).await?;

        assert_eq!(buttons.len(), 1);

        Ok(())
    })
}

#[rstest]
fn query_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let buttons = screen.query_all(By::title("Some title")).await?;

        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn find_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let button = screen.find(By::title("Some title")).await?;

        assert_eq!(button.id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn find_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let buttons = screen.find_all(By::title("Some title")).await?;

        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn get_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::title("NonExistentTitle")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::title("NonExistentTitle")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::title("NonExistentTitle")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::title("NonExistentTitle")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_title_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query(By::title("NonExistentTitle")).await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_title_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query_all(By::title("NonExistentTitle")).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
