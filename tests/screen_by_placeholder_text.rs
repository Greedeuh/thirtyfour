mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let email_input = screen.get(By::placeholder_text("jean@email.fr")).await?;

        assert_eq!(email_input.id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let email_input = screen.query(By::placeholder_text("jean@email.fr")).await?;

        assert!(email_input.is_some());
        assert_eq!(email_input.unwrap().id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn get_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen
            .get_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn query_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen
            .query_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn find_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let input = screen.find(By::placeholder_text("jean@email.fr")).await?;

        assert_eq!(input.id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn find_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let inputs = screen
            .find_all(By::placeholder_text("jean@email.fr"))
            .await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn get_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .get(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .get_all(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .find(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .find_all(By::placeholder_text("NonExistentPlaceholder"))
            .await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .query(By::placeholder_text("NonExistentPlaceholder"))
            .await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_placeholder_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen
            .query_all(By::placeholder_text("NonExistentPlaceholder"))
            .await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
