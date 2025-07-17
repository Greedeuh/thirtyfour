# Thirtyfour Testing Library Extension

[![Crates.io](https://img.shields.io/crates/v/thirtyfour-testing-library-ext.svg)](https://crates.io/crates/thirtyfour-testing-library-ext)
[![Documentation](https://docs.rs/thirtyfour-testing-library-ext/badge.svg)](https://docs.rs/thirtyfour-testing-library-ext)

A standalone Testing Library integration for the [Thirtyfour](https://github.com/Vrtgs/thirtyfour) WebDriver library, providing DOM queries with semantic selectors similar to React Testing Library.

This crate extends Thirtyfour's WebDriver capabilities with a Testing Library-inspired API that emphasizes accessibility and user-centric testing approaches.

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

> **Note**: This crate is a standalone extension that depends on the published `thirtyfour` crate from crates.io. You don't need to fork or clone the entire thirtyfour repository to use this extension.

## Usage

```rust
use thirtyfour::prelude::*;
use thirtyfour_testing_library_ext::{Screen, By};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    
    driver.goto("https://example.com").await?;
    
    // Create a screen instance
    let screen = Screen::build_with_testing_library(driver.clone()).await?;
    
    // Query by role (semantic selector)
    let button = screen.get(By::role("button")).await?;
    button.click().await?;
    
    // Query by text content
    let heading = screen.get(By::text("Welcome")).await?;
    println!("Found heading: {}", heading.text().await?);
    
    // Query within a specific element
    let form = screen.get(By::role("form")).await?;
    let form_screen = screen.within(form);
    let input = form_screen.get(By::label_text("Email")).await?;
    input.send_keys("test@example.com").await?;
    
    driver.quit().await?;
    Ok(())
}
```

## Query Methods

The Screen struct provides several query methods with different behaviors:

- `get()` / `get_all()` - Throw errors if elements aren't found
- `query()` / `query_all()` - Return `None` / empty Vec for missing elements  
- `find()` / `find_all()` - Wait for elements to appear with retries

## Selector Types

- `By::role()` - Query by ARIA role
- `By::text()` - Query by text content
- `By::label_text()` - Query by label text
- `By::placeholder_text()` - Query by placeholder text
- `By::alt_text()` - Query by alt text
- `By::title()` - Query by title attribute
- `By::test_id()` - Query by test ID
- `By::display_value()` - Query by display value

Each selector type supports options for advanced filtering and matching.

## Testing

Tests require WebDriver instances running in the background:
- For Chrome: `chromedriver` (default on port 9515)
- For Firefox: `geckodriver` (default on port 4444)

Run tests with:
```bash
cargo test -- --test-threads=1
```

Use `THIRTYFOUR_BROWSER=firefox cargo test` to test with Firefox instead of Chrome.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.