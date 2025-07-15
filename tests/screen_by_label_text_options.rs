mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByLabelTextOptions, Screen};

// 1. Selector Option Tests

#[rstest]
fn test_selector_option_input_vs_textarea(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_label_text_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test selector="input" finds input element
        let input_options = ByLabelTextOptions::new().selector("input".to_string());

        let input_element = screen
            .get(By::label_text_with_options(
                "Email Address".to_string(),
                input_options.clone(),
            ))
            .await?;
        assert_eq!(input_element.tag_name().await?, "input");
        assert_eq!(input_element.id().await?, Some("email-input".to_string()));

        // Test selector="textarea" finds textarea element
        let textarea_options = ByLabelTextOptions::new().selector("textarea".to_string());

        let textarea_element = screen
            .get(By::label_text_with_options(
                "Email Address".to_string(),
                textarea_options.clone(),
            ))
            .await?;
        assert_eq!(textarea_element.tag_name().await?, "textarea");
        assert_eq!(
            textarea_element.id().await?,
            Some("email-textarea".to_string())
        );

        // Test get_all_by_label_text_with_options with input selector
        let input_elements = screen
            .get_all(By::label_text_with_options(
                "Email Address",
                input_options.clone(),
            ))
            .await?;
        assert_eq!(input_elements.len(), 1);
        assert_eq!(input_elements[0].tag_name().await?, "input");

        // Test get_all_by_label_text_with_options with textarea selector
        let textarea_elements = screen
            .get_all(By::label_text_with_options(
                "Email Address",
                textarea_options.clone(),
            ))
            .await?;
        assert_eq!(textarea_elements.len(), 1);
        assert_eq!(textarea_elements[0].tag_name().await?, "textarea");

        // Test query_by_label_text_with_options
        let maybe_input = screen
            .query(By::label_text_with_options(
                "Email Address",
                input_options.clone(),
            ))
            .await?;
        assert!(maybe_input.is_some());
        assert_eq!(maybe_input.unwrap().tag_name().await?, "input");

        // Test query_all_by_label_text_with_options
        let query_inputs = screen
            .query_all(By::label_text_with_options(
                "Email Address",
                input_options.clone(),
            ))
            .await?;
        assert_eq!(query_inputs.len(), 1);
        assert_eq!(query_inputs[0].tag_name().await?, "input");

        // Test find_by_label_text_with_options
        let find_input = screen
            .find(By::label_text_with_options(
                "Email Address",
                input_options.clone(),
            ))
            .await?;
        assert_eq!(find_input.tag_name().await?, "input");

        // Test find_all_by_label_text_with_options
        let find_inputs = screen
            .find_all(By::label_text_with_options(
                "Email Address",
                input_options.clone(),
            ))
            .await?;
        assert_eq!(find_inputs.len(), 1);
        assert_eq!(find_inputs[0].tag_name().await?, "input");

        Ok(())
    })
}

#[rstest]
fn test_selector_option_with_id(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_label_text_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test selector with specific ID
        let id_options = ByLabelTextOptions::new().selector("#email-input".to_string());

        let element = screen
            .get(By::label_text_with_options(
                "Email Address".to_string(),
                id_options.clone(),
            ))
            .await?;
        assert_eq!(element.id().await?, Some("email-input".to_string()));
        assert_eq!(element.tag_name().await?, "input");

        // Test query_by_label_text_with_options
        let maybe_element = screen
            .query(By::label_text_with_options(
                "Email Address",
                id_options.clone(),
            ))
            .await?;
        assert!(maybe_element.is_some());
        assert_eq!(
            maybe_element.unwrap().id().await?,
            Some("email-input".to_string())
        );

        // Test find_by_label_text_with_options
        let find_element = screen
            .find(By::label_text_with_options(
                "Email Address",
                id_options.clone(),
            ))
            .await?;
        assert_eq!(find_element.id().await?, Some("email-input".to_string()));

        Ok(())
    })
}

// 2. Exact Option Tests

