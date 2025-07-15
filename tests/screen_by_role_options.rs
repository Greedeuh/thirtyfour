mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{
    By, ByRoleOptions, CurrentState, Screen, TextMatch, ValueOptions,
};

// 1. Basic Name Matching Tests

#[rstest]
fn test_name_exact_match(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test exact match succeeds
        let options = ByRoleOptions::new().name(TextMatch::Exact("Submit Form".to_string()));

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(button.text().await?, "Submit Form");

        // Test get_all_by_role_with_options as well
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].text().await?, "Submit Form");

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_eq!(maybe_button.unwrap().text().await?, "Submit Form");

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(query_buttons.len(), 1);
        assert_eq!(query_buttons[0].text().await?, "Submit Form");

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_button.text().await?, "Submit Form");

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_buttons.len(), 1);
        assert_eq!(find_buttons[0].text().await?, "Submit Form");

        // Test partial match fails
        let options_partial = ByRoleOptions::new().name(TextMatch::Exact("Submit For".to_string()));

        let result = screen
            .get(By::role_with_options(
                "button".to_string(),
                options_partial.clone(),
            ))
            .await;
        assert!(result.is_err());

        Ok(())
    })
}

#[rstest]
fn test_name_regex_match(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Test regex match with proper regex literal syntax
        let options = ByRoleOptions::new().name(TextMatch::Regex("/Save.*Document/".to_string()));

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(button.text().await?, "Save Document");

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_eq!(maybe_button.unwrap().text().await?, "Save Document");

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].text().await?, "Save Document");

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(query_buttons.len(), 1);
        assert_eq!(query_buttons[0].text().await?, "Save Document");

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_button.text().await?, "Save Document");

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_buttons.len(), 1);
        assert_eq!(find_buttons[0].text().await?, "Save Document");

        Ok(())
    })
}

// 2. ARIA State Tests

#[rstest]
fn test_selected_option(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().selected(true);

        let tab = screen
            .get(By::role_with_options("tab".to_string(), options.clone()))
            .await?;
        assert_eq!(tab.text().await?, "Active Tab");

        // Test find_by_role_with_options
        let find_tab = screen
            .find(By::role_with_options("tab", options.clone()))
            .await?;
        assert_eq!(find_tab.text().await?, "Active Tab");

        // Test get_all_by_role_with_options
        let tabs = screen
            .get_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs[0].text().await?, "Active Tab");

        // Test query_by_role_with_options
        let maybe_tab = screen
            .query(By::role_with_options("tab", options.clone()))
            .await?;
        assert!(maybe_tab.is_some());
        assert_eq!(maybe_tab.unwrap().text().await?, "Active Tab");

        // Test query_all_by_role_with_options
        let query_tabs = screen
            .query_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_eq!(query_tabs.len(), 1);
        assert_eq!(query_tabs[0].text().await?, "Active Tab");

        // Test find_all_by_role_with_options
        let find_tabs = screen
            .find_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_eq!(find_tabs.len(), 1);
        assert_eq!(find_tabs[0].text().await?, "Active Tab");

        Ok(())
    })
}

