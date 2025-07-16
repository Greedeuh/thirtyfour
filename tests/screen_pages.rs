mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn switch_pages_reload_the_scripts(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_text_exact.html").await?;
        // should not fail
        screen.get(By::text("Login")).await?;

        let screen = test_harness.screen_for_page("by_title_exact.html").await?;
        screen.get(By::title("Click for Help")).await?;

        Ok(())
    })
}