#[rstest]
fn test_exact_true_precise_match(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_label_text_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let exact_options = ByLabelTextOptions::new().exact(true);

        // Test exact match for "Password" (should find only password-input, not confirm-password)
        let password_element = screen
            .get(By::label_text_with_options(
                "Password".to_string(),
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(
            password_element.id().await?,
            Some("password-input".to_string())
        );

        // Test exact match for "Confirm Password"
        let confirm_element = screen
            .get(By::label_text_with_options(
                "Confirm Password".to_string(),
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(
            confirm_element.id().await?,
            Some("confirm-password".to_string())
        );

        // Test get_all_by_label_text_with_options
        let password_elements = screen
            .get_all(By::label_text_with_options(
                "Password",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(password_elements.len(), 1);
        assert_eq!(
            password_elements[0].id().await?,
            Some("password-input".to_string())
        );

        // Test query_by_label_text_with_options
        let maybe_password = screen
            .query(By::label_text_with_options(
                "Password",
                exact_options.clone(),
            ))
            .await?;
        assert!(maybe_password.is_some());
        assert_eq!(
            maybe_password.unwrap().id().await?,
            Some("password-input".to_string())
        );

        // Test query_all_by_label_text_with_options
        let query_passwords = screen
            .query_all(By::label_text_with_options(
                "Password",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(query_passwords.len(), 1);
        assert_eq!(
            query_passwords[0].id().await?,
            Some("password-input".to_string())
        );

        // Test find_by_label_text_with_options
        let find_password = screen
            .find(By::label_text_with_options(
                "Password",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(
            find_password.id().await?,
            Some("password-input".to_string())
        );

        // Test find_all_by_label_text_with_options
        let find_passwords = screen
            .find_all(By::label_text_with_options(
                "Password",
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(find_passwords.len(), 1);
        assert_eq!(
            find_passwords[0].id().await?,
            Some("password-input".to_string())
        );

        Ok(())
    })
}

#[rstest]
fn test_exact_false_partial_match(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_label_text_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let partial_options = ByLabelTextOptions::new().exact(false);

        // Test partial match - "Password" should find both password fields when exact=false
        let password_elements = screen
            .get_all(By::label_text_with_options(
                "Password",
                partial_options.clone(),
            ))
            .await?;
        assert_eq!(password_elements.len(), 2);

        // Verify we got both password elements
        let mut ids = Vec::new();
        for element in &password_elements {
            let id = element.id().await?.unwrap_or_default();
            ids.push(id);
        }
        assert!(ids.contains(&"password-input".to_string()));
        assert!(ids.contains(&"confirm-password".to_string()));

        // Test query_all_by_label_text_with_options
        let query_passwords = screen
            .query_all(By::label_text_with_options(
                "Password",
                partial_options.clone(),
            ))
            .await?;
        assert_eq!(query_passwords.len(), 2);

        // Test find_all_by_label_text_with_options
        let find_passwords = screen
            .find_all(By::label_text_with_options(
                "Password",
                partial_options.clone(),
            ))
            .await?;
        assert_eq!(find_passwords.len(), 2);

        Ok(())
    })
}

#[rstest]
fn test_exact_case_sensitivity(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_label_text_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let exact_options = ByLabelTextOptions::new().exact(true);

        // Test exact case match - "Country" should find the country select
        let country_element = screen
            .get(By::label_text_with_options(
                "Country".to_string(),
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(country_element.id().await?, Some("country".to_string()));
        assert_eq!(country_element.tag_name().await?, "select");

        // Test exact case match - "COUNTRY CODE" should find the country-code input
        let country_code_element = screen
            .get(By::label_text_with_options(
                "COUNTRY CODE".to_string(),
                exact_options.clone(),
            ))
            .await?;
        assert_eq!(
            country_code_element.id().await?,
            Some("country-code".to_string())
        );
        assert_eq!(country_code_element.tag_name().await?, "input");

        // Test case sensitivity - "country" (lowercase) should not find "Country"
        let lowercase_result = screen
            .query(By::label_text_with_options(
                "country",
                exact_options.clone(),
            ))
            .await?;
        assert!(lowercase_result.is_none());

        // Test case sensitivity - "country code" (lowercase) should not find "COUNTRY CODE"
        let lowercase_code_result = screen
            .query(By::label_text_with_options(
                "country code",
                exact_options.clone(),
            ))
            .await?;
        assert!(lowercase_code_result.is_none());

        Ok(())
    })
}
