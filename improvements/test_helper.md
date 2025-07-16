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
// mentor review => can it be merged with TestHarness ?
