use thirtyfour::prelude::*;
use thirtyfour_testing_library_ext::{Screen, By};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Set up WebDriver
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to a test page
    driver.goto("https://example.com").await?;

    // Create a screen instance with testing library capabilities
    let screen = Screen::build_with_testing_library(driver.clone()).await?;

    // Query elements using semantic selectors
    
    // Find by role (accessibility-focused)
    if let Ok(heading) = screen.get(By::role("heading")).await {
        println!("Found heading: {}", heading.text().await?);
    }

    // Find by text content
    if let Some(link) = screen.query(By::text("More information")).await? {
        println!("Found link with text: {}", link.text().await?);
        link.click().await?;
    }

    // Find by test ID (great for test automation)
    if let Ok(button) = screen.get(By::test_id("submit-btn")).await {
        button.click().await?;
    }

    // Scoped queries within a specific element
    if let Ok(form) = screen.get(By::role("form")).await {
        let form_screen = screen.within(form);
        
        // Find input within the form
        if let Ok(input) = form_screen.get(By::label_text("Email")).await {
            input.send_keys("test@example.com").await?;
        }
    }

    // Use configuration for custom behavior
    let configured_screen = screen.configure(
        thirtyfour_testing_library_ext::configure::Options::new()
            .with_test_id_attribute("data-cy")  // Use Cypress convention
            .with_default_hidden(false)         // Include hidden elements by default
    );

    // Clean up
    driver.quit().await?;

    Ok(())
}