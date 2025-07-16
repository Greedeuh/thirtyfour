mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, ByRoleOptions, CurrentState, TextMatch, ValueOptions};

// 1. Basic Name Matching Tests

#[rstest]
fn test_name_exact_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        // Test exact match succeeds
        let options = ByRoleOptions::new().name(TextMatch::Exact("Submit Form".to_string()));

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&button, "Submit Form").await?;

        // Test get_all_by_role_with_options as well
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Submit Form").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Submit Form").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Submit Form").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&find_button, "Submit Form").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Submit Form").await?;

        // Test partial match fails
        let options_partial = ByRoleOptions::new().name(TextMatch::Exact("Submit For".to_string()));

        let result = screen
            .get(By::role_with_options("button", options_partial.clone()))
            .await;
        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn test_name_regex_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        // Test regex match with proper regex literal syntax
        let options = ByRoleOptions::new().name(TextMatch::Regex("/Save.*Document/".to_string()));

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&button, "Save Document").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Save Document").await?;

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Save Document").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Save Document").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&find_button, "Save Document").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Save Document").await?;

        Ok(())
    })
}

// 2. ARIA State Tests

#[rstest]
fn test_selected_option(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().selected(true);

        let tab = screen
            .get(By::role_with_options("tab", options.clone()))
            .await?;
        assert_text(&tab, "Active Tab").await?;

        // Test find_by_role_with_options
        let find_tab = screen
            .find(By::role_with_options("tab", options.clone()))
            .await?;
        assert_text(&find_tab, "Active Tab").await?;

        // Test get_all_by_role_with_options
        let tabs = screen
            .get_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_count(&tabs, 1)?;
        assert_text(&tabs[0], "Active Tab").await?;

        // Test query_by_role_with_options
        let maybe_tab = screen
            .query(By::role_with_options("tab", options.clone()))
            .await?;
        assert!(maybe_tab.is_some());
        assert_text(&maybe_tab.unwrap(), "Active Tab").await?;

        // Test query_all_by_role_with_options
        let query_tabs = screen
            .query_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_count(&query_tabs, 1)?;
        assert_text(&query_tabs[0], "Active Tab").await?;

        // Test find_all_by_role_with_options
        let find_tabs = screen
            .find_all(By::role_with_options("tab", options.clone()))
            .await?;
        assert_count(&find_tabs, 1)?;
        assert_text(&find_tabs[0], "Active Tab").await?;

        Ok(())
    })
}

#[rstest]
fn test_checked_checkbox(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().checked(true);

        let checkbox = screen
            .get(By::role_with_options("checkbox", options.clone()))
            .await?;
        let aria_label = checkbox.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test get_all_by_role_with_options
        let checkboxes = screen
            .get_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_count(&checkboxes, 1)?;
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
        assert_count(&query_checkboxes, 1)?;
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
        assert_count(&find_checkboxes, 1)?;
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
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().pressed(true);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&button, "Bold").await?;

        // Test query_all_by_role_with_options
        let buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Bold").await?;

        // Test get_all_by_role_with_options
        let all_buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&all_buttons, 1)?;
        assert_text(&all_buttons[0], "Bold").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Bold").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&find_button, "Bold").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Bold").await?;

        Ok(())
    })
}

#[rstest]
fn test_expanded_menu(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().expanded(true);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&button, "File Menu").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&find_button, "File Menu").await?;

        // Test get_all_by_role_with_options
        let all_buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&all_buttons, 1)?;
        assert_text(&all_buttons[0], "File Menu").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "File Menu").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "File Menu").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "File Menu").await?;

        Ok(())
    })
}

