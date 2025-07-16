mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::By;

// 1. Selector Option Tests

#[rstest]
fn test_selector_option_input_vs_textarea(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_label_text_options.html")
            .await?;

        // Test selector="input" finds input element, should not find textarea
        let input_element = screen
            .get(By::label_text("Email Address").selector("input"))
            .await?;
        assert_eq!(input_element.tag_name().await?, "input");
        assert_eq!(input_element.id().await?, Some("email-input".to_string()));

        // Test selector="textarea" finds textarea element, should not find input
        let textarea_element = screen
            .get(By::label_text("Email Address").selector("textarea"))
            .await?;
        assert_eq!(textarea_element.tag_name().await?, "textarea");
        assert_eq!(
            textarea_element.id().await?,
            Some("email-textarea".to_string())
        );

        let input_elements = screen
            .get_all(By::label_text("Email Address").selector("input"))
            .await?;
        assert_count(&input_elements, 1)?;
        assert_eq!(input_elements[0].tag_name().await?, "input");

        let textarea_elements = screen
            .get_all(By::label_text("Email Address").selector("textarea"))
            .await?;
        assert_count(&textarea_elements, 1)?;
        assert_eq!(textarea_elements[0].tag_name().await?, "textarea");

        let maybe_input = screen
            .query(By::label_text("Email Address").selector("input"))
            .await?;
        assert!(maybe_input.is_some());
        assert_eq!(maybe_input.unwrap().tag_name().await?, "input");

        let query_inputs = screen
            .query_all(By::label_text("Email Address").selector("input"))
            .await?;
        assert_count(&query_inputs, 1)?;
        assert_eq!(query_inputs[0].tag_name().await?, "input");

        let find_input = screen
            .find(By::label_text("Email Address").selector("input"))
            .await?;
        assert_eq!(find_input.tag_name().await?, "input");

        let find_inputs = screen
            .find_all(By::label_text("Email Address").selector("input"))
            .await?;
        assert_count(&find_inputs, 1)?;
        assert_eq!(find_inputs[0].tag_name().await?, "input");

        Ok(())
    })
}

#[rstest]
fn test_exact_true_precise_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_label_text_options.html")
            .await?;

        // Test exact match for "Password" (should find only password-input, not confirm-password)
        let password_element = screen.get(By::label_text("Password").exact(true)).await?;
        assert_eq!(
            password_element.id().await?,
            Some("password-input".to_string())
        );

        // Test exact match for "Confirm Password"
        let confirm_element = screen
            .get(By::label_text("Confirm Password").exact(true))
            .await?;
        assert_eq!(
            confirm_element.id().await?,
            Some("confirm-password".to_string())
        );

        let password_elements = screen
            .get_all(By::label_text("Password").exact(true))
            .await?;
        assert_count(&password_elements, 1)?;
        assert_eq!(
            password_elements[0].id().await?,
            Some("password-input".to_string())
        );

        let maybe_password = screen.query(By::label_text("Password").exact(true)).await?;
        assert!(maybe_password.is_some());
        assert_eq!(
            maybe_password.unwrap().id().await?,
            Some("password-input".to_string())
        );

        let query_passwords = screen
            .query_all(By::label_text("Password").exact(true))
            .await?;
        assert_count(&query_passwords, 1)?;
        assert_eq!(
            query_passwords[0].id().await?,
            Some("password-input".to_string())
        );

        let find_password = screen.find(By::label_text("Password").exact(true)).await?;
        assert_eq!(
            find_password.id().await?,
            Some("password-input".to_string())
        );

        let find_passwords = screen
            .find_all(By::label_text("Password").exact(true))
            .await?;
        assert_count(&find_passwords, 1)?;
        assert_eq!(
            find_passwords[0].id().await?,
            Some("password-input".to_string())
        );

        Ok(())
    })
}

#[rstest]
fn test_exact_false_partial_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness
            .screen_for_page("by_label_text_options.html")
            .await?;

        // Test partial match - "Password" should find both password fields when exact=false
        let password_elements = screen
            .get_all(By::label_text("Password").exact(false))
            .await?;
        assert_count(&password_elements, 2)?;

        Ok(())
    })
}