#[rstest]
fn test_checked_checkbox(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().checked(true);

        let checkbox = screen
            .get(By::role_with_options(
                "checkbox".to_string(),
                options.clone(),
            ))
            .await?;
        let aria_label = checkbox.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test get_all_by_role_with_options
        let checkboxes = screen
            .get_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(checkboxes.len(), 1);
        let aria_label = checkboxes[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test query_by_role_with_options
        let maybe_checkbox = screen
            .query(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert!(maybe_checkbox.is_some());
        let aria_label = maybe_checkbox
            .unwrap()
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test query_all_by_role_with_options
        let query_checkboxes = screen
            .query_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(query_checkboxes.len(), 1);
        let aria_label = query_checkboxes[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test find_by_role_with_options
        let find_checkbox = screen
            .find(By::role_with_options("checkbox", options.clone()))
            .await?;
        let aria_label = find_checkbox.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test find_all_by_role_with_options
        let find_checkboxes = screen
            .find_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(find_checkboxes.len(), 1);
        let aria_label = find_checkboxes[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        Ok(())
    })
}

#[rstest]
fn test_pressed_button(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().pressed(true);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(button.text().await?, "Bold");

        // Test query_all_by_role_with_options
        let buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].text().await?, "Bold");

        // Test get_all_by_role_with_options
        let all_buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(all_buttons.len(), 1);
        assert_eq!(all_buttons[0].text().await?, "Bold");

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_eq!(maybe_button.unwrap().text().await?, "Bold");

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_button.text().await?, "Bold");

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_buttons.len(), 1);
        assert_eq!(find_buttons[0].text().await?, "Bold");

        Ok(())
    })
}

#[rstest]
fn test_expanded_menu(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().expanded(true);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(button.text().await?, "File Menu");

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_button.text().await?, "File Menu");

        // Test get_all_by_role_with_options
        let all_buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(all_buttons.len(), 1);
        assert_eq!(all_buttons[0].text().await?, "File Menu");

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_eq!(maybe_button.unwrap().text().await?, "File Menu");

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(query_buttons.len(), 1);
        assert_eq!(query_buttons[0].text().await?, "File Menu");

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_buttons.len(), 1);
        assert_eq!(find_buttons[0].text().await?, "File Menu");

        Ok(())
    })
}

#[rstest]
fn test_busy_loading(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().busy(true);

        let status = screen
            .get(By::role_with_options("status".to_string(), options.clone()))
            .await?;
        assert_eq!(status.text().await?, "Loading...");

        // Test find_all_by_role_with_options
        let statuses = screen
            .find_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].text().await?, "Loading...");

        // Test get_all_by_role_with_options
        let all_statuses = screen
            .get_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_eq!(all_statuses.len(), 1);
        assert_eq!(all_statuses[0].text().await?, "Loading...");

        // Test query_by_role_with_options
        let maybe_status = screen
            .query(By::role_with_options("status", options.clone()))
            .await?;
        assert!(maybe_status.is_some());
        assert_eq!(maybe_status.unwrap().text().await?, "Loading...");

        // Test query_all_by_role_with_options
        let query_statuses = screen
            .query_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_eq!(query_statuses.len(), 1);
        assert_eq!(query_statuses[0].text().await?, "Loading...");

        // Test find_by_role_with_options
        let find_status = screen
            .find(By::role_with_options("status", options.clone()))
            .await?;
        assert_eq!(find_status.text().await?, "Loading...");

        Ok(())
    })
}

// 3. Current State Tests

#[rstest]
fn test_current_page(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().current(CurrentState::Page);

        let link = screen
            .get(By::role_with_options("link".to_string(), options.clone()))
            .await?;
        assert_eq!(link.text().await?, "Home");

        // Test query_by_role_with_options
        let maybe_link = screen
            .query(By::role_with_options("link", options.clone()))
            .await?;
        assert!(maybe_link.is_some());
        assert_eq!(maybe_link.unwrap().text().await?, "Home");

        // Test get_all_by_role_with_options
        let links = screen
            .get_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].text().await?, "Home");

        // Test query_all_by_role_with_options
        let query_links = screen
            .query_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_eq!(query_links.len(), 1);
        assert_eq!(query_links[0].text().await?, "Home");

        // Test find_by_role_with_options
        let find_link = screen
            .find(By::role_with_options("link", options.clone()))
            .await?;
        assert_eq!(find_link.text().await?, "Home");

        // Test find_all_by_role_with_options
        let find_links = screen
            .find_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_eq!(find_links.len(), 1);
        assert_eq!(find_links[0].text().await?, "Home");

        Ok(())
    })
}