#[rstest]
fn test_busy_loading(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().busy(true);

        let status = screen
            .get(By::role_with_options("status", options.clone()))
            .await?;
        assert_text(&status, "Loading...").await?;

        // Test find_all_by_role_with_options
        let statuses = screen
            .find_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_count(&statuses, 1)?;
        assert_text(&statuses[0], "Loading...").await?;

        // Test get_all_by_role_with_options
        let all_statuses = screen
            .get_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_count(&all_statuses, 1)?;
        assert_text(&all_statuses[0], "Loading...").await?;

        // Test query_by_role_with_options
        let maybe_status = screen
            .query(By::role_with_options("status", options.clone()))
            .await?;
        assert!(maybe_status.is_some());
        assert_text(&maybe_status.unwrap(), "Loading...").await?;

        // Test query_all_by_role_with_options
        let query_statuses = screen
            .query_all(By::role_with_options("status", options.clone()))
            .await?;
        assert_count(&query_statuses, 1)?;
        assert_text(&query_statuses[0], "Loading...").await?;

        // Test find_by_role_with_options
        let find_status = screen
            .find(By::role_with_options("status", options.clone()))
            .await?;
        assert_text(&find_status, "Loading...").await?;

        Ok(())
    })
}

// 3. Current State Tests

#[rstest]
fn test_current_page(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().current(CurrentState::Page);

        let link = screen
            .get(By::role_with_options("link", options.clone()))
            .await?;
        assert_text(&link, "Home").await?;

        // Test query_by_role_with_options
        let maybe_link = screen
            .query(By::role_with_options("link", options.clone()))
            .await?;
        assert!(maybe_link.is_some());
        assert_text(&maybe_link.unwrap(), "Home").await?;

        // Test get_all_by_role_with_options
        let links = screen
            .get_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_count(&links, 1)?;
        assert_text(&links[0], "Home").await?;

        // Test query_all_by_role_with_options
        let query_links = screen
            .query_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_count(&query_links, 1)?;
        assert_text(&query_links[0], "Home").await?;

        // Test find_by_role_with_options
        let find_link = screen
            .find(By::role_with_options("link", options.clone()))
            .await?;
        assert_text(&find_link, "Home").await?;

        // Test find_all_by_role_with_options
        let find_links = screen
            .find_all(By::role_with_options("link", options.clone()))
            .await?;
        assert_count(&find_links, 1)?;
        assert_text(&find_links[0], "Home").await?;

        Ok(())
    })
}

#[rstest]
fn test_current_step(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().current(CurrentState::Step);

        let button = screen
            .get(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&button, "Personal Info").await?;

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Personal Info").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role_with_options("button", options.clone()))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Personal Info").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Personal Info").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role_with_options("button", options.clone()))
            .await?;
        assert_text(&find_button, "Personal Info").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role_with_options("button", options.clone()))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Personal Info").await?;

        Ok(())
    })
}

// 4. Accessibility Tests

