mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::{ Screen, Selector};
use thirtyfour::support::block_on;

#[rstest]
fn switch_pages_reload_the_scripts(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_text_exact_page_url();
        c.goto(&url).await?;


        let screen = Screen::build_with_testing_library(c.clone()).await?;
        // should not fail
        screen.get(Selector::text("Login")).await?;

        let url = by_title_exact_page_url();
        c.goto(&url).await?;

        screen.get(Selector::title("Click for Help")).await?;
        
        Ok(())
    })
}
