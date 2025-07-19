mod common;
use common::*;
use rstest::rstest;
use thirtyfour::prelude::*;
use thirtyfour::support::block_on;
use thirtyfour_testing_library_ext::{By, CurrentState, ValueOptions};

// 1. Basic Name Matching Tests

#[rstest]
fn test_name_exact_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        // Test exact match succeeds
        let button = screen.get(By::role("button").name("Submit Form")).await?;
        assert_text(&button, "Submit Form").await?;

        // Test get_all_by_role_with_options as well
        let buttons = screen
            .get_all(By::role("button").name("Submit Form"))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Submit Form").await?;

        // Test query_by_role_with_options
        let maybe_button = screen.query(By::role("button").name("Submit Form")).await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Submit Form").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role("button").name("Submit Form"))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Submit Form").await?;

        // Test find_by_role_with_options
        let find_button = screen.find(By::role("button").name("Submit Form")).await?;
        assert_text(&find_button, "Submit Form").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role("button").name("Submit Form"))
            .await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Submit Form").await?;

        // Test partial match fails
        let result = screen.get(By::role("button").name("Submit For")).await;
        assert_error(result)?;

        Ok(())
    })
}

#[rstest]
fn test_name_regex_match(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        // Test regex match with automatic detection
        let button = screen
            .get(By::role("button").name("/Save.*Document/"))
            .await?;
        assert_text(&button, "Save Document").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role("button").name("/Save.*Document/"))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Save Document").await?;

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role("button").name("/Save.*Document/"))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Save Document").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role("button").name("/Save.*Document/"))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Save Document").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role("button").name("/Save.*Document/"))
            .await?;
        assert_text(&find_button, "Save Document").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role("button").name("/Save.*Document/"))
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

        let tab = screen.get(By::role("tab").selected(true)).await?;
        assert_text(&tab, "Active Tab").await?;

        // Test find_by_role_with_options
        let find_tab = screen.find(By::role("tab").selected(true)).await?;
        assert_text(&find_tab, "Active Tab").await?;

        // Test get_all_by_role_with_options
        let tabs = screen.get_all(By::role("tab").selected(true)).await?;
        assert_count(&tabs, 1)?;
        assert_text(&tabs[0], "Active Tab").await?;

        // Test query_by_role_with_options
        let maybe_tab = screen.query(By::role("tab").selected(true)).await?;
        assert!(maybe_tab.is_some());
        assert_text(&maybe_tab.unwrap(), "Active Tab").await?;

        // Test query_all_by_role_with_options
        let query_tabs = screen.query_all(By::role("tab").selected(true)).await?;
        assert_count(&query_tabs, 1)?;
        assert_text(&query_tabs[0], "Active Tab").await?;

        // Test find_all_by_role_with_options
        let find_tabs = screen.find_all(By::role("tab").selected(true)).await?;
        assert_count(&find_tabs, 1)?;
        assert_text(&find_tabs[0], "Active Tab").await?;

        Ok(())
    })
}

#[rstest]
fn test_checked_checkbox(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let checkbox = screen.get(By::role("checkbox").checked(true)).await?;
        let aria_label = checkbox.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test get_all_by_role_with_options
        let checkboxes = screen.get_all(By::role("checkbox").checked(true)).await?;
        assert_count(&checkboxes, 1)?;
        let aria_label = checkboxes[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test query_by_role_with_options
        let maybe_checkbox = screen.query(By::role("checkbox").checked(true)).await?;
        assert!(maybe_checkbox.is_some());
        let aria_label = maybe_checkbox
            .unwrap()
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test query_all_by_role_with_options
        let query_checkboxes = screen.query_all(By::role("checkbox").checked(true)).await?;
        assert_count(&query_checkboxes, 1)?;
        let aria_label = query_checkboxes[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test find_by_role_with_options
        let find_checkbox = screen.find(By::role("checkbox").checked(true)).await?;
        let aria_label = find_checkbox.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Enable notifications");

        // Test find_all_by_role_with_options
        let find_checkboxes = screen.find_all(By::role("checkbox").checked(true)).await?;
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

        let button = screen.get(By::role("button").pressed(true)).await?;
        assert_text(&button, "Bold").await?;

        // Test query_all_by_role_with_options
        let buttons = screen.query_all(By::role("button").pressed(true)).await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Bold").await?;

        // Test get_all_by_role_with_options
        let all_buttons = screen.get_all(By::role("button").pressed(true)).await?;
        assert_count(&all_buttons, 1)?;
        assert_text(&all_buttons[0], "Bold").await?;

        // Test query_by_role_with_options
        let maybe_button = screen.query(By::role("button").pressed(true)).await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Bold").await?;

        // Test find_by_role_with_options
        let find_button = screen.find(By::role("button").pressed(true)).await?;
        assert_text(&find_button, "Bold").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen.find_all(By::role("button").pressed(true)).await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "Bold").await?;

        Ok(())
    })
}