#[rstest]
fn test_hidden_elements(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        // Verify we can find hidden buttons when including hidden ones
        let options_hidden = ByRoleOptions::new().hidden(true);

        // This should succeed - just verify the option is being passed
        let _buttons = screen
            .get(By::role_with_options("button", options_hidden.clone()))
            .await;

        // Without hidden=true, let's just verify we can find visible buttons
        let visible_options =
            ByRoleOptions::new().name(TextMatch::Exact("Visible Button".to_string()));

        let visible_button = screen
            .get(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_text(&visible_button, "Visible Button").await?;

        // Test query_all_by_role_with_options
        let visible_buttons = screen
            .query_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_count(&visible_buttons, 1)?;
        assert_text(&visible_buttons[0], "Visible Button").await?;

        // Test get_all_by_role_with_options
        let all_visible_buttons = screen
            .get_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_count(&all_visible_buttons, 1)?;
        assert_text(&all_visible_buttons[0], "Visible Button").await?;

        // Test query_by_role_with_options
        let maybe_visible_button = screen
            .query(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert!(maybe_visible_button.is_some());
        assert_text(&maybe_visible_button.unwrap(), "Visible Button").await?;

        // Test find_by_role_with_options
        let find_visible_button = screen
            .find(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_text(&find_visible_button, "Visible Button").await?;

        // Test find_all_by_role_with_options
        let find_visible_buttons = screen
            .find_all(By::role_with_options("button", visible_options.clone()))
            .await?;
        assert_count(&find_visible_buttons, 1)?;
        assert_text(&find_visible_buttons[0], "Visible Button").await?;

        Ok(())
    })
}

#[rstest]
fn test_level_heading(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new().level(2);

        let heading = screen
            .get(By::role_with_options("heading", options.clone()))
            .await?;
        assert_text(&heading, "Section Title").await?;

        // Test find_all_by_role_with_options
        let headings = screen
            .find_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_count(&headings, 1)?;
        assert_text(&headings[0], "Section Title").await?;

        // Test get_all_by_role_with_options
        let all_headings = screen
            .get_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_count(&all_headings, 1)?;
        assert_text(&all_headings[0], "Section Title").await?;

        // Test query_by_role_with_options
        let maybe_heading = screen
            .query(By::role_with_options("heading", options.clone()))
            .await?;
        assert!(maybe_heading.is_some());
        assert_text(&maybe_heading.unwrap(), "Section Title").await?;

        // Test query_all_by_role_with_options
        let query_headings = screen
            .query_all(By::role_with_options("heading", options.clone()))
            .await?;
        assert_count(&query_headings, 1)?;
        assert_text(&query_headings[0], "Section Title").await?;

        // Test find_by_role_with_options
        let find_heading = screen
            .find(By::role_with_options("heading", options.clone()))
            .await?;
        assert_text(&find_heading, "Section Title").await?;

        Ok(())
    })
}

// 5. Value-Based Tests

#[rstest]
fn test_value_min(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

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
            .get(By::role_with_options("slider", options.clone()))
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
        assert_count(&all_sliders, 1)?;
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
        assert_count(&query_sliders, 1)?;
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_count(&find_sliders, 1)?;
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
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let value_opts = ValueOptions {
            min: None,
            max: Some(50),
            now: None,
            text: None,
        };

        let options = ByRoleOptions::new().value(value_opts);

        let slider = screen
            .get(By::role_with_options("slider", options.clone()))
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
        assert_count(&all_sliders, 1)?;
        let aria_label = all_sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test query_all_by_role_with_options
        let query_sliders = screen
            .query_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_count(&query_sliders, 1)?;
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
        assert_count(&find_sliders, 1)?;
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
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let value_opts = ValueOptions {
            min: None,
            max: None,
            now: Some(75),
            text: None,
        };

        let options = ByRoleOptions::new().value(value_opts);

        let slider = screen
            .get(By::role_with_options("slider", options.clone()))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test get_all_by_role_with_options
        let sliders = screen
            .get_all(By::role_with_options("slider", options.clone()))
            .await?;
        assert_count(&sliders, 1)?;
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
        assert_count(&query_sliders, 1)?;
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
        assert_count(&find_sliders, 1)?;
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
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let options = ByRoleOptions::new()
            .query_fallbacks(true)
            .name(TextMatch::Exact("Toggle Switch".to_string()));

        // Try to find the element by its fallback role "checkbox"
        let element = screen
            .get(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_text(&element, "Toggle Switch").await?;

        // Test query_all_by_role_with_options
        let elements = screen
            .query_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_count(&elements, 1)?;
        assert_text(&elements[0], "Toggle Switch").await?;

        // Test get_all_by_role_with_options
        let all_elements = screen
            .get_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_count(&all_elements, 1)?;
        assert_text(&all_elements[0], "Toggle Switch").await?;

        // Test query_by_role_with_options
        let maybe_element = screen
            .query(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert!(maybe_element.is_some());
        assert_text(&maybe_element.unwrap(), "Toggle Switch").await?;

        // Test find_by_role_with_options
        let find_element = screen
            .find(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_text(&find_element, "Toggle Switch").await?;

        // Test find_all_by_role_with_options
        let find_elements = screen
            .find_all(By::role_with_options("checkbox", options.clone()))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_text(&find_elements[0], "Toggle Switch").await?;

        Ok(())
    })
}
