# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Thirtyfour is a Selenium WebDriver library for Rust that provides async automation for web browser testing. It supports the W3C WebDriver v1 specification and is tested primarily with Chrome and Firefox.

## Development Commands

### Core Commands
- `cargo build` - Build the project
- `cargo test` - Run tests (requires running WebDriver instances)
- `cargo test -- --test-threads=1` - Run tests with single thread (recommended for WebDriver tests)
- `cargo fmt` - Format code according to project standards
- `cargo clippy` - Run linter and static analysis
- `cargo doc --no-deps --all-features` - Generate documentation
- `cargo run --example <example_name>` - Run specific examples

### Testing Setup
Tests require WebDriver instances running in the background:
- For Chrome: `chromedriver` (default on port 9515)
- For Firefox: `geckodriver` (default on port 4444)
- Use `THIRTYFOUR_BROWSER=firefox cargo test` to test with Firefox instead of Chrome

### Examples
Run examples with: `cargo run --example <name>`
- `tokio_async` - Basic async example
- `tokio_basic` - Simple WebDriver usage
- `selenium_example` - Selenium Grid example
- `minimal_async` - Minimal async setup
- `wikipedia` - Advanced query example
- `chrome_devtools` - Chrome DevTools Protocol usage
- `playground` - Component usage example

## Code Architecture

### Core Components
- **WebDriver** (`web_driver.rs`): Main driver session management
- **WebElement** (`web_element.rs`): Individual element manipulation
- **SessionHandle** (`session/handle.rs`): Low-level session management
- **Query System** (`extensions/query/`): Advanced element querying with polling and filtering
- **Components** (`components/`): Page Object Model-like wrappers for elements
- **Screen** (`screen.rs`): Testing Library integration for DOM queries

### Key Modules
- `session/`: HTTP client, session creation, and management
- `common/`: Shared types, capabilities, commands, and browser-specific configs
- `extensions/`: Browser-specific extensions (Chrome CDP, Firefox tools)
- `action_chain.rs`: Complex user interaction sequences
- `alert.rs`: JavaScript alert handling
- `switch_to.rs`: Context switching (frames, windows, alerts)

### Component System
Components use derive macros to create smart element resolvers:
- `#[derive(Component)]` for component structs
- `#[by(...)]` attributes for element selectors
- `ElementResolver<T>` for lazy element resolution
- Supports nesting and caching for complex web apps

### Query Interface
Advanced querying with `driver.query()` and `element.query()`:
- Supports polling with customizable timeouts
- Multiple selector strategies with `.or()` chaining
- Filtering capabilities with custom predicates
- Explicit wait conditions

## Project Structure

### Workspace Layout
- `thirtyfour/` - Main library crate
- `thirtyfour-macros/` - Derive macros for Component system
- `testing-library/` - JavaScript testing library integration
- `docs/` - mdBook documentation source
- `ci/` - Platform-specific CI scripts

### Feature Flags
- `rustls-tls` (default): Use rustls for TLS
- `native-tls`: Use system TLS
- `component` (default): Enable Component derive macros
- `tokio-multi-threaded`: Enable multi-threaded tokio runtime
- `debug_sync_quit`: Debug synchronous quit behavior

## Testing

### Test Environment
Tests use HTML fixtures in `tests/test_html/` and require WebDriver instances:
- Chrome tests: chromedriver on port 9515
- Firefox tests: geckodriver on port 4444 (set `THIRTYFOUR_BROWSER=firefox`)

### Test Categories
- `elements.rs`: Basic element operations
- `queries.rs`: Advanced query interface
- `components.rs`: Component system
- `screen_by_*.rs`: Testing library integration
- `actions.rs`: Action chains and user interactions
- `alert.rs`: JavaScript alert handling
- `window.rs`: Window and frame management

## Browser Support

### Chrome/Chromium
- Chrome DevTools Protocol (CDP) support in `extensions/cdp/`
- Chrome-specific capabilities in `common/capabilities/chrome.rs`
- Network condition simulation
- Performance metrics collection

### Firefox
- Firefox-specific tools in `extensions/addons/firefox/`
- Preference management
- Firefox-specific capabilities

## Development Notes

### Async Patterns
- Uses tokio for async runtime
- WebDriver sessions must be explicitly closed with `driver.quit()`
- No async destructors - use explicit cleanup to avoid blocking
- Single-threaded test execution recommended due to WebDriver limitations

### Code Style
- Uses rustfmt with max_width=100 and small heuristics disabled
- Comprehensive error handling with `WebDriverResult<T>`
- Extensive use of traits for extensibility
- Builder patterns for complex configurations