# 13. Test Organization

## Problem
21 test files with repetitive patterns. Could benefit from parameterized tests to reduce duplication, integration test helpers, and property-based testing for edge cases.

## Current Issues

### Repetitive test patterns
```rust
// Similar patterns across multiple files
#[rstest]
#[tokio::test]
async fn get_by_role_button() {
    let (driver, screen) = get_driver_and_screen("sample_page.html").await;
    let button = screen.get(By::role("button")).await.unwrap();
    assert_eq!(button.id().await.unwrap().unwrap(), "button-id");
    quit_driver(driver).await;
}

#[rstest]
#[tokio::test]  
async fn query_by_role_button() {
    let (driver, screen) = get_driver_and_screen("sample_page.html").await;
    let button = screen.query(By::role("button")).await.unwrap().unwrap();
    assert_eq!(button.id().await.unwrap().unwrap(), "button-id");
    quit_driver(driver).await;
}
```

## Solutions

### Parameterized tests to reduce duplication
```rust
// tests/query_methods.rs
use rstest::*;

#[derive(Debug)]
enum QueryMethod {
    Get,
    Query, 
    Find,
}

#[derive(Debug)]
struct TestCase {
    method: QueryMethod,
    selector: By,
    html_file: &'static str,
    expected_id: &'static str,
    should_succeed: bool,
}

#[rstest]
#[case::get_role_button(TestCase {
    method: QueryMethod::Get,
    selector: By::role("button"),
    html_file: "sample_page.html",
    expected_id: "button-id",
    should_succeed: true,
})]
#[case::query_role_button(TestCase {
    method: QueryMethod::Query,
    selector: By::role("button"), 
    html_file: "sample_page.html",
    expected_id: "button-id",
    should_succeed: true,
})]
#[case::get_nonexistent(TestCase {
    method: QueryMethod::Get,
    selector: By::role("nonexistent"),
    html_file: "sample_page.html", 
    expected_id: "",
    should_succeed: false,
})]
#[tokio::test]
async fn test_query_method(#[case] test_case: TestCase) {
    let (driver, screen) = get_driver_and_screen(test_case.html_file).await;
    
    let result = match test_case.method {
        QueryMethod::Get => screen.get(test_case.selector).await.map(Some),
        QueryMethod::Query => screen.query(test_case.selector).await,
        QueryMethod::Find => screen.find(test_case.selector).await.map(Some),
    };
    
    match (result, test_case.should_succeed) {
        (Ok(Some(element)), true) => {
            let id = element.id().await.unwrap().unwrap();
            assert_eq!(id, test_case.expected_id);
        }
        (Ok(None), false) => {
            // Expected when using query() on non-existent elements
        }
        (Err(_), false) => {
            // Expected when using get() or find() on non-existent elements  
        }
        _ => panic!("Unexpected result for test case: {:?}", test_case),
    }
    
    quit_driver(driver).await;
}
```

### Integration test helpers
```rust
// tests/helpers/mod.rs
use thirtyfour::prelude::*;
use thirtyfour_testing_library_ext::{Screen, By};

pub struct TestContext {
    pub driver: WebDriver,
    pub screen: Screen,
}

impl TestContext {
    pub async fn new(html_file: &str) -> Self {
        let (driver, screen) = get_driver_and_screen(html_file).await;
        Self { driver, screen }
    }
    
    pub async fn assert_element_exists(&self, selector: By, expected_id: &str) {
        let element = self.screen.get(selector).await
            .expect("Element should exist");
        let id = element.id().await.unwrap()
            .expect("Element should have an ID");
        assert_eq!(id, expected_id);
    }
    
    pub async fn assert_element_not_found(&self, selector: By) {
        let result = self.screen.query(selector).await.unwrap();
        assert!(result.is_none(), "Element should not exist");
    }
    
    pub async fn assert_elements_count(&self, selector: By, expected_count: usize) {
        let elements = self.screen.query_all(selector).await.unwrap();
        assert_eq!(elements.len(), expected_count);
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Ensure cleanup even if test panics
        tokio::spawn({
            let driver = self.driver.clone();
            async move {
                let _ = driver.quit().await;
            }
        });
    }
}

// Fluent assertion builder
pub struct ElementAssertion<'a> {
    context: &'a TestContext,
    selector: By,
}

impl<'a> ElementAssertion<'a> {
    pub fn new(context: &'a TestContext, selector: By) -> Self {
        Self { context, selector }
    }
    
    pub async fn exists(self) -> ElementAssertion<'a> {
        self.context.screen.get(self.selector.clone()).await
            .expect("Element should exist");
        self
    }
    
    pub async fn not_exists(self) -> ElementAssertion<'a> {
        let result = self.context.screen.query(self.selector.clone()).await.unwrap();
        assert!(result.is_none(), "Element should not exist");
        self
    }
    
    pub async fn has_id(self, expected_id: &str) -> ElementAssertion<'a> {
        let element = self.context.screen.get(self.selector.clone()).await
            .expect("Element should exist");
        let id = element.id().await.unwrap()
            .expect("Element should have an ID");
        assert_eq!(id, expected_id);
        self
    }
    
    pub async fn has_text(self, expected_text: &str) -> ElementAssertion<'a> {
        let element = self.context.screen.get(self.selector.clone()).await
            .expect("Element should exist");
        let text = element.text().await.unwrap();
        assert_eq!(text, expected_text);
        self
    }
}

impl TestContext {
    pub fn assert_that(&self, selector: By) -> ElementAssertion {
        ElementAssertion::new(self, selector)
    }
}
```