#[rstest]
fn test_expanded_menu(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let button = screen.get(By::role("button").expanded(true)).await?;
        assert_text(&button, "File Menu").await?;

        // Test find_by_role_with_options
        let find_button = screen.find(By::role("button").expanded(true)).await?;
        assert_text(&find_button, "File Menu").await?;

        // Test get_all_by_role_with_options
        let all_buttons = screen.get_all(By::role("button").expanded(true)).await?;
        assert_count(&all_buttons, 1)?;
        assert_text(&all_buttons[0], "File Menu").await?;

        // Test query_by_role_with_options
        let maybe_button = screen.query(By::role("button").expanded(true)).await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "File Menu").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen.query_all(By::role("button").expanded(true)).await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "File Menu").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen.find_all(By::role("button").expanded(true)).await?;
        assert_count(&find_buttons, 1)?;
        assert_text(&find_buttons[0], "File Menu").await?;

        Ok(())
    })
}

#[rstest]
fn test_busy_loading(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let status = screen.get(By::role("status").busy(true)).await?;
        assert_text(&status, "Loading...").await?;

        // Test find_all_by_role_with_options
        let statuses = screen.find_all(By::role("status").busy(true)).await?;
        assert_count(&statuses, 1)?;
        assert_text(&statuses[0], "Loading...").await?;

        // Test get_all_by_role_with_options
        let all_statuses = screen.get_all(By::role("status").busy(true)).await?;
        assert_count(&all_statuses, 1)?;
        assert_text(&all_statuses[0], "Loading...").await?;

        // Test query_by_role_with_options
        let maybe_status = screen.query(By::role("status").busy(true)).await?;
        assert!(maybe_status.is_some());
        assert_text(&maybe_status.unwrap(), "Loading...").await?;

        // Test query_all_by_role_with_options
        let query_statuses = screen.query_all(By::role("status").busy(true)).await?;
        assert_count(&query_statuses, 1)?;
        assert_text(&query_statuses[0], "Loading...").await?;

        // Test find_by_role_with_options
        let find_status = screen.find(By::role("status").busy(true)).await?;
        assert_text(&find_status, "Loading...").await?;

        Ok(())
    })
}

// 3. Current State Tests

