mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

#[rstest]
fn within(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("screen_within.html").await?;

        let parent_element = test_harness.driver().find(thirtyfour::prelude::By::Id("parent")).await?;
        let result = screen.get(By::text("Some text")).await;
        assert!(
            result.is_err(),
            "Expected error because there is 2 occurrences"
        );

        let within_screen = screen.within(parent_element);

        // testing the `get` and `find` methods beaucse they use different execution methods
        let result = within_screen.get(By::text("Some text")).await;
        assert_id(&result.unwrap(), "child").await?;

        let result = within_screen.find(By::text("Some text")).await;
        assert_id(&result.unwrap(), "child").await?;

        Ok(())
    })
}