### Property-based testing for edge cases
```rust
// tests/property_tests.rs
use proptest::prelude::*;
use thirtyfour_testing_library_ext::{By, ByRoleOptions};

prop_compose! {
    fn valid_role_name()(name in "[a-zA-Z][a-zA-Z0-9-_]*") -> String {
        name
    }
}

prop_compose! {
    fn valid_text_content()(text in ".*") -> String {
        text
    }
}

proptest! {
    #[test]
    fn role_selector_creation_never_panics(
        name in valid_role_name(),
        exact in any::<bool>(),
        hidden in any::<bool>()
    ) {
        let selector = By::role_with_options(
            name.clone(),
            ByRoleOptions::new().exact(exact).hidden(hidden)
        );
        
        // Selector should be creatable without panicking
        assert_eq!(selector.value(), &name);
    }
    
    #[test] 
    fn text_selector_handles_any_text(text in valid_text_content()) {
        let selector = By::text(text.clone());
        assert_eq!(selector.value(), &text);
        
        // Should be able to serialize options without panicking
        let _ = selector.options_json();
    }
}

// Regression tests for specific edge cases
#[tokio::test]
async fn empty_string_selectors_handled_gracefully() {
    let context = TestContext::new("sample_page.html").await;
    
    // These should not panic, but should return appropriate errors
    let result = context.screen.query(By::role("")).await;
    assert!(result.is_err() || result.unwrap().is_none());
    
    let result = context.screen.query(By::text("")).await;
    assert!(result.is_err() || result.unwrap().is_none());
}

#[tokio::test]
async fn unicode_text_content_handling() {
    let context = TestContext::new("unicode_page.html").await;
    
    // Test various unicode scenarios
    let test_cases = [
        "Hello 世界",
        "🚀 Rocket",
        "Ñiño",
        "Москва",
        "🎉🎊🎈",
    ];
    
    for &text in &test_cases {
        // Should handle unicode gracefully
        let result = context.screen.query(By::text(text)).await;
        assert!(result.is_ok());
    }
}
```

### Organized test modules
```rust
// tests/mod.rs
mod helpers;

mod unit {
    mod selector_creation;
    mod option_serialization;
    mod query_building;
}

mod integration {
    mod basic_queries;
    mod advanced_queries;
    mod error_scenarios;
    mod performance;
}

mod browser_specific {
    mod chrome_tests;
    mod firefox_tests;
}

mod regression {
    mod issue_123_unicode_handling;
    mod issue_456_empty_selectors;
}
```

### Snapshot testing for complex scenarios
```rust
// tests/snapshot_tests.rs
use insta::assert_debug_snapshot;

#[tokio::test]
async fn complex_query_serialization_snapshot() {
    let complex_selector = By::role_with_options(
        "button",
        ByRoleOptions::new()
            .exact(true)
            .hidden(false)
            .level(Some(2))
            .name("Submit Order")
    );
    
    let json = complex_selector.options_json().unwrap();
    assert_debug_snapshot!(json);
}
```

## Benefits
- Reduced code duplication
- Better test coverage
- Easier maintenance
- More comprehensive edge case testing
- Cleaner test organization
- Fluent assertion API

## Impact
- **Maintainability**: High (much easier to maintain tests)
- **Coverage**: High (better edge case coverage)  
- **Complexity**: Medium (requires test infrastructure setup)
- **Breaking**: None (internal test improvements)