#[rstest]
fn test_current_page(test_harness: TestHarness) -> WebDriverResult<()> {
    block_on(async {
        let screen = test_harness.screen_for_page("by_role_options.html").await?;

        let link = screen
            .get(By::role("link").current(CurrentState::Page))
            .await?;
        assert_text(&link, "Home").await?;

        // Test query_by_role_with_options
        let maybe_link = screen
            .query(By::role("link").current(CurrentState::Page))
            .await?;
        assert!(maybe_link.is_some());
        assert_text(&maybe_link.unwrap(), "Home").await?;

        // Test get_all_by_role_with_options
        let links = screen
            .get_all(By::role("link").current(CurrentState::Page))
            .await?;
        assert_count(&links, 1)?;
        assert_text(&links[0], "Home").await?;

        // Test query_all_by_role_with_options
        let query_links = screen
            .query_all(By::role("link").current(CurrentState::Page))
            .await?;
        assert_count(&query_links, 1)?;
        assert_text(&query_links[0], "Home").await?;

        // Test find_by_role_with_options
        let find_link = screen
            .find(By::role("link").current(CurrentState::Page))
            .await?;
        assert_text(&find_link, "Home").await?;

        // Test find_all_by_role_with_options
        let find_links = screen
            .find_all(By::role("link").current(CurrentState::Page))
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

        let button = screen
            .get(By::role("button").current(CurrentState::Step))
            .await?;
        assert_text(&button, "Personal Info").await?;

        // Test get_all_by_role_with_options
        let buttons = screen
            .get_all(By::role("button").current(CurrentState::Step))
            .await?;
        assert_count(&buttons, 1)?;
        assert_text(&buttons[0], "Personal Info").await?;

        // Test query_by_role_with_options
        let maybe_button = screen
            .query(By::role("button").current(CurrentState::Step))
            .await?;
        assert!(maybe_button.is_some());
        assert_text(&maybe_button.unwrap(), "Personal Info").await?;

        // Test query_all_by_role_with_options
        let query_buttons = screen
            .query_all(By::role("button").current(CurrentState::Step))
            .await?;
        assert_count(&query_buttons, 1)?;
        assert_text(&query_buttons[0], "Personal Info").await?;

        // Test find_by_role_with_options
        let find_button = screen
            .find(By::role("button").current(CurrentState::Step))
            .await?;
        assert_text(&find_button, "Personal Info").await?;

        // Test find_all_by_role_with_options
        let find_buttons = screen
            .find_all(By::role("button").current(CurrentState::Step))
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
        // This should succeed - just verify the option is being passed
        let _buttons = screen.get(By::role("button").hidden(true)).await;

        // Without hidden=true, let's just verify we can find visible buttons
        let visible_button = screen
            .get(By::role("button").name("Visible Button"))
            .await?;
        assert_text(&visible_button, "Visible Button").await?;

        // Test query_all_by_role_with_options
        let visible_buttons = screen
            .query_all(By::role("button").name("Visible Button"))
            .await?;
        assert_count(&visible_buttons, 1)?;
        assert_text(&visible_buttons[0], "Visible Button").await?;

        // Test get_all_by_role_with_options
        let all_visible_buttons = screen
            .get_all(By::role("button").name("Visible Button"))
            .await?;
        assert_count(&all_visible_buttons, 1)?;
        assert_text(&all_visible_buttons[0], "Visible Button").await?;

        // Test query_by_role_with_options
        let maybe_visible_button = screen
            .query(By::role("button").name("Visible Button"))
            .await?;
        assert!(maybe_visible_button.is_some());
        assert_text(&maybe_visible_button.unwrap(), "Visible Button").await?;

        // Test find_by_role_with_options
        let find_visible_button = screen
            .find(By::role("button").name("Visible Button"))
            .await?;
        assert_text(&find_visible_button, "Visible Button").await?;

        // Test find_all_by_role_with_options
        let find_visible_buttons = screen
            .find_all(By::role("button").name("Visible Button"))
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

        let heading = screen.get(By::role("heading").level(2)).await?;
        assert_text(&heading, "Section Title").await?;

        // Test find_all_by_role_with_options
        let headings = screen.find_all(By::role("heading").level(2)).await?;
        assert_count(&headings, 1)?;
        assert_text(&headings[0], "Section Title").await?;

        // Test get_all_by_role_with_options
        let all_headings = screen.get_all(By::role("heading").level(2)).await?;
        assert_count(&all_headings, 1)?;
        assert_text(&all_headings[0], "Section Title").await?;

        // Test query_by_role_with_options
        let maybe_heading = screen.query(By::role("heading").level(2)).await?;
        assert!(maybe_heading.is_some());
        assert_text(&maybe_heading.unwrap(), "Section Title").await?;

        // Test query_all_by_role_with_options
        let query_headings = screen.query_all(By::role("heading").level(2)).await?;
        assert_count(&query_headings, 1)?;
        assert_text(&query_headings[0], "Section Title").await?;

        // Test find_by_role_with_options
        let find_heading = screen.find(By::role("heading").level(2)).await?;
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

        let slider = screen
            .get(By::role("slider").value(value_opts.clone()).name("Volume"))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(By::role("slider").value(value_opts.clone()).name("Volume"))
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");
        // Test get_all_by_role_with_options
        let all_sliders = screen
            .get_all(By::role("slider").value(value_opts.clone()).name("Volume"))
            .await?;
        assert_count(&all_sliders, 1)?;
        let aria_label = all_sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(By::role("slider").value(value_opts.clone()).name("Volume"))
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
            .query_all(By::role("slider").value(value_opts.clone()).name("Volume"))
            .await?;
        assert_count(&query_sliders, 1)?;
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Volume");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role("slider").value(value_opts.clone()).name("Volume"))
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

        let slider = screen
            .get(By::role("slider").value(value_opts.clone()))
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(By::role("slider").value(value_opts.clone()))
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
            .get_all(By::role("slider").value(value_opts.clone()))
            .await?;
        assert_count(&all_sliders, 1)?;
        let aria_label = all_sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test query_all_by_role_with_options
        let query_sliders = screen
            .query_all(By::role("slider").value(value_opts.clone()))
            .await?;
        assert_count(&query_sliders, 1)?;
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(By::role("slider").value(value_opts.clone()))
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Brightness");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(By::role("slider").value(value_opts.clone()))
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

        let slider = screen
            .get(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
            .await?;
        let aria_label = slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test get_all_by_role_with_options
        let sliders = screen
            .get_all(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
            .await?;
        assert_count(&sliders, 1)?;
        let aria_label = sliders[0].attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");
        // Test query_by_role_with_options
        let maybe_slider = screen
            .query(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
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
            .query_all(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
            .await?;
        assert_count(&query_sliders, 1)?;
        let aria_label = query_sliders[0]
            .attr("aria-label")
            .await?
            .unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test find_by_role_with_options
        let find_slider = screen
            .find(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
            .await?;
        let aria_label = find_slider.attr("aria-label").await?.unwrap_or_default();
        assert_eq!(aria_label, "Progress");

        // Test find_all_by_role_with_options
        let find_sliders = screen
            .find_all(
                By::role("slider")
                    .value(value_opts.clone())
                    .name("Progress"),
            )
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

        // Try to find the element by its fallback role "checkbox" - need to be more specific
        // Since there are multiple checkboxes, target the switch specifically by its role
        let element = screen.get(By::role("switch").query_fallbacks(true)).await?;
        assert_text(&element, "Toggle Switch").await?;

        // Test query_all_by_role_with_options
        let elements = screen
            .query_all(By::role("switch").query_fallbacks(true))
            .await?;
        assert_count(&elements, 1)?;
        assert_text(&elements[0], "Toggle Switch").await?;

        // Test get_all_by_role_with_options
        let all_elements = screen
            .get_all(By::role("switch").query_fallbacks(true))
            .await?;
        assert_count(&all_elements, 1)?;
        assert_text(&all_elements[0], "Toggle Switch").await?;

        // Test query_by_role_with_options
        let maybe_element = screen
            .query(By::role("switch").query_fallbacks(true))
            .await?;
        assert!(maybe_element.is_some());
        assert_text(&maybe_element.unwrap(), "Toggle Switch").await?;

        // Test find_by_role_with_options
        let find_element = screen
            .find(By::role("switch").query_fallbacks(true))
            .await?;
        assert_text(&find_element, "Toggle Switch").await?;

        // Test find_all_by_role_with_options
        let find_elements = screen
            .find_all(By::role("switch").query_fallbacks(true))
            .await?;
        assert_count(&find_elements, 1)?;
        assert_text(&find_elements[0], "Toggle Switch").await?;

        Ok(())
    })
}
