use crate::common::*;
use rstest::rstest;
use thirtyfour::screen::Screen;
use thirtyfour::support::block_on;
use thirtyfour::{ prelude::*};

mod common;

#[rstest]
fn by_role(test_harness: TestHarness) -> WebDriverResult<()> {
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
