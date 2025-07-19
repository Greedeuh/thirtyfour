# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Testing Library integration for the Thirtyfour WebDriver library. It provides DOM queries with semantic selectors similar to React Testing Library, bringing the popular Testing Library philosophy to Rust WebDriver testing.

## Development Commands

### Core Commands
- `cargo build` - Build the project
- `cargo test` - Run tests (requires WebDriver instances)
- `cargo test test_name` - Run specific test
- `cargo fmt` - Format code according to project standards
- `cargo clippy` - Run linter and static analysis

### Testing Requirements
Tests require WebDriver instances running in the background:
- **Chrome**: `chromedriver` on port 9515 (default)

### JavaScript Build Process
- `cd testing-library && npm run predeploy` - Build JavaScript bundles
- `cd testing-library && npm run deploy` - Copy bundles to `/js/`

## Code Architecture

### Core Library Structure
- **`lib.rs`**: Main entry point with `Screen` struct and query methods
- **`configure.rs`**: Configuration options for Testing Library behavior
- **`options/`**: Query option types for different selector strategies
  - `role.rs`: ARIA role-based queries with extensive options
  - `label_text.rs`: Label text queries with selector and exact matching
  - `simple.rs`: Simple queries for text, alt text, display value, etc.
  - `common.rs`: Common traits and utilities

### Query Method Patterns
The library provides three types of query methods with different behaviors:
- **`get()` / `get_all()`**: Throw errors if elements aren't found
- **`query()` / `query_all()`**: Return `None` / empty Vec for missing elements  
- **`find()` / `find_all()`**: Wait for elements to appear with retries

### Selector Types
- **`By::role()`**: ARIA role-based queries with extensive options
- **`By::text()`**: Text content queries
- **`By::label_text()`**: Label text queries
- **`By::placeholder_text()`**: Placeholder text queries
- **`By::alt_text()`**: Alt text queries
- **`By::title()`**: Title attribute queries
- **`By::test_id()`**: Test ID queries
- **`By::display_value()`**: Display value queries

### Key Features
- **Scoped queries**: `within()` method for querying within specific elements
- **Configuration options**: Customizable Testing Library behavior
- **Debugging support**: Testing playground URL generation via `log_testing_playground_url()`
- **Fluent API**: Builder pattern for complex queries with options

## JavaScript Integration

### Architecture
- JavaScript Testing Library is injected into the browser and bridged with Rust WebDriver
- Webpack builds self-contained JavaScript bundles in `/js/`
- Bridge pattern connects Rust queries to JavaScript Testing Library implementation

### Build System
- Webpack configuration in `testing-library/webpack.config.js`
- Source files in `testing-library/src/`
- Output bundles in `/js/` directory

## Testing

### Test Structure
- `tests/common.rs`: Test harness with axum server setup for HTML fixtures
- `tests/test_html/`: HTML fixtures for testing different scenarios
- `tests/screen_by_*.rs`: Individual test files for each query type

### Test Environment
- Uses axum web server to serve HTML fixtures
- Browser-agnostic testing with Chrome and Firefox support
- Single-threaded execution to avoid WebDriver conflicts
- `TestHarness` struct manages WebDriver lifecycle and server setup

### Test Patterns
- Parameterized testing with `rstest`
- Comprehensive coverage of all query methods and options
- Error condition testing alongside success cases

## Development Notes

### Testing Library Philosophy
The library follows Testing Library's core philosophy:
- Query elements by how users interact with them (accessibility-first)
- Prioritize semantic selectors over implementation details
- Write resilient tests that survive UI changes

### Error Handling
- Comprehensive error handling with `WebDriverResult<T>`
- Detailed error messages for failed queries
- Automatic Testing Library injection with retry logic

### Dependencies
- **Core**: `thirtyfour` (WebDriver), `serde` (JSON), `tokio` (async)
- **Dev**: `axum` (test server), `rstest` (parameterized tests), `serial_test` (sequential execution)
- **JavaScript**: `@testing-library/dom`, `webpack` (bundling)

## Platform Support

CI scripts support multiple platforms:
- Linux (Ubuntu) with Chrome/Firefox
- macOS with Chrome/Firefox  
- Windows with Chrome/Firefox