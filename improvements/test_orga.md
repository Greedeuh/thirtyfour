

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
