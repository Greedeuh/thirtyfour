use crate::common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::screen::Screen;
use thirtyfour::support::block_on;

mod common;

#[rstest]
fn query_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
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

#[rstest]
fn query_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;

        let tooltip = screen.query_by_role("tooltip").await?;
        assert!(tooltip.is_some());
        assert_eq!(tooltip.unwrap().text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn get_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltip = screen.get_by_role("tooltip").await?;

        assert_eq!(tooltip.text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn get_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltips = screen.get_all_by_role("tooltip").await?;

        assert_eq!(tooltips.len(), 1);
        assert_eq!(tooltips[0].text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn find_all_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltips = screen.find_all_by_role("tooltip").await?;

        assert_eq!(tooltips.len(), 1);
        assert_eq!(tooltips[0].text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn find_by_role(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let tooltip = screen.find_by_role("tooltip").await?;

        assert_eq!(tooltip.text().await?, "Tooltip text");

        Ok(())
    })
}

#[rstest]
fn query_all_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let items = screen.query_all_by_role("item").await?;

        assert_eq!(items.len(), 0);

        Ok(())
    })
}

#[rstest]
fn query_by_role_item_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let item = screen.query_by_role("item").await?;

        assert!(item.is_none());

        Ok(())
    })
}

#[rstest]
fn get_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_all_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_all_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_all_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn find_by_role_item_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.find_by_role("item").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let span = screen.get_by_text("some text to find").await?;

        assert_eq!(span.id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn get_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let input = screen.get_by_label_text("User name:").await?;

        assert_eq!(input.id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn get_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let option = screen.get_by_display_value("Red").await?;

        assert_eq!(option.value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn query_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let span = screen.query_by_text("some text to find").await?;

        assert!(span.is_some());
        assert_eq!(span.unwrap().id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn query_by_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let button = screen.query_by_text("NonExistent").await?;

        assert!(button.is_none());

        Ok(())
    })
}

#[rstest]
fn get_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let spans = screen.get_all_by_text("some text to find").await?;

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn query_all_by_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let spans = screen.query_all_by_text("some text to find").await?;

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].id().await?.unwrap(), "text-to-find");

        Ok(())
    })
}

#[rstest]
fn get_all_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let username = screen.get_by_label_text("User name:").await?;

        assert_eq!(username.id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn query_by_label_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let password = screen.query_by_label_text("User name:").await?;

        assert!(password.is_some());
        assert_eq!(password.unwrap().id().await?.unwrap(), "user-name");

        Ok(())
    })
}

#[rstest]
fn get_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let email_input = screen.get_by_placeholder_text("jean@email.fr").await?;

        assert_eq!(email_input.id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let email_input = screen.query_by_placeholder_text("jean@email.fr").await?;

        assert!(email_input.is_some());
        assert_eq!(email_input.unwrap().id().await?.unwrap(), "email");

        Ok(())
    })
}

#[rstest]
fn get_all_by_placeholder_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let inputs = screen.get_all_by_placeholder_text("jean@email.fr").await?;

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].id().await?.unwrap(), "email");

        Ok(())
    })
}


#[rstest]
fn query_by_display_value(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let option = screen.query_by_display_value("Red").await?;

        assert!(option.is_some());
        assert_eq!(option.unwrap().value().await?.unwrap(), "red");

        Ok(())
    })
}

#[rstest]
fn get_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let image = screen.get_by_alt_text("Some image").await?;

        assert_eq!(image.id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let image: Option<WebElement> = screen.query_by_alt_text("Some image").await?;

        assert!(image.is_some());
        assert_eq!(image.unwrap().id().await?.unwrap(), "some-image");

        Ok(())
    })
}

#[rstest]
fn get_all_by_alt_text(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let images = screen.get_all_by_alt_text("Some image").await?;

        assert_eq!(images.len(), 1);

        Ok(())
    })
}

#[rstest]
fn get_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let save_btn = screen.get_by_title("Some title").await?;

        assert_eq!(save_btn.id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn query_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let delete_btn = screen.query_by_title("Some title").await?;

        assert!(delete_btn.is_some());
        assert_eq!(delete_btn.unwrap().id().await?.unwrap(), "some-title");

        Ok(())
    })
}

#[rstest]
fn get_all_by_title(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let buttons = screen.get_all_by_title("Some title").await?;

        assert_eq!(buttons.len(), 1);

        Ok(())
    })
}

#[rstest]
fn get_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notification = screen.get_by_test_id("notification").await?;

        assert_eq!(notification.id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn query_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notification = screen.query_by_test_id("notification").await?;

        assert!(notification.is_some());
        assert_eq!(notification.unwrap().id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn get_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notifications = screen.get_all_by_test_id("notification").await?;

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id().await?.unwrap(), "notification");

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let notifications = screen.query_all_by_test_id("notification").await?;

        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].id().await?.unwrap(), "notification");

        Ok(())
    })
}

// Negative test cases - methods that should fail
#[rstest]
fn get_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_text("NonExistentText").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_all_by_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_all_by_text("NonExistentText").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_label_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_label_text("NonExistentLabel").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_placeholder_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_placeholder_text("NonExistentPlaceholder").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_display_value_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_display_value("NonExistentValue").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_alt_text_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_alt_text("NonExistentAlt").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_title_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_title("NonExistentTitle").await;

        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn get_by_test_id_should_fail(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.get_by_test_id("non-existent-test-id").await;

        assert!(result.is_err());

        Ok(())
    })
}

// Query methods that should return None
#[rstest]
fn query_by_label_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_label_text("NonExistentLabel").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_by_placeholder_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_placeholder_text("NonExistentPlaceholder").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_by_display_value_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_display_value("NonExistentValue").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_by_alt_text_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_alt_text("NonExistentAlt").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_by_title_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_title("NonExistentTitle").await?;

        assert!(result.is_none());

        Ok(())
    })
}

#[rstest]
fn query_by_test_id_not_found(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_by_test_id("non-existent-test-id").await?;

        assert!(result.is_none());

        Ok(())
    })
}

// Query all methods that should return empty arrays
#[rstest]
fn query_all_by_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_all_by_text("NonExistentText").await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}

#[rstest]
fn query_all_by_label_text_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_all_by_label_text("NonExistentLabel").await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}

#[rstest]
fn query_all_by_test_id_empty(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = sample_page_url();
        c.goto(&url).await?;

        let screen = Screen::load_with_testing_library(c.clone()).await?;
        let result = screen.query_all_by_test_id("non-existent-test-id").await?;

        assert_eq!(result.len(), 0);

        Ok(())
    })
}
