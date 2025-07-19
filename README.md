# Thirtyfour Testing Library Extension

[![Crates.io](https://img.shields.io/crates/v/thirtyfour-testing-library-ext.svg)](https://crates.io/crates/thirtyfour-testing-library-ext)
[![Documentation](https://docs.rs/thirtyfour-testing-library-ext/badge.svg)](https://docs.rs/thirtyfour-testing-library-ext)

Extends [Thirtyfour](https://github.com/Vrtgs/thirtyfour)'s crate capabilities with the [Javascript Testing Library API](https://testing-library.com/docs) that emphasizes accessibility and user-centric testing approaches.

Find elements using testing library approches and relay on Thirtyfour library to get their properties, click on them, ect.

## About Testing Library

[Testing Library](https://testing-library.com/docs) is a family of packages that help you test UI components in a way that resembles how users interact with your application. The core philosophy is: **"The more your tests resemble the way your software is used, the more confidence they can give you."**

This extension brings Testing Library's semantic query approach to Rust WebDriver testing:

- **Focus on user experience**: Query elements by their role, label, or text content rather than implementation details
- **Accessibility-first**: Prioritize queries that work well with assistive technologies
- **Resilient tests**: Less brittle tests that survive UI refactoring

As The Thirtyfour Testing Library Extension is just a binding to the official Testing Library it is really recommended to rely on their doc to understand how to write your tests: [official documentation](https://testing-library.com/docs).

## Features

- **Semantic selectors**: Query elements by role, text, label, placeholder, and other semantic attributes
- **Multiple query methods**: `get*`, `query*`, and `find*` methods with different behaviors
- **Scoped queries**: Query within specific elements using `within()`
- **Configuration options**: Customize testing library behavior
- **Debugging support**: Testing playground URL generation for debugging

## Getting Started

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
thirtyfour = "0.36.1"
thirtyfour-testing-library-ext = "0.1"
```

> **Note**: This crate is a extension that depends on the published `thirtyfour` crate from crates.io. So the [thirtyfour doc](https://docs.rs/thirtyfour/latest/thirtyfour/) is a good starting point.

## Usage

```rust
use thirtyfour::prelude::*;
use thirtyfour_testing_library_ext::{Screen, By, Configure};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Setup the thirtyfour WebDriver as need, https://docs.rs/thirtyfour/latest/thirtyfour/
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    
    driver.goto("https://example.com").await?;
    
    // Basic usage - create a screen instance
    let screen = Screen::build_with_testing_library(driver.clone()).await?;
    
    // Query by role (semantic selector)
    let button = screen.get(By::role("button")).await?;
    button.click().await?;
    
    // Wait for elements to appear with find()
    let heading = screen.find(By::text("Welcome")).await?;
    println!("Found heading: {}", heading.text().await?);
    
    // Get all matching elements (returns empty vec if none found)
    let all_links = screen.query_all(By::role("link")).await?;
    println!("Found {} links", all_links.len());
    
    // Query with options for more specific matching
    let submit_button = screen.find(By::role("button").name("Submit")).await?;
    let error_messages = screen.find_all(By::title("Error").exact(false)).await?;
    
    // Query within a specific element scope
    let form = screen.get(By::role("form")).await?;
    let form_screen = screen.within(form);
    let input = form_screen.get(By::label_text("Email")).await?;
    input.send_keys("test@example.com").await?;
    
    // Configure testing library behavior
    let config = Configure::new().timeout(5000).test_id_attribute("data-cy");
    let configured_screen = Screen::build_with_testing_library_and_configure(driver.clone(), config).await?;
    
    driver.quit().await?;
    Ok(())
}
```

## Query Methods

The Screen struct provides several query methods with different behaviors:

- `get()` / `get_all()` - Throw errors if elements aren't found
- `query()` / `query_all()` - Return `None` / empty Vec for missing elements  
- `find()` / `find_all()` - Wait for elements to appear with retries

Learn more about Testing Library queries on [the official guide](https://testing-library.com/docs/queries/about)

## Selector Types

- `By::role()` - [ref](https://testing-library.com/docs/queries/byrole)
- `By::text()` - [ref](https://testing-library.com/docs/queries/bytext)
- `By::label_text()` - [ref](https://testing-library.com/docs/queries/bylabeltext)
- `By::placeholder_text()` - [ref](https://testing-library.com/docs/queries/byplaceholdertext)
- `By::alt_text()` - [ref](https://testing-library.com/docs/queries/byalttext)
- `By::title()` - [ref](https://testing-library.com/docs/queries/bytitle)
- `By::test_id()` - [ref](https://testing-library.com/docs/queries/bytestid)
- `By::display_value()` - [ref](https://testing-library.com/docs/queries/bydisplayvalue)

Each selector type supports options for advanced filtering and matching.

## Advanced usage

### Regex

[TextMatch](https://testing-library.com/docs/queries/about#textmatch) from testing library allow regex matching, to use them prefix & suffix by `/`:
```rust
      By::text("/Hello.*/"),                       
      By::text("/hello world/i"),                  
      By::label_text("/username|email/i"),         
      By::placeholder_text("/enter.*/i"),         
      By::alt_text("/profile|avatar/i"),           
      By::title("/click|tap/i"),                  
      By::test_id("/submit|send/"),                
      By::display_value("/[0-9]+%/"),              
      
      By::role("button").name("/submit|send/i"),   
      By::role("textbox").description("/enter.*here/"), 
```

## How It Works

This extension works by injecting the official Testing Library JavaScript code into the browser and bridging it with Thirtyfour's WebDriver capabilities. Here's what happens under the hood:

1. **JavaScript Injection**: When you create a `Screen` instance, the extension injects the Testing Library JavaScript bundle into the current page
2. **Query Translation**: Your Rust `By::role()`, `By::text()`, etc. calls are translated into JavaScript Testing Library queries
3. **Element Resolution**: The JavaScript Testing Library finds elements in the DOM
4. **WebDriver Bridge**: Found elements are returned as standard Thirtyfour `WebElement` objects that you can interact with normally

This approach gives you:
- **Testing Library compatibility**: All the same query logic and options from the JavaScript version
- **Seamless integration**: Results work with all existing Thirtyfour methods (`.click()`, `.send_keys()`, etc.)
- **Robustness**: The Testing Library is battle-tested

This approach is inspired by [Webdriverio Testing Library](https://testing-library.com/docs/webdriverio-testing-library/intro/), [selenium-testing-library](https://medium.com/codex/the-testing-library-meets-selenium-5f74cc712114) (Kotlin) and [selenium-testing-library](https://github.com/anze3db/selenium-testing-library).

## Contributing: 

> You only need to run the tests if you plan on contributing to the development of `thirtyfour-testing-library-ext`.
> If you just want to use the crate in your own project, you can skip this section.

To run the tests, you need to have an instance of `chromedriver` running in the background, perhaps in separate tabs in your terminal.

Download chromedriver: https://chromedriver.chromium.org/downloads

In separate terminal tabs, run the following:

* Tab 1:

      chromedriver

* Tab 2 (navigate to the root of this repository):

      cargo test

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.