mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn get_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let image = screen.get(By::alt_text("Some image")).await?;

        assert_eq!(image.id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let image: Option<WebElement> = screen.query(By::alt_text("Some image")).await?;

        assert!(image.is_some());
        assert_eq!(image.unwrap().id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn get_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let images = screen.get_all(By::alt_text("Some image")).await?;

        assert_eq!(images.len(), 1);

        Ok(())
    })
}

#[rstest]
fn query_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let images = screen.query_all(By::alt_text("Some image")).await?;

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn find_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let image = screen.find(By::alt_text("Some image")).await?;

        assert_eq!(image.id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn find_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let images = screen.find_all(By::alt_text("Some image")).await?;

        assert_eq!(images.len(), 1);
        assert_eq!(images[0].id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn get_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::alt_text("NonExistentAlt")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::alt_text("NonExistentAlt")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::alt_text("NonExistentAlt")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::alt_text("NonExistentAlt")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query(By::alt_text("NonExistentAlt")).await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_all_by_alt_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.query_all(By::alt_text("NonExistentAlt")).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
