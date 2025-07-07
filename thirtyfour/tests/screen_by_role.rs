mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::Screen;
use thirtyfour::support::block_on;

#[rstest]
fn query_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltips = screen.query_all_by_role("tooltip").await?;

        assert_eq!(tooltips.len(), 1);
        assert_eq!(tooltips[0].text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn query_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;

        let tooltip = screen.query_by_role("tooltip").await?;
        assert!(tooltip.is_some());
        assert_eq!(tooltip.unwrap().text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn get_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltip = screen.get_by_role("tooltip").await?;

        assert_eq!(tooltip.text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn get_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltips = screen.get_all_by_role("tooltip").await?;

        assert_eq!(tooltips.len(), 1);
        assert_eq!(tooltips[0].text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn find_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltips = screen.find_all_by_role("tooltip").await?;

        assert_eq!(tooltips.len(), 1);
        assert_eq!(tooltips[0].text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn find_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltip = screen.find_by_role("tooltip").await?;

        assert_eq!(tooltip.text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn query_all_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let items = screen.query_all_by_role("item").await?;

        assert_eq!(items.len(), 0);

        Ok(())
    })
}

#[rstest]
fn query_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let item = screen.query_by_role("item").await?;

        assert!(item.is_none());

        Ok(())
    })
}

#[rstest]
fn get_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_all_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_all_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}