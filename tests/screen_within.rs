mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, Screen};

#[rstest]
fn within(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = screen_within_page_url();
        c.goto(&url).await?;

        let parent_element = c.find(thirtyfour::prelude::By::Id("parent")).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(By::text("Some text")).await;
        assert!(result.is_err(), "Expected error because there is 2 occurrences");

        let within_screen = screen.within(parent_element);

        // testing the `get` and `find` methods beaucse they use different execution methods
        let result = within_screen.get(By::text("Some text")).await;
        assert_eq!(result.unwrap().id().await?.unwrap(), "child");

        let result = within_screen.find(By::text("Some text")).await;
        assert_eq!(result.unwrap().id().await?.unwrap(), "child");

        Ok(())
    })
}
