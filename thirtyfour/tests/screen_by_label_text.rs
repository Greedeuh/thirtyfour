mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::{Screen, Selector};
use thirtyfour::support::block_on;

#[rstest]
fn get_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let input = screen.get(Selector::label_text("User name:")).await?;

        assert_eq!(input.id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn get_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen.get_all(Selector::label_text("User name:")).await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn query_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let password = screen.query(Selector::label_text("User name:")).await?;

        assert!(password.is_some());
        assert_eq!(password.unwrap().id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn query_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen.query_all(Selector::label_text("User name:")).await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn find_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let input = screen.find(Selector::label_text("User name:")).await?;

        assert_eq!(input.id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn find_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen.find_all(Selector::label_text("User name:")).await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn get_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(Selector::label_text("NonExistentLabel")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(Selector::label_text("NonExistentLabel")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(Selector::label_text("NonExistentLabel")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(Selector::label_text("NonExistentLabel")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_label_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query(Selector::label_text("NonExistentLabel")).await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_label_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query_all(Selector::label_text("NonExistentLabel")).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
