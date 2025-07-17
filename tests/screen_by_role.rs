mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByRoleOptions, Screen, TextMatch};

#[rstest]
fn query_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let tooltips = screen.query_all(By::role("tooltip")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let tooltip = screen.query(By::role("tooltip")).await?;
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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let tooltip = screen.get(By::role("tooltip")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let tooltips = screen.get_all(By::role("tooltip")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let tooltips = screen.find_all(By::role("tooltip")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let tooltip = screen.find(By::role("tooltip")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let items = screen.query_all(By::role("item")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let item = screen.query(By::role("item")).await?;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get_all(By::role("item")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::role("item")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find_all(By::role("item")).await;

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

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.find(By::role("item")).await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_exact(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test with exact name match
        let options = ByRoleOptions::new().name(TextMatch::Exact("Copy".to_string()));

        let button = screen.get(By::role_with_options("button", options.clone())).await?;
        assert_eq!(button.text().await?, "Copy");

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_exact_specific(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test with exact name match for specific button
        let options = ByRoleOptions::new().name(TextMatch::Exact("Show alert".to_string()));

        let button = screen.get(By::role_with_options("button", options.clone())).await?;
        assert_eq!(button.text().await?, "Show alert");

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_another_button(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test with exact name match for another button
        let options = ByRoleOptions::new().name(TextMatch::Exact("Show confirm".to_string()));

        let button = screen.get(By::role_with_options("button", options.clone())).await?;
        assert_eq!(button.text().await?, "Show confirm");

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_multiple_options(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test with multiple options
        let options = ByRoleOptions::new().name(TextMatch::Exact("Copy".to_string())).hidden(false);

        let button = screen.get(By::role_with_options("button", options.clone())).await?;
        assert_eq!(button.text().await?, "Copy");

        Ok(())
    })
}