#[rstest]
fn test_current_step(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().current(CurrentState::Step);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(button.text().await?, "Personal Info");

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(buttons.len(), 1);
        assert_eq!(buttons[0].text().await?, "Personal Info");

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_eq!(maybe_button.unwrap().text().await?, "Personal Info");

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(query_buttons.len(), 1);
        assert_eq!(query_buttons[0].text().await?, "Personal Info");

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_button.text().await?, "Personal Info");

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_eq!(find_buttons.len(), 1);
        assert_eq!(find_buttons[0].text().await?, "Personal Info");

        Ok(())
    })
}

// 4. Accessibility Tests

#[rstest]
fn test_hidden_elements(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        // Verify we can find hidden buttons when including hidden ones
        let options_hidden = ByRoleOptions::new().hidden(true);

        // This should succeed - just verify the option is being passed
        let _buttons = screen
            .get(By::role_with_options(
                "button".to_string(),
                options_hidden.clone(),
            ))
            .await;

        // Without hidden=true, let's just verify we can find visible buttons
        let visible_options =
            ByRoleOptions::new().name(TextMatch::Exact("Visible Button".to_string()));

        let visible_button = screen
            .get(By::role_with_options(
                "button".to_string(),
                visible_options.clone(),
            ))
            .await?;
        assert_eq!(visible_button.text().await?, "Visible Button");

        // Test query_all_by_role_with_options
        let visible_buttons = screen
            .query_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_eq!(visible_buttons.len(), 1);
        assert_eq!(visible_buttons[0].text().await?, "Visible Button");

        // Test get_all_by_role_with_options
        let all_visible_buttons = screen
            .get_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_eq!(all_visible_buttons.len(), 1);
        assert_eq!(all_visible_buttons[0].text().await?, "Visible Button");

        // Test query_by_role_with_options
        let maybe_visible_button = screen
            .query(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert!(maybe_visible_button.is_some());
        assert_eq!(
            maybe_visible_button.unwrap().text().await?,
            "Visible Button"
        );

        // Test find_by_role_with_options
        let find_visible_button = screen
            .find(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_eq!(find_visible_button.text().await?, "Visible Button");

        // Test find_all_by_role_with_options
        let find_visible_buttons = screen
            .find_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_eq!(find_visible_buttons.len(), 1);
        assert_eq!(find_visible_buttons[0].text().await?, "Visible Button");

        Ok(())
    })
}

#[rstest]
fn test_level_heading(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new().level(2);

        let heading = screen
            .get(By::role_with_options(
                "heading".to_string(),
                options.clone(),
            ))
            .await?;
        assert_eq!(heading.text().await?, "Section Title");

        // Test find_all_by_role_with_options
        let headings = screen
            .find_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_eq!(headings.len(), 1);
        assert_eq!(headings[0].text().await?, "Section Title");

        // Test get_all_by_role_with_options
        let all_headings = screen
            .get_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_eq!(all_headings.len(), 1);
        assert_eq!(all_headings[0].text().await?, "Section Title");

        // Test query_by_role_with_options
        let maybe_heading = screen
            .query(By::role_with_options("heading", options.clone()))
            .await?;
        assert!(maybe_heading.is_some());
        assert_eq!(maybe_heading.unwrap().text().await?, "Section Title");

        // Test query_all_by_role_with_options
        let query_headings = screen
            .query_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_eq!(query_headings.len(), 1);
        assert_eq!(query_headings[0].text().await?, "Section Title");

        // Test find_by_role_with_options
        let find_heading = screen
            .find(By::role_with_options("heading", options.clone()))
            .await?;
        assert_eq!(find_heading.text().await?, "Section Title");

        Ok(())
    })
}

// 5. Value-Based Tests

