mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn switch_pages_reload_the_scripts(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_text_exact_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        // should not fail
        screen.get(By::text("Login")).await?;

        let url = by_title_exact_page_url();
        c.goto(&url).await?;

        screen.get(By::title("Click for Help")).await?;

        Ok(())
    })
}
