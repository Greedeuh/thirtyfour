mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let option = screen.get(By::display_value("Red")).await?;

        assert_eq!(option.value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn query_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let option = screen.query(By::display_value("Red")).await?;

        assert!(option.is_some());
        assert_eq!(option.unwrap().value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn get_all_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let options = screen.get_all(By::display_value("Red")).await?;

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn query_all_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let options = screen.query_all(By::display_value("Red")).await?;

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn find_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let option = screen.find(By::display_value("Red")).await?;

        assert_eq!(option.value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn find_all_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let options = screen.find_all(By::display_value("Red")).await?;

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn get_by_display_value_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::display_value("NonExistentValue")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_display_value_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::display_value("NonExistentValue")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_display_value_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::display_value("NonExistentValue")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_display_value_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::display_value("NonExistentValue")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_display_value_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query(By::display_value("NonExistentValue")).await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_display_value_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .query_all(By::display_value("NonExistentValue"))
            .await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
