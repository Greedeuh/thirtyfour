mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::{Screen, Selector};
use thirtyfour::support::block_on;

#[rstest]
fn within(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = screen_within_page_url();
        c.goto(&url).await?;

        let parent_element = c.find(By::Id("parent")).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;
        let result = screen.get(Selector::text("Some text")).await;
        assert!(result.is_err(), "Expected error because there is 2 occurrences");

        let within_screen = screen.within(parent_element);

        // testing the `get` and `find` methods beaucse they use different execution methods
        let result = within_screen.get(Selector::text("Some text")).await;
        assert_eq!(result.unwrap().id().await?.unwrap(), "child");

        let result = within_screen.find(Selector::text("Some text")).await;
        assert_eq!(result.unwrap().id().await?.unwrap(), "child");

        Ok(())
    })
}