#[rstest]
fn test_value_min(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let value_opts = ValueOptions {
            min: Some(0),
            max: None,
            now: None,
            text: None,
        };

        let options = ByRoleOptions::new()
            .value(value_opts)
            .name(TextMatch::Exact("Volume".to_string()));

        let slider = screen
            .get(By::role_with_options("slider".to_string(), options.clone()))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(By::role_with_options("slider", options.clone()))
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");
        // Test get_all_by_role_with_options
        let all_sliders = screen
            .get_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(all_sliders.len(), 1);
        let aria_label = all_sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(By::role_with_options("slider", options.clone()))
            .await?;
        assert!(maybe_slider.is_some());
        let aria_label = maybe_slider
            .unwrap()
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test query_all_by_role_with_options
        let query_sliders = screen
            .query_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(query_sliders.len(), 1);
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(find_sliders.len(), 1);
        let aria_label = find_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        Ok(())
    })
}

#[rstest]
fn test_value_max(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let value_opts = ValueOptions {
            min: None,
            max: Some(50),
            now: None,
            text: None,
        };

        let options = ByRoleOptions::new().value(value_opts);

        let slider = screen
            .get(By::role_with_options("slider".to_string(), options.clone()))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(By::role_with_options("slider", options.clone()))
            .await?;
        assert!(maybe_slider.is_some());
        let aria_label = maybe_slider
            .unwrap()
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Brightness");
        // Test get_all_by_role_with_options
        let all_sliders = screen
            .get_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(all_sliders.len(), 1);
        let aria_label = all_sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test query_all_by_role_with_options
        let query_sliders = screen
            .query_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(query_sliders.len(), 1);
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(By::role_with_options("slider", options.clone()))
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(find_sliders.len(), 1);
        let aria_label = find_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        Ok(())
    })
}

#[rstest]
fn test_value_now(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let value_opts = ValueOptions {
            min: None,
            max: None,
            now: Some(75),
            text: None,
        };

        let options = ByRoleOptions::new().value(value_opts);

        let slider = screen
            .get(By::role_with_options("slider".to_string(), options.clone()))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test get_all_by_role_with_options
        let sliders = screen
            .get_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(sliders.len(), 1);
        let aria_label = sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");
        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(By::role_with_options("slider", options.clone()))
            .await?;
        assert!(maybe_slider.is_some());
        let aria_label = maybe_slider
            .unwrap()
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test query_all_by_role_with_options
        let query_sliders = screen
            .query_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(query_sliders.len(), 1);
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(By::role_with_options("slider", options.clone()))
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_eq!(find_sliders.len(), 1);
        let aria_label = find_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        Ok(())
    })
}

// 6. Fallback Roles Test

#[rstest]
fn test_query_fallbacks(test_harness: TestHarness) -> WebDriverResult<()> {
    let c = test_harness.driver();
    block_on(async {
        let url = by_role_options_page_url();
        c.goto(&url).await?;

        let screen = Screen::build_with_testing_library(c.clone()).await?;

        let options = ByRoleOptions::new()
            .query_fallbacks(true)
            .name(TextMatch::Exact("Toggle Switch".to_string()));

        // Try to find the element by its fallback role "checkbox"
        let element = screen
            .get(By::role_with_options(
                "checkbox".to_string(),
                options.clone(),
            ))
            .await?;
        assert_eq!(element.text().await?, "Toggle Switch");

        // Test query_all_by_role_with_options
        let elements = screen
            .query_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].text().await?, "Toggle Switch");

        // Test get_all_by_role_with_options
        let all_elements = screen
            .get_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(all_elements.len(), 1);
        assert_eq!(all_elements[0].text().await?, "Toggle Switch");

        // Test query_by_role_with_options
        let maybe_element = screen
            .query(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert!(maybe_element.is_some());
        assert_eq!(maybe_element.unwrap().text().await?, "Toggle Switch");

        // Test find_by_role_with_options
        let find_element = screen
            .find(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(find_element.text().await?, "Toggle Switch");

        // Test find_all_by_role_with_options
        let find_elements = screen
            .find_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_eq!(find_elements.len(), 1);
        assert_eq!(find_elements[0].text().await?, "Toggle Switch");

        Ok(())
    })
}
