mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn test_text_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_text_exact.html").await?;

        screen.get(By::text("/^Please.*Here$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_alt_text_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_alt_text_exact.html").await?;

        screen.get(By::alt_text("/^Company.*Image$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_placeholder_text_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_placeholder_text_exact.html").await?;

        screen.get(By::placeholder_text("/^Enter.*username$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_test_id_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_test_id_exact.html").await?;

        screen.get(By::test_id("/^save.*button$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_title_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_title_exact.html").await?;

        screen.get(By::title("/^Click.*Help$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_display_value_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_display_value_exact.html").await?;

        screen.get(By::display_value("/^Submit.*Form$/")).await?;

        Ok(())
    })
}

#[rstest]
fn test_label_text_regex(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_label_text_options.html").await?;

        screen.get(By::label_text("/^Pa.*word$/")).await?;

        Ok(())
    })
}