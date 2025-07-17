mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn log_testing_playground_url_full_document(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let playground_url = screen.log_testing_playground_url(None).await?;

        // Verify that we got a string that looks like a URL
        assert!(playground_url.starts_with("http"));
        assert!(playground_url.contains("testing-playground.com"));

        Ok(())
    })
}

#[rstest]
fn log_testing_playground_url_with_element(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let element = screen.get(By::text("some text to find")).await?;
        let playground_url = screen.log_testing_playground_url(Some(element)).await?;

        // Verify that we got a string that looks like a URL
        assert!(playground_url.starts_with("http"));
        assert!(playground_url.contains("testing-playground.com"));

        Ok(())
    })
}
