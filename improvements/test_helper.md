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
