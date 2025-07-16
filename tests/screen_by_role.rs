mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, TextMatch};

#[rstest]
fn query_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let tooltips = screen.query_all(By::role("tooltip")).await?;

        assert_count(&tooltips, 1)?;
        assert_text(&tooltips[0], "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn query_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        let tooltip = screen.query(By::role("tooltip")).await?;
        assert!(tooltip.is_some());
        assert_text(&tooltip.unwrap(), "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let tooltip = screen.get(By::role("tooltip")).await?;

        assert_text(&tooltip, "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let tooltips = screen.get_all(By::role("tooltip")).await?;

        assert_count(&tooltips, 1)?;
        assert_text(&tooltips[0], "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let tooltips = screen.find_all(By::role("tooltip")).await?;

        assert_count(&tooltips, 1)?;
        assert_text(&tooltips[0], "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn find_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let tooltip = screen.find(By::role("tooltip")).await?;

        assert_text(&tooltip, "Tooltip text").await?;

        Ok(())
    })
}

#[rstest]
fn query_all_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let items = screen.query_all(By::role("item")).await?;

        assert_count(&items, 0)?;

        Ok(())
    })
}

#[rstest]
fn query_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let item = screen.query(By::role("item")).await?;

        assert_none(item)?;

        Ok(())
    })
}

#[rstest]
fn get_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get_all(By::role("item")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.get(By::role("item")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find_all(By::role("item")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn find_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;
        let result = screen.find(By::role("item")).await;

        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_exact(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        // Test with exact name match
        let button = screen
            .get(By::role("button").name(TextMatch::Exact("Copy".to_string())))
            .await?;
        assert_text(&button, "Copy").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_exact_specific(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        // Test with exact name match for specific button
        let button = screen
            .get(By::role("button").name(TextMatch::Exact("Show alert".to_string())))
            .await?;
        assert_text(&button, "Show alert").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_name_another_button(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        // Test with exact name match for another button
        let button = screen
            .get(By::role("button").name(TextMatch::Exact("Show confirm".to_string())))
            .await?;
        assert_text(&button, "Show confirm").await?;

        Ok(())
    })
}

#[rstest]
fn get_by_role_with_options_multiple_options(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("sample_page.html").await?;

        // Test with multiple options
        let button = screen
            .get(
                By::role("button")
                    .name(TextMatch::Exact("Copy".to_string()))
                    .hidden(false),
            )
            .await?;
        assert_text(&button, "Copy").await?;

        Ok(())
    })
}
